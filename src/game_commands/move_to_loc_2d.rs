use bevy::prelude::*;

use super::CommandCompleteIndicator;

pub struct MoveToLoc2DCommand {
    pub delta: u128,
    pub elapsedTime: u128,
    pub duration: f32,
    pub location: Vec2,
    pub destination: Vec2,
    pub entity: Entity,
}

impl bevy::ecs::system::Command for MoveToLoc2DCommand {
    fn write(self, world: &mut World) {
        
        let elapsed_time_sec= (self.elapsedTime as f32) / 1000.0;

        let percentage: f32 = if (elapsed_time_sec as f32 > self.duration) {1.0} else {elapsed_time_sec / self.duration};
       
        let mut ynynTF = world.get_mut::<Transform>(self.entity).unwrap();
        let new_pos: Vec2 = self.location.lerp(self.destination, percentage);

        ynynTF.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
        
        if (percentage >= 1.0) {
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
        } 
    }
}