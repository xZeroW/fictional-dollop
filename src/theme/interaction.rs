use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{audio::sound_effect, screens::Screen};

#[derive(AssetCollection, Resource)]
struct InteractionAssets {
    #[asset(key = "menu.hover")]
    hover: Handle<AudioSource>,
    #[asset(key = "menu.click")]
    click: Handle<AudioSource>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Splash)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "data/interaction.assets.ron",
            )
            .load_collection::<InteractionAssets>(),
    );

    app.add_observer(apply_interaction_palette_on_click);
    app.add_observer(apply_interaction_palette_on_over);
    app.add_observer(apply_interaction_palette_on_out);

    app.add_observer(play_sound_effect_on_click);
    app.add_observer(play_sound_effect_on_over);
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette_on_click(
    click: On<Pointer<Click>>,
    mut palette_query: Query<(&InteractionPalette, &mut BackgroundColor)>,
) {
    let Ok((palette, mut bg)) = palette_query.get_mut(click.event_target()) else {
        return;
    };

    *bg = palette.pressed.into();
}

fn apply_interaction_palette_on_over(
    over: On<Pointer<Over>>,
    mut palette_query: Query<(&InteractionPalette, &mut BackgroundColor)>,
) {
    let Ok((palette, mut bg)) = palette_query.get_mut(over.event_target()) else {
        return;
    };

    *bg = palette.hovered.into();
}

fn apply_interaction_palette_on_out(
    out: On<Pointer<Out>>,
    mut palette_query: Query<(&InteractionPalette, &mut BackgroundColor)>,
) {
    let Ok((palette, mut bg)) = palette_query.get_mut(out.event_target()) else {
        return;
    };

    *bg = palette.none.into();
}

fn play_sound_effect_on_click(
    click: On<Pointer<Click>>,
    interaction_assets: If<Res<InteractionAssets>>,
    mut commands: Commands,
    palette_query: Query<&InteractionPalette>,
) {
    if palette_query.get(click.event_target()).is_ok() {
        commands.spawn(sound_effect(interaction_assets.click.clone()));
    }
}

fn play_sound_effect_on_over(
    over: On<Pointer<Over>>,
    interaction_assets: If<Res<InteractionAssets>>,
    mut commands: Commands,
    palette_query: Query<&InteractionPalette>,
) {
    if palette_query.get(over.event_target()).is_ok() {
        commands.spawn(sound_effect(interaction_assets.hover.clone()));
    }
}
