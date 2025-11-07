extern crate dirs;
extern crate rand;
extern crate serde;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

pub mod args;
pub mod error;
pub mod first_run;
pub mod project_paths;
pub mod rand_names;
pub mod tmux;

pub struct Debug(AtomicBool);

impl Debug {
    pub fn load(&self) -> bool {
        self.0.load(Relaxed)
    }

    pub fn swap(&self, value: bool) -> bool {
        self.0.swap(value, Relaxed)
    }
}

pub static DEBUG: Debug = Debug(AtomicBool::new(false));
