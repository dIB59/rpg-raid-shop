use std::iter::FusedIterator;
use std::time::Duration;

use bevy::app::{App, Plugin, Update};
use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlas};
use bevy::math::UVec2;
use bevy::prelude::{Assets, Bundle, Component, Query, Res, TextureAtlasLayout};
use bevy::sprite::Sprite;
use bevy::time::{Time, Timer, TimerMode};

#[derive(Component, Clone)]
pub struct AnimationClip {
    first: usize,
    last: usize,
    next: Option<usize>,
}

impl AnimationClip {
    pub fn new(first: usize, last: usize) -> Self {
        Self {
            first,
            last,
            next: Some(first),
        }
    }

    pub fn reset(&mut self) {
        self.next = Some(self.first);
    }

    pub fn is_finished(&self) -> bool {
        self.next.is_none()
    }
}

impl Iterator for AnimationClip {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let current = self.next?;
        self.next = (current < self.last).then(|| current + 1);
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = match self.next {
            Some(next) => self.last.saturating_sub(next) + 1,
            None => 0,
        };
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AnimationClip {}
impl FusedIterator for AnimationClip {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    Once,
    Repeating,
}

/// Everything needed to drive one sprite's frames: the clip, its playback mode,
/// and the frame timer.
#[derive(Component, Clone)]
pub struct Animation {
    clip: AnimationClip,
    mode: AnimationMode,
    timer: Timer,
    frame: usize,
    /// One frame *after* a [`AnimationMode::Once`] clip's last frame, so frame gets its full time on screen.
    done: bool,
}

impl Animation {
    /// `last` is inclusive.
    pub fn new(first: usize, last: usize, secs_per_frame: f32, mode: AnimationMode) -> Self {
        let mut clip = AnimationClip::new(first, last);
        // Consume the first frame up front: it's what the sprite spawns showing,
        // so `tick` should hand back the *second* frame the first time it fires.
        let frame = clip.next().unwrap_or(first);
        Self {
            clip,
            mode,
            timer: Timer::from_seconds(secs_per_frame, TimerMode::Repeating),
            frame,
            done: false,
        }
    }

    pub fn repeating(first: usize, last: usize, secs_per_frame: f32) -> Self {
        Self::new(first, last, secs_per_frame, AnimationMode::Repeating)
    }

    pub fn once(first: usize, last: usize, secs_per_frame: f32) -> Self {
        Self::new(first, last, secs_per_frame, AnimationMode::Once)
    }

    /// Builds a layout for a single-row sprite strip, sizes the clip to it
    /// Modelled after [`TextureAtlasLayout::from_grid`]
    pub fn from_grid(
        layouts: &mut Assets<TextureAtlasLayout>,
        image: Handle<Image>,
        frame_size: UVec2,
        columns: u32,
        secs_per_frame: f32,
        mode: AnimationMode,
    ) -> impl Bundle {
        let layout = layouts.add(TextureAtlasLayout::from_grid(
            frame_size, columns, 1, None, None,
        ));
        let last = columns.saturating_sub(1) as usize;
        Self::new(0, last, secs_per_frame, mode).with_sprite(image, layout)
    }

    /// The frame currently on screen.
    pub fn frame(&self) -> usize {
        self.frame
    }

    /// Bundles this animation with the sprite it drives, already showing
    /// [`Self::frame`]. The handles aren't retained: once spawned, the `Sprite`
    /// is the only owner of the atlas.
    pub fn with_sprite(
        self,
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    ) -> impl Bundle {
        let atlas = TextureAtlas {
            layout,
            index: self.frame,
        };
        (Sprite::from_atlas_image(image, atlas), self)
    }

    /// `true` once a [`AnimationMode::Once`] animation has shown its last frame
    /// for its full duration. Always `false` while repeating.
    pub fn is_finished(&self) -> bool {
        self.done
    }

    /// Advances playback, returning the new frame only on the ticks where it changed.
    pub fn tick(&mut self, delta: Duration) -> Option<usize> {
        self.timer.tick(delta);
        if !self.timer.just_finished() {
            return None;
        }

        if self.clip.is_finished() {
            match self.mode {
                AnimationMode::Repeating => self.clip.reset(),
                AnimationMode::Once => {
                    self.done = true;
                    return None;
                }
            }
        }

        self.frame = self.clip.next().expect("clip refilled above");
        Some(self.frame)
    }
}

fn animate(time: Res<Time>, mut q: Query<(&mut Animation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut q {
        if let (Some(frame), Some(atlas)) =
            (animation.tick(time.delta()), &mut sprite.texture_atlas)
        {
            atlas.index = frame;
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}
