use valence::{prelude::*, network::async_trait};
use crate::minigames::Minigame;

pub struct SpleefMinigame {
}       

impl Minigame for SpleefMinigame {    
    fn init(&self, world: &mut World, commands: &mut Commands){
        world.iter_entities().for_each(|b| {println!("{:?}", b.location())})
    }
} 
