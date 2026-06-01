//! The between-wave monster buff choice menu.

use bevy::prelude::*;

use crate::{
    Pause,
    menus::Menu,
    systems::{MONSTER_BUFF_CHOICES, MonsterBuff, MonsterProgression},
    theme::widget,
};

pub(super) struct MonsterBuffMenuPlugin;

impl Plugin for MonsterBuffMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::MonsterBuff), spawn_monster_buff_menu);
    }
}

fn spawn_monster_buff_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Monster Buff Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::MonsterBuff),
        children![
            widget::header("Wave complete"),
            widget::label("Choose how the monsters evolve."),
            widget::button(MONSTER_BUFF_CHOICES[0].title, choose_thick_blood),
            widget::label(buff_description(MONSTER_BUFF_CHOICES[0])),
            widget::button(MONSTER_BUFF_CHOICES[1].title, choose_frenzy),
            widget::label(buff_description(MONSTER_BUFF_CHOICES[1])),
            widget::button(MONSTER_BUFF_CHOICES[2].title, choose_cruel_claws),
            widget::label(buff_description(MONSTER_BUFF_CHOICES[2])),
        ],
    ));
}

fn buff_description(buff: MonsterBuff) -> String {
    format!("{} | {}", buff.danger, buff.reward)
}

fn choose_thick_blood(
    _: On<Pointer<Click>>,
    progression: ResMut<MonsterProgression>,
    next_menu: ResMut<NextState<Menu>>,
    next_pause: ResMut<NextState<Pause>>,
) {
    choose_buff(MONSTER_BUFF_CHOICES[0], progression, next_menu, next_pause);
}

fn choose_frenzy(
    _: On<Pointer<Click>>,
    progression: ResMut<MonsterProgression>,
    next_menu: ResMut<NextState<Menu>>,
    next_pause: ResMut<NextState<Pause>>,
) {
    choose_buff(MONSTER_BUFF_CHOICES[1], progression, next_menu, next_pause);
}

fn choose_cruel_claws(
    _: On<Pointer<Click>>,
    progression: ResMut<MonsterProgression>,
    next_menu: ResMut<NextState<Menu>>,
    next_pause: ResMut<NextState<Pause>>,
) {
    choose_buff(MONSTER_BUFF_CHOICES[2], progression, next_menu, next_pause);
}

fn choose_buff(
    buff: MonsterBuff,
    mut progression: ResMut<MonsterProgression>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_pause: ResMut<NextState<Pause>>,
) {
    progression.apply(buff);
    next_menu.set(Menu::None);
    next_pause.set(Pause(false));
}
