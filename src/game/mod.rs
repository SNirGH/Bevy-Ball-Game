use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::GameUIPlugin;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
                GameUIPlugin,
            ))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
