//! Player-specific behavior.

use bevy::prelude::*;
use bevy_gauge::prelude::Attributes;
use leafwing_input_manager::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    assets::CharacterAssets,
    components::{Health, Movement, Player, Weapon},
    game::attributes::player_attributes,
    systems::PlayerAnimation,
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
}

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            record_player_input
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
        );
    }
}

/// The player character.
pub fn player(player_assets: &CharacterAssets, weapon: String) -> impl Bundle {
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        Weapon::new(weapon),
        Attributes::new(),
        player_attributes(),
        Health::default(),
        Sprite::from_atlas_image(
            player_assets.sprite.clone(),
            TextureAtlas {
                layout: player_assets.layout.clone(),
                index: player_animation.get_atlas_index(),
            },
        ),
        Transform::from_scale(Vec2::splat(3.0).extend(1.0)),
        Movement::default(),
        player_animation,
        PlayerAction::default_input_map(),
    )
}

impl PlayerAction {
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

        input_map
    }
}

fn record_player_input(
    action_state: Single<&ActionState<PlayerAction>>,
    mut controller_query: Query<&mut Movement, With<Player>>,
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
