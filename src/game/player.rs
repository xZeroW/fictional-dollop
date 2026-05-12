//! Player-specific behavior.

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    assets::CharacterAssets,
    game::{animation::PlayerAnimation, movement::MovementController},
};

use crate::components::{c_movement::Movement, health::Health};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Attack,
    SwitchWeapon,
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
pub fn player(player_assets: &CharacterAssets, weapon: String) -> impl Bundle {
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player {
            weapon,
            ..default()
        },
        Sprite::from_atlas_image(
            player_assets.sprite.clone(),
            TextureAtlas {
                layout: player_assets.layout.clone(),
                index: player_animation.get_atlas_index(),
            },
        ),
        Transform::from_scale(Vec2::splat(3.0).extend(1.0)),
        MovementController::default(),
        player_animation,
        Player::default_input_map(),
    )
}

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
#[require(Health, Movement)]
pub struct Player {
    pub weapon: String,
    pub weapon_entity: Option<Entity>,
    pub last_shot_time: f32,
    pub switching_weapon: bool,
    pub switch_timer: Timer,
    pub can_shoot_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            weapon: "dagger".to_string(),
            weapon_entity: None,
            last_shot_time: 0.0,
            switching_weapon: false,
            switch_timer: Timer::from_seconds(3.0, TimerMode::Once),
            can_shoot_timer: Timer::from_seconds(0.2, TimerMode::Once),
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
        input_map.insert(SwitchWeapon, KeyCode::KeyQ);

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
