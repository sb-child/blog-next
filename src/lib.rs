// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod components;
pub mod sences;
use bevy::app::App;
use sences::use_sence;

pub fn run_app() {
  println!("Welcome to sbchild blog!");
  let build_text = build_info::format!("{{{} v{} built with {} at {}}}", $.crate_info.name, $.crate_info.version, $.compiler, $.timestamp);
  println!("Build Info: {build_text}");
  let mut app = App::new();
  use_sence(&mut app);
  app.run();
}
