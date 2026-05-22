use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct HitFlash {
    timer: Timer,
    original_color: Color,
}

impl HitFlash {
    const FLASH_DURATION: f32 = 0.1;
    const FLASH_INTERVAL: f32 = 0.05;
    pub const FLASH_COLOR: Color = Color::WHITE;

    pub fn new(original_color: Color) -> Self {
        Self {
            timer: Timer::from_seconds(Self::FLASH_DURATION, TimerMode::Once),
            original_color,
        }
    }

    pub fn restart(&mut self) {
        self.timer.reset();
    }
}

pub fn tick_hit_flash(time: Res<Time>, mut query: Query<&mut HitFlash>) {
    for mut flash in &mut query {
        flash.timer.tick(time.delta());
    }
}

pub fn update_hit_flash(
    mut commands: Commands,
    mut query: Query<(Entity, &HitFlash, &mut Sprite)>,
) {
    for (entity, flash, mut sprite) in &mut query {
        if flash.timer.is_finished() {
            sprite.color = flash.original_color;
            commands.entity(entity).try_remove::<HitFlash>();
            continue;
        }

        let phase = (flash.timer.elapsed_secs() / HitFlash::FLASH_INTERVAL) as usize;
        sprite.color = if phase % 2 == 0 {
            HitFlash::FLASH_COLOR
        } else {
            let mut transparent_white = HitFlash::FLASH_COLOR;
            transparent_white.set_alpha(0.0);
            transparent_white
        };
    }
}
