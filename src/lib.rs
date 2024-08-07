// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bevy::app::App;

pub fn run_app() {
  println!("Welcome to sbchild blog!");
  let build_text = build_info::format!("{{{} v{} built with {} at {}}}", $.crate_info.name, $.crate_info.version, $.compiler, $.timestamp)
  App::new().run();
}
