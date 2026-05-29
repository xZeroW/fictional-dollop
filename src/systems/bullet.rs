use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, game::weapon::Bullet, screens::Screen};

pub(super) struct BulletSystemsPlugin;

impl Plugin for BulletSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_bullet, despawn_bullet)
                .in_set(AppSystems::Update)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn move_bullet(mut bullet_query: Query<(&mut Transform, &mut Bullet)>, time: Res<Time>) {
    for (mut transform, mut bullet) in &mut bullet_query {
        bullet.lifetime.tick(time.delta());
        let velocity = bullet.velocity * time.delta_secs();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn despawn_bullet(mut commands: Commands, bullet_query: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in bullet_query.iter() {
        if bullet.lifetime.elapsed() >= bullet.lifetime.duration() {
            commands.entity(entity).despawn();
        }
    }
}
