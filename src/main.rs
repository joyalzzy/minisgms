#![allow(clippy::type_complexity)]

mod anims;
use itertools;
use valence::abilities::{FlyingSpeed, FovModifier, PlayerAbilitiesFlags};
use valence::prelude::*;
use crate::anims::plugin;

const SPAWN_POS: DVec3 = DVec3::new(0.0, 256.0, 0.0);


/* #[derive(Parser, Resource)]
#[clap(author, version, about)]
struct Cli {
    /// The path to a Minecraft world save containing a `region` subdirectory.
    path: PathBuf,
} */

pub fn main() {
    // let cli = Cli::parse();

    // if !cli.path.exists() {
    //     eprintln!(
    //     "Directory `{}` does not exist. Exiting.",
    //     cli.path.display()
    // );
    //     return;
    // }

   // if !cli.path.is_dir() {
    //     eprintln!("`{}` is not a directory. Exiting.", cli.path.display());
    //     return;
    // }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugin::AnimsPlugin)
        //.insert_resource(cli)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                despawn_disconnected_clients,
                (init_clients).chain(),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    server: Res<Server>
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for (x,z) in itertools::iproduct!(-5..5, -5..5) {
    
        layer.chunk.insert_chunk([x,z], UnloadedChunk::new());
    }
    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
    (
    &mut EntityLayerId,
    &mut VisibleChunkLayer,
    &mut VisibleEntityLayers,
    &mut Position,
    &mut GameMode,
    &mut PlayerAbilitiesFlags,
    &mut FlyingSpeed,
    &mut FovModifier,
),
    Added<Client>,
    >,
    layers: Query<Entity, With<ChunkLayer>>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut abilities,
        mut flying_speed,
        mut fov_modifier,
    ) in &mut clients
        {
            let layer = layers.single();

            layer_id.0 = layer;
            visible_chunk_layer.0 = layer;
            visible_entity_layers.0.insert(layer);
            pos.set(SPAWN_POS);
            *game_mode = GameMode::Creative;
            abilities.set_allow_flying(true);
            flying_speed.0 = 0.1;
            // fov_modifier.0 = 0.05;
        }
}


