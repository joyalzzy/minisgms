pub mod plugin;
pub mod manager;
pub mod spleef;

use valence::{prelude::*, Layer};

trait Minigame {
    fn init(&self, world: &mut World, commands: &mut Commands);
    fn update () {

    }
    fn end() {

    }
   
}

