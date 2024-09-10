// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod components;
pub mod sense;
use bevy::app::App;
use sense::use_sense;

pub fn run_app() {
  println!("Welcome to sbchild blog!");
  let build_text = build_info::format!("{{{} v{} built with {} at {}}}", $.crate_info.name, $.crate_info.version, $.compiler, $.timestamp);
  println!("Build Info: {build_text}");
  let mut app = App::new();
  use_sense(&mut app);
  app.run();
}
