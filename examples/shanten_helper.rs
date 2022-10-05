use std::time::Instant;
use yaku_checker::ReadyTileSet;

fn main() {
    let time = Instant::now();
    // haku hatsu chun
    let tiles = "123456789p 238m hatsu".parse::<ReadyTileSet>().unwrap();
    // let tiles = "23455m 234677p 35s".parse::<ReadyTileSet>().unwrap();
    // let tiles = "1p2 4p2 7p2 1m2 4m2 78s haku"
    // .parse::<ReadyTileSet>()
    // .unwrap();
    // let tiles = "19p 19s 19m ton nan shaa pei haku 12s"
    //     .parse::<ReadyTileSet>()
    //     .unwrap();
    println!("{:?}", tiles.check());
    println!("took {}s", time.elapsed().as_secs_f64());
}
