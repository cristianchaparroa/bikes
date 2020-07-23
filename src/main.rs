#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

mod application;
mod bikes;

use application::rest;

fn main() {
    let _ = rest::run();
    // let _ = rest::State::new();
}
