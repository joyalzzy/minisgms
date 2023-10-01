pub struct MinigamesPlugin;
use valence::prelude::*;
use crate::minigames::things::animate;

impl Plugin for MinigamesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, animate);
    }
}
