use itertools;
use valence::prelude::*;
use valence::math::{Vec3Swizzles,IVec3,DVec3};
use libm::{sin,cos,sqrt};
use std::f64::consts::{PI};

const SPAWN_POS: DVec3 = DVec3::new(0.0, 256.0, 0.0);
static mut PAST : Vec<IVec3> = vec![];
const CHARS : &str = r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#;
const LEN : usize = CHARS.len();


// gives out a vec of all pos
fn spinning_disk  (
    r: i32,
    t: i32
) -> Vec<(i32, i32, i32)>
{
    let mut poses : Vec<(i32, i32, i32)> = vec![];
    let pis = 2.0*r as f64/PI;
    let amp = (20 * t) as f64;
    for (x, z) in itertools::iproduct!((-1 * r)..r,(-1 * r)..r){
        if (sqrt((x*x + z*z) as f64) > r as f64 )  {
            continue;
        }
        let y = (SPAWN_POS.y - ( amp *sin((x as f64/pis).into())) - (amp *sin((z as f64/pis).into()))) as i32;
        poses.push((x,y,z))
        
    }
    return poses;

}

fn weird_shape  (
    r: i32,
    t: i32
) -> Vec<IVec3>
{
     
    let mut poses : Vec<IVec3>  = vec![];
    let pis = 2.0*r as f64/PI;
    let amp = (t) as f64;
    let range =  (-1 * r)..r;
    let mut visual : Vec<Vec<char>> = vec![vec!['0'; 2*(r as usize)]; 2*(r as usize)];

    for (x, z) in itertools::iproduct!(range.clone(),range.clone()){
        // if (sqrt((x*x + z*z) as f64) > r as f64)  {
        //     continue;
        // }
        let yi = (amp * (x as f64/pis).cos()) + (amp *(z as f64/pis).cos());
        // println!("{:?}", yi);
        let mut y = (SPAWN_POS.y - yi).abs();
        y = if y > 256.0 {yi} else {y};

        visual[(x + r) as usize][(z+r) as usize] = shadeness(y); 
        poses.push(IVec3::new(x,y as i32,z));
    }
    // for row in &visual {
    //     for &y in row {
    //         print!("{}", y);
    //     }
    //     println!();
    // }
    return poses;

}

fn sinness  (
    r: i32,
    t: i32
) -> Vec<(i32, i32, i32)>
{
    let mut poses : Vec<(i32, i32, i32)> = vec![];
    let pis = 2.0*r as f64/PI;
    let amp = (50) as f64;
    for (x, z) in itertools::iproduct!((-1 * r)..r,(-1 * r)..r){
        if (sqrt((x*x + z*z) as f64) > r as f64 )  {
            continue;
        }
        let y = (SPAWN_POS.y - ( amp *sin((x as f64*pis).into())) - (amp *sin((z as f64*pis).into()))) as i32;
        poses.push((x,y,z))
        
    }
    return poses;

}

pub fn animate (
    mut layers: Query<&mut ChunkLayer>,
    server: Res<Server>,
) {
    if server.current_tick() % 5 != 0 {
        return;
    }
    let mut layer = layers.single_mut();
    let poses =  weird_shape(50,  (server.current_tick() / 5) as i32);

    unsafe {
        for pos in PAST.clone() {
            layer
                .set_block(pos.to_array(), BlockState::AIR);
        }
        PAST = poses.clone();
    }
    for p in poses {
        layer
            .set_block(p.to_array(), BlockState::RED_WOOL);
    }

}


fn shadeness (y : f64) -> char {
    if let Some(shade) = CHARS.chars().nth(((y/256.0)* LEN as f64) as usize) {
        return shade;
    }
    return 'O';
}
