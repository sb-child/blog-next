// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bevy::{
  core_pipeline::{
    bloom::{BloomCompositeMode, BloomSettings},
    tonemapping::Tonemapping,
  },
  prelude::*,
  sprite::MaterialMesh2dBundle,
};
// use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

pub fn use_plugins(app: &mut App) -> &mut App {
  app.add_plugins(DefaultPlugins.set(AssetPlugin {
    file_path: "../assets".to_owned(),
    processed_file_path: "../assets".to_owned(),
    watch_for_changes_override: None,
    mode: AssetMode::Unprocessed,
    meta_check: bevy::asset::AssetMetaCheck::Always,
  }))
  // .add_plugins(FpsOverlayPlugin {
  //   config: FpsOverlayConfig {
  //     text_config: TextStyle {
  //       font_size: 16.0,
  //       color: Color::srgb(0.0, 1.0, 0.0),
  //       font: default(),
  //     },
  //   },
  // })
}
