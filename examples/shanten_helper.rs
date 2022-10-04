use std::time::Instant;
use yaku_checker::ReadyTileSet;

fn main() {
    let time = Instant::now();
    let tiles = "1245678p hatsu chun 258m haku"
        .parse::<ReadyTileSet>()
        .unwrap();
    println!("{:?}", tiles.shanten());
    println!("took {}s", time.elapsed().as_secs_f64());
}
