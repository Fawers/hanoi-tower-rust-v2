use hanoi_tower::game::Game;

fn main() {
    let mut g = Game::new();

    println!("{:?}", g);
    g.play(g.default_read_input());
}
