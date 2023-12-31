pub struct AnimsPlugin;
use valence::prelude::*;
use crate::anims::things::animate;

impl Plugin for AnimsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, animate);
    }
}
