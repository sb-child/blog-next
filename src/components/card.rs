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

#[derive(Component)]
struct Card {
  x: f32,
  y: f32,
}
