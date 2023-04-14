use bevy::prelude::*;

use super::CommandCompleteIndicator;

pub struct MoveToLoc3DCommand {
    pub delta: u128,
    pub elapsedTime: u128,
    pub duration: f32,
    pub location: Vec3,
    pub destination: Vec3,
    pub entity: Entity,
}

impl bevy::ecs::system::Command for MoveToLoc3DCommand {
    fn write(self, world: &mut World) {
        
        let elapsedTime_sec= (self.elapsedTime as f32) / 1000.0;

        let percentage: f32 = if (elapsedTime_sec as f32 > self.duration) {1.0} else {elapsedTime_sec / self.duration};
       
        let mut ynynTF = world.get_mut::<Transform>(self.entity).unwrap();
        let newPos: Vec3 = self.location.lerp(self.destination, percentage);

        ynynTF.translation = newPos;
        
        if (percentage >= 1.0) {
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
        } 
    }
}