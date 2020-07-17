#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

mod application;
mod bikes;

use application::rest;

fn main() {
    rest::run();
}
