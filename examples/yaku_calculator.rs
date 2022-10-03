use std::io::stdin;
use yaku_checker::ReadyTileSet;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let tiles = input.trim().parse::<ReadyTileSet>().unwrap();
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let tiles = tiles.draw(input.trim().parse().unwrap());
        println!("{:?}", tiles.yakus())
    }
}
