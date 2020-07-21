// G - Window test
// Desmond Germans, 2020

use e::*;
use g::*;
use std::rc::Rc;

fn main() {
    let system = Rc::new(match System::new() {
        Ok(system) => system,
        Err(_) => { panic!("Cannot open system."); },
    });

    let engine = Rc::new(match Engine::new(&system,vec2!(1024,576),vec2!(256,144)) {
        Ok(engine) => engine,
        Err(_) => { panic!("Cannot open engine."); },
    });

    while engine.running {
        system.wait();
        system.pump();
    }
}
