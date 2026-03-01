//! Player-specific behavior.

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
    },
    ron_asset::CharacterAssets,
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Attack,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        record_player_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(max_speed: f32, player_assets: &CharacterAssets, weapon: String) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player { weapon },
        Sprite::from_atlas_image(
            player_assets.sprite.clone(),
            TextureAtlas {
                layout: player_assets.layout.clone(),
                index: player_animation.get_atlas_index(),
            },
        ),
        Transform::from_scale(Vec2::splat(3.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
        player_animation,
        Player::default_input_map(),
    )
}

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub weapon: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            weapon: "dagger".to_string(),
        }
    }
}

impl Player {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, KeyCode::ArrowUp);

        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, KeyCode::ArrowDown);

        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, KeyCode::ArrowLeft);

        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, KeyCode::ArrowRight);

        input_map.insert(Attack, MouseButton::Left);

        input_map
    }
}

fn record_player_input(
    action_state: Single<&ActionState<PlayerAction>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec2::ZERO;
    if action_state.pressed(&PlayerAction::Up) {
        intent.y += 1.0;
    }
    if action_state.pressed(&PlayerAction::Down) {
        intent.y -= 1.0;
    }
    if action_state.pressed(&PlayerAction::Left) {
        intent.x -= 1.0;
    }
    if action_state.pressed(&PlayerAction::Right) {
        intent.x += 1.0;
    }

    let intent = intent.normalize_or_zero();

    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}
