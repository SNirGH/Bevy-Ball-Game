use bevy::prelude::*;

pub mod resources;
mod systems;

use crate::AppState;

use {resources::*, systems::*};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(
                Update,
                (
                    update_score.run_if(in_state(AppState::Game)),
                    update_high_scores,
                    high_scores_updated,
                ),
            )
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
