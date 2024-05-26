mod life;
use life::universe::Universe;

fn main() {
    let mut u = Universe::new(64, 64, 64);
    loop {
        u.tick();
    }
}
