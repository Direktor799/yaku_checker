use std::io::stdin;
use yaku_checker::ReadyTileSet;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut tiles = input.trim().parse::<ReadyTileSet>().unwrap();
    loop {
        println!("{:?}", tiles.check());
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let full_tiles = tiles.draw(input.trim().parse().unwrap());
        println!("{:?}", full_tiles.yakus());
        input.clear();
        stdin().read_line(&mut input).unwrap();
        tiles = full_tiles.discard(input.trim().parse().unwrap()).unwrap();
    }
}
