use bevy::{prelude::*, ecs::system::SystemState};
use seldom_pixel::prelude::PxPosition;

use crate::mandoqueue::{MandoParam, mpiv2, mpf, mpe};

use super::CommandCompleteIndicator;

pub struct MoveToLoc2DIv2Command {
    pub delta: u128,
    pub elapsedTime: u128,
    pub duration: f32,
    pub location: IVec2,
    pub destination: IVec2,
    pub entity: Entity,
}

impl bevy::ecs::system::Command for MoveToLoc2DIv2Command {
    fn write(self, world: &mut World) {
        
        let elapsedTime_sec= (self.elapsedTime as f32) / 1000.0;

        let percentage: f32 = if (elapsedTime_sec as f32 > self.duration) {1.0} else {elapsedTime_sec / self.duration};
       
        let mut ynynPxP = world.get_mut::<PxPosition>(self.entity).unwrap();
        let locv2 = Vec2::new(self.location.x as f32, self.location.y as f32);
        let destv2 = Vec2::new(self.destination.x as f32, self.destination.y as f32);
        let newPos: Vec2 = locv2.lerp(destv2, percentage);
        let newIVec2 = IVec2 { x: newPos.x as i32, y: newPos.y as i32 };
        // let newPos: Vec2 = self.location.lerp(self.destination, percentage);
        ynynPxP.0 = newIVec2;
        
        if (percentage >= 1.0) {
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
        } 
    }
}

pub fn move_to_loc_2d_iv2_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let duration = mpf(&mp[0]);
    let location = mpiv2(&mp[1]);
    let destination = mpiv2(&mp[2]);
    let entity = mpe(&mp[3]);

    let elapsed_time_sec= (elapsed_time as f32) / 1000.0;

    let percentage: f32 = if elapsed_time_sec as f32 > duration {1.0} else {elapsed_time_sec / duration};

    let mut state = SystemState::<(Commands, ParamSet<(
        Query<Entity, With<Camera2d>>,
        )>)>::new(world);
    let (mut commands, mut set) = state.get_mut(world);
    
    let cam_ent = set.p0().single();
    
    let mut ynyn_px_p = world.get_mut::<PxPosition>(entity).unwrap();
    let locv2 = Vec2::new(location.x as f32, location.y as f32);
    let destv2 = Vec2::new(destination.x as f32, destination.y as f32);
    let new_pos: Vec2 = locv2.lerp(destv2, percentage);
    let new_ivec2 = IVec2 { x: new_pos.x as i32, y: new_pos.y as i32 };

    // have camera follow player
    // let newPos: Vec2 = self.location.lerp(self.destination, percentage);
    let mut x_dif = (ynyn_px_p.0.x - new_ivec2.x) as f32;
    let mut y_dif  = (ynyn_px_p.0.y - new_ivec2.y) as f32;
    if new_ivec2.x < ynyn_px_p.0.x {
        println!("peeecco");
        x_dif *= -6.85;   
    }
    if new_ivec2.y < ynyn_px_p.0.y {
        y_dif *= -6.5;   
    }
    ynyn_px_p.0 = new_ivec2;
    
    println!("claro {} {}", new_ivec2.x, ynyn_px_p.0.x);
    
    let mut cam_transf = world.get_mut::<Transform>(cam_ent).unwrap();
    
    cam_transf.translation.x += x_dif as f32;//new_ivec2.x as f32;
    cam_transf.translation.y += y_dif as f32;//new_ivec2.y as f32;
    
    if percentage >= 1.0 {
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
    } 

}