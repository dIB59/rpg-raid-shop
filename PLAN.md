# Plan: Enemies & Sprites

Goal: replace the colored-rectangle player with proper animated 2D pixel art, then add animated enemies with basic chase
AI.

## 1. Get an asset pack (the "knight one")

Free itch.io packs, all in the classic pixel-art style:

- **Tiny Swords** by Pixel Frog — most popular; warriors/knights, goblins, buildings, terrain. Bright, chunky, cohesive.
  Best if you want a whole game's worth of art.
- **Hero Knight** by LuizMelo — single detailed animated knight (idle/run/attack/roll — pairs well with the existing
  dodge).
- **Fantasy Knight** by aamatniekss — moodier, darker knight with a full animation set.

For enemies: Tiny Swords has goblins; LuizMelo's free "Evil Wizard" and "Bringer of Death" pair well with Hero Knight.

Download, unzip, and put the PNGs in `assets/` at the project root — Bevy's `AssetServer` loads from there
automatically.

## 2. Make pixel art render crisp (one line)

In `main.rs`:

```rust
app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
```

Without this, sprites get bilinear-filtered and look blurry when scaled up.

## 3. Sprite sheets → animated sprites

Packs ship animations as sprite sheets (one PNG, frames in a grid). In Bevy 0.19, slice them with `TextureAtlasLayout`:

```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("knight/idle.png");
    // e.g. 8 frames of 120x80 in one row — check the pack's readme
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(120, 80), 8, 1, None, None,
    ));
    commands.spawn((
        Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 }),
        Transform::from_scale(Vec3::splat(2.0)),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
```

One animation system drives every animated sprite:

```rust
#[derive(Component)]
struct AnimationTimer(Timer);

fn animate(time: Res<Time>, mut q: Query<(&mut AnimationTimer, &mut Sprite)>) {
    for (mut timer, mut sprite) in &mut q {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % 8; // frame count
            }
        }
    }
}
```

Later: an `AnimationState` enum (Idle/Run/Dodge/Attack) that swaps which texture + frame range plays, driven by the
existing movement/dodge state. Flip facing with `sprite.flip_x = velocity.x < 0.0`.

## Gotcha

Frame sizes and counts differ per animation file in these packs — check each PNG's dimensions and divide by frame count
rather than assuming one grid size fits all. For Tiny Swords' Warrior, every animation uses **192×192** frames; only the
column count varies (Idle 8, Run 6, Guard 6, Attack1/2 4).

---

# Plan: Enemies & Factions

Status: steps 1–3 above are done. The player renders from the Blue Units warrior sheet, and `camera.rs` holds a
`GameCameraPlugin` that follows anything tagged `CameraTarget`.

The asset pack turned out to be the newer Tiny Swords with colour variants rather than goblins —
`Units/Red Units/Warrior/` is the drop-in enemy, identical layout to the blue one.

## 1. `src/faction.rs`

```rust
use bevy::prelude::Component;

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Faction {
    Player,
    Hostile,
}

impl Faction {
    /// The single place the hostility rule lives.
    pub fn is_hostile_to(self, other: Faction) -> bool {
        self != other
    }
}
```

That method is the point of the whole module. Every AI, targeting, and damage check calls it instead of comparing
variants directly, so adding free-for-all later is one function body — a `Faction::Anarchy` variant that returns `true`
unconditionally, or a `Berserk` marker the function consults. Scatter `a != b` across five systems and you'll be hunting
them all down.

`Copy` matters: factions get passed by value constantly, and without it every comparison needs a borrow dance.

## 2. `src/movement.rs`

```rust
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);
```

Moved out of `player.rs` (currently private there) so enemies can use it without importing from `player`. The field
becomes `pub` because `move_player` now writes it from another module.

## 3. `src/enemy.rs`

```rust
#[derive(Component)]
pub struct Enemy {
    pub speed: f32,        // ~250.0 — noticeably slower than the player's 600
    pub aggro_radius: f32, // ~600.0
}
```

No `ChaseTarget` marker — `Faction` covers it. (`CameraTarget` stays; the camera genuinely doesn't care what it watches,
so a bare marker is still right there.)

**Spawn:**

```rust
fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Tiny Swords/Units/Red Units/Warrior/Warrior_Idle.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(192, 192), 8, 1, None, None,
    ));

    for (x, y) in [(400.0, 300.0), (-500.0, 150.0), (250.0, -450.0)] {
        commands.spawn((
            Enemy { speed: 250.0, aggro_radius: 600.0 },
            Faction::Hostile,
            Velocity::default(),
            Sprite::from_atlas_image(texture.clone(), TextureAtlas::from(layout.clone())),
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}
```

`texture.clone()` and `layout.clone()` clone *handles* — refcounted pointers into the asset store, not image data. All
three enemies share one GPU texture.

**Chase — decides only, never touches `Transform`:**

```rust
fn chase(
    mut movers: Query<(Entity, &Transform, &Faction, &mut Velocity, &Enemy)>,
    candidates: Query<(Entity, &Transform, &Faction)>,
) {
    for (me, tf, my_faction, mut vel, enemy) in &mut movers {
        let pos = tf.translation.truncate();

        let nearest = candidates
            .iter()
            .filter(|(e, _, f)| *e != me && my_faction.is_hostile_to(**f))
            .map(|(_, t, _)| t.translation.truncate())
            .min_by(|a, b| a.distance_squared(pos).total_cmp(&b.distance_squared(pos)));

        let Some(target) = nearest else {
            vel.0 = Vec2::ZERO;
            continue;
        };

        let to_target = target - pos;
        vel.0 = if to_target.length() > enemy.aggro_radius {
            Vec2::ZERO
        } else {
            to_target.normalize_or_zero() * enemy.speed
        };
    }
}
```

Why it's split this way: both queries take `Transform` and `Faction` **read-only**, and only `Velocity` is mutable, in
only one of them. Read/read never conflicts, so no `Without` filter is needed and the two entity sets may overlap
freely — which is precisely what lets an enemy also be someone else's target. Had `chase` written `Transform` directly,
Bevy would panic with `error[B0001]` at startup and the only fixes would be a disjointness filter (a lie, once enemies
target each other) or a `ParamSet`.

General rule that falls out: when two systems fight over a component, usually one of them shouldn't be writing it.

Two smaller details:

- `distance_squared`, not `distance` — you're only ranking, so the square root is wasted work in a loop over every
  enemy × every candidate.
- `total_cmp` rather than `partial_cmp().unwrap()` — the latter panics on `NaN`; `total_cmp` gives a total order and
  can't.
- `normalize_or_zero` rather than `normalize` — `normalize` on a zero vector yields `NaN`, the transform becomes `NaN`,
  and the sprite vanishes permanently with no error.

**Apply:**

```rust
fn apply_velocity(
    mut q: Query<(&mut Transform, &Velocity), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut tf, vel) in &mut q {
        tf.translation += (vel.0 * time.delta_secs()).extend(0.0);
    }
}
```

`With<Enemy>` is a deliberate stopgap. `move_player` still integrates its own velocity, and `Dodge` writes `Transform`
directly with an early return — drop the filter and the player moves at double speed. Unifying that is a real refactor
of the dodge path; do it after enemies are on screen, not during.

**Plugin:**

```rust
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (chase, apply_velocity).chain());
    }
}
```

`.chain()` because `apply_velocity` consumes what `chase` produces. No ordering against `move_player` — unlike the
camera, a chase target that's one frame stale is off by ~10 units and invisible. Don't add ordering constraints
reflexively; each one costs parallelism.

## 4. Wiring

- `player.rs` — add `Faction::Player` to the spawn tuple, import `Velocity` from `crate::movement` instead of declaring
  it, keep `CameraTarget`.
- `main.rs` — `mod faction; mod movement; mod enemy;` and `.add_plugins(EnemyPlugin)`.

## Suggested order

1. Spawn the three enemies with no AI. Confirm three red warriors appear when you walk to them.
2. Add `chase` + `apply_velocity` with `aggro_radius` set huge so they always chase — easiest thing to confirm visually.
3. Tune the radius down and check they go passive at distance.

## Gotchas

- **They'll clump into one sprite.** No separation force, so all three converge on the identical point. Expected, not a
  bug in the chase code — repulsion between enemies is the next step.
- **They'll walk straight through the player.** Add a stopping distance (
  `if to_target.length() < 60.0 { vel.0 = Vec2::ZERO; continue }`) as the cheap fix that reads correctly before real
  collision exists.
- **Draw order is arbitrary at equal z.** Everything sits at `z = 0.0`, so an overlapping enemy and player can flicker
  over each other between frames. Give the player `z = 1.0` for now; y-sorting (`z = -y * small_factor`, so
  lower-on-screen draws in front) is the real fix once there are more sprites.
- **Run with `cargo run`, not `./target/debug/rpg-raid-shop`.** Bevy resolves the asset root relative to the manifest
  under cargo and relative to the executable otherwise — the bare binary looks in `target/debug/assets/` and silently
  renders nothing.

## Later

- Free-for-all: extend `Faction::is_hostile_to` rather than touching any AI code.
- `Health` + damage: needs the faction check too, so it goes through the same function.
- Separation force between enemies; then real collision.
- Animation state (Idle/Run) driven off `Velocity`, shared by player and enemies — `Warrior_Run.png` is 6 columns of the
  same 192×192 grid.
