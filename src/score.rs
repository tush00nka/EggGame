use bevy::prelude::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), spawn_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Resource, Default)]
pub struct Score {
    current: u32,
    best: u32,
}

impl Score {
    /// Checks whetehr the current score is higher than best,
    /// if so, rewrites the best score with current.
    /// Returns best score.
    pub fn get_best(&mut self) -> u32 {
        if self.current > self.best {
            self.best = self.current;
        }

        self.best
    } 

    /// Simply adds 1 to the current score
    pub fn add(&mut self) {
        self.current += 1;
    }
}

#[derive(Component)]
struct ScoreText;

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    score.current = 0;

    commands.spawn(TextBundle::from_section(
        "0",
        TextStyle {
            font: asset_server.load("fonts/cartoon.ttf"),
            font_size: 32.,
            color: Color::BLACK,
        })
    )
    .insert(ScoreText)
    .insert(StateScoped(GameState::Playing));
}

fn update_ui(
    mut q_text: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    let Ok(mut text) = q_text.get_single_mut() else { return };

    text.sections[0].value = format!("{}", score.current);
}