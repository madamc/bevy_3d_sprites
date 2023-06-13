use bevy::prelude::*;

use crate::components::{Panel, PanelChar};

const ZERO_X: f32 = 0.0;
const FAST_X: f32 = 0.03;
const SLOW_X: f32 = 0.0008;
const ZERO_Z: f32 = 0.0;
const FAST_Z: f32 = -0.03;

pub fn move_panel(
    mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet<(
        Query<(&mut Transform, Entity), With<Panel>>,
        Query<(&mut Transform, Entity), With<PanelChar>>,
    )>,
) {
    let mut x_velocity = 0.;
    let mut z_velocity = 0.;

    for (mut transform, entity) in &mut set.p1() {

        if keyboard_input.pressed(KeyCode::N) {
            x_velocity -= 0.03; 
        }
        
        if keyboard_input.pressed(KeyCode::M) {
            x_velocity += 0.03;
        }

        if keyboard_input.pressed(KeyCode::V) {
            x_velocity -= 0.0005; 
        }
        
        if keyboard_input.pressed(KeyCode::B) {
            x_velocity += 0.0005;
        }

        let panel_transform = transform.translation.clone();
        let panel_vec = Vec3 { x: panel_transform.x + x_velocity, y: panel_transform.y, z: panel_transform.z };
        transform.translation = panel_vec;
    }

    let mut x_velocity = 0.;
    let mut z_velocity = 0.;

    for (mut transform, entity) in &mut set.p0() {

        if keyboard_input.pressed(KeyCode::I) {
            z_velocity -= 0.02; 
        }
        
        if keyboard_input.pressed(KeyCode::K) {
            z_velocity += 0.02;
        }

        if keyboard_input.pressed(KeyCode::N) {
            x_velocity -= 0.03; 
        }
        
        if keyboard_input.pressed(KeyCode::M) {
            x_velocity += 0.03;
        }

        if keyboard_input.pressed(KeyCode::V) {
            x_velocity -= 0.0005; 
        }
        
        if keyboard_input.pressed(KeyCode::B) {
            x_velocity += 0.0005;
        }

        let panel_transform = transform.translation.clone();
        let panel_vec = Vec3 { x: panel_transform.x + x_velocity, y: panel_transform.y, z: panel_transform.z + z_velocity };
        transform.translation = panel_vec;
    }

}