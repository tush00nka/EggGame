use bevy::prelude::*;

use crate::{score::Score, GameState};

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), spawn_screen)
            .add_systems(Update, restart
                .run_if(in_state(GameState::GameOver)));
    }
}

fn spawn_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Game Over))\nBest score: {}\nPress LMB to try again))", score.get_best()),
            TextStyle { 
                font: asset_server.load("fonts/cartoon.ttf"),
                font_size: 48.,
                color: Color::BLACK
            }).with_text_justify(JustifyText::Center)
        );
    })
    .insert(StateScoped(GameState::GameOver));
}

fn restart(
    mut next_state: ResMut<NextState<GameState>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    if mouse.just_released(MouseButton::Left)
    || touches.any_just_released() {
        next_state.set(GameState::Playing);
    }
}