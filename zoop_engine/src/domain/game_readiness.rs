use bevy::prelude::*;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameReadiness {
    #[default]
    LoadingAssets,
    LoadingScene,
    Ready,
}
