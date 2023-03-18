// 1. Bring the macros `error, warn, info, debug, trace` into scope from the log package with a
// `use` statement.
use log::{error,warn,info,debug,trace};
// You should be able to run `cargo build --lib` successfully after this step (and each step in this
// file)
//
// Hint: You need to update Cargo.toml to add the `log` dependency, first.

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        debug!(target: "Frog::new", "A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        info!(target: "Frog::hop", "a Frog hopped. {} energy left", self.energy);
        if self.energy == 0 {
            warn!(target: "Frog::hop", "the frog will go to sleep since he ran out of energy");
            self.sleep();
        }
    }
    pub fn sleep(&mut self) {
        if self.sleeping {
            error!(target: "Frog::sleep", "Frog is already asleep");
        } else {
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        let frog = Frog {
            energy: 5,
            sleeping: false,
        };
        trace!(target: "Frog::default" ,"a new default Frog was generated {:?}", frog);

        frog
    }
}
