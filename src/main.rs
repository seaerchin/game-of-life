mod life;
use life::universe::Universe;

fn main() {
    let mut u = Universe::new(64, 64, 64);
    print!("\x1B[1;1H");
    loop {
        u.tick();
    }
}
