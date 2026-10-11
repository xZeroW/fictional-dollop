use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    messages::{ApplyDamageMessage, BulletHitEnemyMessage},
    screens::Screen,
};

pub struct BulletCollisionListenerPlugin;

impl Plugin for BulletCollisionListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_bullet_enemy_collision
                .in_set(AppSystems::CollisionEvents)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn handle_bullet_enemy_collision(
    mut hit_reader: MessageReader<BulletHitEnemyMessage>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    for hit in hit_reader.read() {
        damage_writer.write(ApplyDamageMessage {
            target: hit.enemy,
            damage: hit.damage,
        });
    }
}
