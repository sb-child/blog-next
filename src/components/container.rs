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

#[derive(Component, Clone, Debug)]
pub struct ContainerState {}

#[derive(Bundle, Clone, Debug)]
pub struct ContainerBundle {
  pub node: NodeBundle,
  pub state: ContainerState,
}

pub fn use_container(app: &mut App) -> &mut App {
  // NodeBundle {
  //   style: Style {
  //       width: Val::Percent(100.0),
  //       height: Val::Percent(100.0),
  //       align_items: AlignItems::Center,
  //       justify_content: JustifyContent::Center,
  //       ..default()
  //   },
  //   ..default()
  // }
  app.add_systems(Update, container_update_event)
}

pub fn container_update_event(
  mut container: Query<&mut ContainerState>,
  mut commands: Commands,
  keycode: Res<ButtonInput<KeyCode>>,
  // mouse: Query<&Interaction>,
  time: Res<Time>,
) {
}

fn test() {
  // let s = NodeBundle {};
}
