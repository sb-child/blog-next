// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod utils;

use sbchild_blog::run_app;
use wasm_bindgen::prelude::*;

extern crate wee_alloc;

#[global_allocator]
/// global allocator used by wee_alloc
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  /// location changed event.
  ///
  /// `loc` is the location of current window.
  fn on_location_change(loc: String);
}

#[wasm_bindgen]
pub fn main() {
  println!("=== Browser Release Environment ===");
  utils::set_panic_hook();
  run_app();
}
