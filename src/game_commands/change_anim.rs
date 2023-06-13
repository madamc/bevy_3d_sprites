use bevy::{prelude::*, ecs::system::SystemState};
use seldom_pixel::asset::{PxAsset, PxAssetData};
use seldom_pixel::prelude::PxAssets;
use seldom_pixel::sprite::{PxSpriteData, PxSprite};

// use crate::mandoqueue::MandoParam;
use crate::{ui::UIMessageWindow, game_commands::CommandCompleteIndicator};
use crate::Person;

use super::mando_queue::MandoParam;

pub fn change_anim_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    
    let mut state = SystemState::<(Commands, ParamSet<(
        PxAssets<PxSprite>,
        Query<(&Handle<PxAsset<PxSpriteData>>, With<Person>)>,
        // Query<(&mut Animation, &mut AtlasSprite3dComponent)>
        )>)>::new(world);
    let (mut cmds, mut set) = state.get_mut(world);
    let runner = set.p0().load_animated("art\\mom-idle.png", 2).clone();
    let mut sprite = set.p1().get_single().unwrap().0;

    sprite = &runner;

    let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    cc.completed = true;
}