use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{seldom_pixel_prep::{YNYNWriteComp, YNYNSitComp}, VoxelChar, VoxelVelocity};

pub fn move_plyr(
    mut commands: Commands, 
    keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet<(
        Query<(&mut Transform, &mut VoxelVelocity, Entity), With<VoxelChar>>,
        Query<&mut Transform, With<Camera3d>>, 
    )>,
) {
    let player_is_empty = set.p0().is_empty();
    let camera_is_empty = set.p1().is_empty();
        if !player_is_empty && !camera_is_empty {
        // let form = set.p0().single_mut();

        let mut x_velocity = 0.;
        let mut z_velocity = 0.;
        let mut x_direction = 0.;
        let mut z_direction = 0.;

        if keyboard_input.pressed(KeyCode::Left) {
            x_velocity -= 0.01; 
            x_direction = -1.;
        }
        
        if keyboard_input.pressed(KeyCode::Right) {
            x_velocity += 0.01;
            x_direction = 1.;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            z_velocity -= 0.08; 
            z_direction = -1.;
        }
        
        if keyboard_input.pressed(KeyCode::Down) {
            z_velocity += 0.08;
            z_direction = 1.;
        }
        
        let entity = set.p0().single_mut().2;
        let velocity_vec = Vec3 { x: x_direction, y: 0., z: z_direction};
        
        let mut cam_transform = set.p1().single_mut().translation;
        let cam_vec = Vec3 { x: cam_transform.x + x_velocity, y: cam_transform.y, z: cam_transform.z + z_velocity};
        set.p1().single_mut().translation = cam_vec;

        
        //
        if keyboard_input.pressed(KeyCode::Y) {
            // println!("triggered to sit");
            commands.entity(entity).insert(YNYNSitComp);
        }
        if keyboard_input.pressed(KeyCode::U) {
            // println!("triggered to write");
            commands.entity(entity).insert(YNYNWriteComp);
        }
        //
        let plyr_transform = set.p0().single_mut().0.translation.clone();
        let plyr_vec = Vec3 { x: plyr_transform.x + x_velocity, y: plyr_transform.y, z: plyr_transform.z + z_velocity };
        let mut plyr_rot = set.p0().single_mut().0.rotation;
        if velocity_vec.x < 0. && velocity_vec.z == 0. {
            plyr_rot = Quat::from_axis_angle(Vec3::Y, -PI/2.);
        } else if velocity_vec.x == 0. && velocity_vec.z > 0. {
            plyr_rot = Quat::from_axis_angle(Vec3::Y, 0.);
        } else if velocity_vec.x == 0. && velocity_vec.z < 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI);
        } else if velocity_vec.x > 0. && velocity_vec.z == 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI/2.);
        } else if velocity_vec.x < 0. && velocity_vec.z < 0. {
            plyr_rot = Quat::from_axis_angle(Vec3::Y, -(PI/4.+PI/2.));
        } else if velocity_vec.x < 0. && velocity_vec.z > 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, -PI/4.);
        } else if velocity_vec.x > 0. && velocity_vec.z < 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI/2. + PI/4.);
        } else if velocity_vec.x > 0. && velocity_vec.z > 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI/4.);
        }

        
        // let left_bound = LEFT_WALL;
        // let right_bound = RIGHT_WALL;

        set.p0().single_mut().0.translation = plyr_vec;
        set.p0().single_mut().0.rotation = plyr_rot;
        set.p0().single_mut().1.vector = velocity_vec; //Vec3 { x: x_direction, y: 0., z: z_direction};
        
    }   
}
    