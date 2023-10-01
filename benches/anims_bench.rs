use criterion::{black_box, criterion_group, criterion_main, Criterion};

use itertools;
use valence::prelude::*;

use minigames::anims;



const SPAWN_POS: DVec3 = DVec3::new(0.0, 256.0, 0.0);


/* #[derive(Parser, Resource)]
#[clap(author, version, about)]
struct Cli {
    /// The path to a Minecraft world save containing a `region` subdirectory.
    path: PathBuf,
} */

pub fn bench_main(c: &mut Criterion) {
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

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.update();


    let mut layer = LayerBundle::new(
        ident!("overworld"),
        app.world.resource::<DimensionTypeRegistry>(),
        app.world.resource::<BiomeRegistry>(),
        app.world.resource::<Server>(),
    );    
    for (x,z) in itertools::iproduct!(-5..5, -5..5) {

        layer.chunk.insert_chunk([x,z], UnloadedChunk::new());
    }
    app.world.spawn(layer);
    app.add_plugins(anims::plugin::AnimsPlugin);
    app.update();

    // for (
    //     mut layer_id,
    //     mut visible_chunk_layer,
    //     mut visible_entity_layers,
    //     mut pos,
    //     mut game_mode,
    //     mut abilities,
    //     mut flying_speed,
    //     mut fov_modifier,
    // ) in &mut clients
    //     {
    //         let layer = layers.single();
    //
    //         layer_id.0 = layer;
    //         visible_chunk_layer.0 = layer;
    //         visible_entity_layers.0.insert(layer);
    //         pos.set(SPAWN_POS);
    //         *game_mode = GameMode::Creative;
    //         abilities.set_allow_flying(true);
    //         flying_speed.0 = 0.1;
    //         // fov_modifier.0 = 0.05;
    //     }


    c.bench_function("bench",|b| {
        b.iter(|| {app.update()
        })
    });

}

criterion_group!(benches, bench_main);
criterion_main!(benches);
