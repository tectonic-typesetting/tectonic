extern crate tectonic;

use tectonic::Engine;

fn main() {
    let mut e = Engine::new ();
    e.process ("trip.tex");
}
