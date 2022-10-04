use std::time::Instant;
use yaku_checker::ReadyTileSet;

fn main() {
    let time = Instant::now();
    let tiles = "12345678p hatsu 258m haku".parse::<ReadyTileSet>().unwrap();
    // let tiles = "19p 19s 19m ton nan shaa pei haku 12s"
    // .parse::<ReadyTileSet>()
    // .unwrap();
    println!("{:?}", tiles.shanten());
    println!("took {}s", time.elapsed().as_secs_f64());
}
