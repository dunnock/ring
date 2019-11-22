#[path="../../build.rs"]
mod build;

use build::ring_build_rs_main;

fn main() {
  let crate_root = std::env::current_dir().unwrap();
  //dbg!(std::env::current_dir().unwrap());
  std::env::set_current_dir("../..").unwrap();
  ring_build_rs_main();
  std::env::set_current_dir(crate_root).unwrap();
  dbg!(std::env::current_dir().unwrap());
}

