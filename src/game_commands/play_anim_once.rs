use bevy::{prelude::*, ecs::system::SystemState, time::FixedTimesteps};
use seldom_pixel::{prelude::PxAssets, sprite::{PxSprite, PxSpriteData}, asset::PxAsset};

use crate::{mandoqueue::MandoParam, components::Person};

use super::CommandCompleteIndicator;

pub struct PlayAnimOnceCommand{
    pub entity: Entity
}

impl bevy::ecs::system::Command for PlayAnimOnceCommand {
    fn write(self, world: &mut World) {
    
    //     images: Res<ImageAssets>,
    // mut sprite_params: Sprite3dParams,

    let mut state = SystemState::<(Commands, ParamSet<(
        Res<FixedTimesteps>,
        // ResMut<FontMapping>,
        // Query<(&mut Animation, &mut AtlasSprite3dComponent)>
        )>)>::new(world);
    let (mut commands, mut set) = state.get_mut(world);

        // time: Res<FixedTimesteps>,
        
        // let step = set.p0().get_current().unwrap().step;
        // let mut query = set.p1();
        // let (mut animation, mut sprite) = query.get_mut(self.entity).unwrap();

        //     if animation.current + 1 < animation.frames.len() {
        //         animation.timer.tick(step);
        //         if animation.timer.just_finished() {
        //             sprite.index = animation.frames[animation.current];
        //             animation.current += 1;
        //         }
        //     } else {
        //         let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        //         cc.completed = true;
        //     }

        
        // for (mut animation, mut sprite) in set.p1().iter_mut() {
        //     if animation.current + 1 < animation.frames.len() {
        //         animation.timer.tick(step);
        //         if animation.timer.just_finished() {
        //             sprite.index = animation.frames[animation.current];
        //             animation.current += 1;
        //         }
        //     }
        // }
    }
}

pub fn change_anim_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    
    let mut state = SystemState::<(Commands, ParamSet<(
        Res<FixedTimesteps>,
        // ResMut<FontMapping>,
        // Query<(&mut Animation, &mut AtlasSprite3dComponent)>
        )>)>::new(world);
    let (mut commands, mut set) = state.get_mut(world);

        // time: Res<FixedTimesteps>,
        
        // let step = set.p0().get_current().unwrap().step;
        // let mut query = set.p1();
        // let (mut animation, mut sprite) = query.get_mut(self.entity).unwrap();

        //     if animation.current + 1 < animation.frames.len() {
        //         animation.timer.tick(step);
        //         if animation.timer.just_finished() {
        //             sprite.index = animation.frames[animation.current];
        //             animation.current += 1;
        //         }
        //     } else {
        //         let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        //         cc.completed = true;
        //     }

        
        // for (mut animation, mut sprite) in set.p1().iter_mut() {
        //     if animation.current + 1 < animation.frames.len() {
        //         animation.timer.tick(step);
        //         if animation.timer.just_finished() {
        //             sprite.index = animation.frames[animation.current];
        //             animation.current += 1;
        //         }
        //     }
        // }
}