// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod plugins;
use bevy::{
  core_pipeline::{
    bloom::{Bloom, BloomCompositeMode},
    tonemapping::Tonemapping,
  },
  prelude::*,
  sprite::Material2d,
};
use plugins::use_plugins;

pub fn use_sence(app: &mut App) -> &mut App {
  let mut app = app;
  app = use_plugins(app);
  app.add_systems(Startup, setup).add_systems(Update, update_bloom_settings);
  app
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.spawn((Camera2d::default(), Bloom::default()));

 /**
  *

    mesh: meshes.add(Circle::new(100.)).into(),
    material: materials.add(Color::srgb(7.5, 0.0, 7.5)),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),

  */

  // Sprite
  commands.spawn(Sprite {
    image: asset_server.load("image.png"),
    texture_atlas: None,
    color: Color::srgb(1.0, 1.0, 1.0),
    custom_size: Some(Vec2::splat(160.0)),
    ..default()
  });

  // Circle mesh
  commands.spawn(MeshMaterial2d {

  });

  // Hexagon mesh
  commands.spawn(MeshMaterial2d {
    mesh: meshes.add(RegularPolygon::new(100., 6)).into(),
    material: materials.add(Color::srgb(6.25, 9.4, 9.1)),
    transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
    ..default()
  });

  // UI
  commands.spawn(
    TextBundle::from_section("", TextStyle::default()).with_style(Style {
      position_type: PositionType::Absolute,
      bottom: Val::Px(12.0),
      left: Val::Px(12.0),
      ..default()
    }),
  );
}

// ------------------------------------------------------------------------------------------------

fn update_bloom_settings(
  mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
  mut text: Query<&mut Text>,
  mut windows: Query<&mut Window>,
  mut commands: Commands,
  keycode: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  let bloom_settings = camera.single_mut();
  let mut text = text.single_mut();
  let text = &mut text.sections[0].value;
  let window = windows.single();
  let window_settings = format!(
    "{}x{} cur {:?} fac {}",
    window.physical_width(),
    window.physical_height(),
    window.physical_cursor_position(),
    window.scale_factor()
  );
  match bloom_settings {
    (entity, Some(mut bloom_settings)) => {
      *text = format!("BloomSettings (Toggle: Space) {}\n", window_settings);
      text
        .push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
      text.push_str(&format!(
        "(W/S) Low-frequency boost: {}\n",
        bloom_settings.low_frequency_boost
      ));
      text.push_str(&format!(
        "(E/D) Low-frequency boost curvature: {}\n",
        bloom_settings.low_frequency_boost_curvature
      ));
      text.push_str(&format!(
        "(R/F) High-pass frequency: {}\n",
        bloom_settings.high_pass_frequency
      ));
      text.push_str(&format!("(T/G) Mode: {}\n", match bloom_settings
        .composite_mode
      {
        BloomCompositeMode::EnergyConserving => "Energy-conserving",
        BloomCompositeMode::Additive => "Additive",
      }));
      text.push_str(&format!(
        "(Y/H) Threshold: {}\n",
        bloom_settings.prefilter_settings.threshold
      ));
      text.push_str(&format!(
        "(U/J) Threshold softness: {}\n",
        bloom_settings.prefilter_settings.threshold_softness
      ));

      if keycode.just_pressed(KeyCode::Space) {
        commands.entity(entity).remove::<BloomSettings>();
      }

      let dt = time.delta_seconds();

      if keycode.pressed(KeyCode::KeyA) {
        bloom_settings.intensity -= dt / 10.0;
      }
      if keycode.pressed(KeyCode::KeyQ) {
        bloom_settings.intensity += dt / 10.0;
      }
      bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);

      if keycode.pressed(KeyCode::KeyS) {
        bloom_settings.low_frequency_boost -= dt / 10.0;
      }
      if keycode.pressed(KeyCode::KeyW) {
        bloom_settings.low_frequency_boost += dt / 10.0;
      }
      bloom_settings.low_frequency_boost =
        bloom_settings.low_frequency_boost.clamp(0.0, 1.0);

      if keycode.pressed(KeyCode::KeyD) {
        bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
      }
      if keycode.pressed(KeyCode::KeyE) {
        bloom_settings.low_frequency_boost_curvature += dt / 10.0;
      }
      bloom_settings.low_frequency_boost_curvature =
        bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);

      if keycode.pressed(KeyCode::KeyF) {
        bloom_settings.high_pass_frequency -= dt / 10.0;
      }
      if keycode.pressed(KeyCode::KeyR) {
        bloom_settings.high_pass_frequency += dt / 10.0;
      }
      bloom_settings.high_pass_frequency =
        bloom_settings.high_pass_frequency.clamp(0.0, 1.0);

      if keycode.pressed(KeyCode::KeyG) {
        bloom_settings.composite_mode = BloomCompositeMode::Additive;
      }
      if keycode.pressed(KeyCode::KeyT) {
        bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
      }

      if keycode.pressed(KeyCode::KeyH) {
        bloom_settings.prefilter_settings.threshold -= dt;
      }
      if keycode.pressed(KeyCode::KeyY) {
        bloom_settings.prefilter_settings.threshold += dt;
      }
      bloom_settings.prefilter_settings.threshold =
        bloom_settings.prefilter_settings.threshold.max(0.0);

      if keycode.pressed(KeyCode::KeyJ) {
        bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
      }
      if keycode.pressed(KeyCode::KeyU) {
        bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
      }
      bloom_settings.prefilter_settings.threshold_softness =
        bloom_settings.prefilter_settings.threshold_softness.clamp(0.0, 1.0);
    },

    (entity, None) => {
      *text = "Bloom: Off (Toggle: Space)".to_string();

      if keycode.just_pressed(KeyCode::Space) {
        commands.entity(entity).insert(BloomSettings::default());
      }
    },
  }
}
