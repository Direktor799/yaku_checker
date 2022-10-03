use std::time::Instant;
use yaku_checker::ReadyTileSet;

fn main() {
    let time = Instant::now();
    let tiles = "123456789p 3568m".parse::<ReadyTileSet>().unwrap();
    println!("{:?}", tiles.shanten());
    println!("took {}s", time.elapsed().as_secs_f64());
}
