use bevy::prelude::*;
use crate::mandoqueue::MandoParam;

use super::CommandCompleteIndicator;

pub struct PauseQueueCommand;

impl bevy::ecs::system::Command for PauseQueueCommand {
    fn write(self, world: &mut World) {
        
        // let mut state = SystemState::<(Commands, Query<Entity, With<CurrentTextState>>)>::new(world);
        // let (mut cmds, query) = state.get_mut(world);
        // let entity = query.get_single();

        // let keyboard_input = world.get_resource::<Input<KeyCode>>().unwrap();

        // if keyboard_input.just_pressed(KeyCode::Space) {
        //     println!("unpause");
        //     let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        //     cc.completed = true;
        //     world.get_resource_mut::<Input<KeyCode>>().unwrap().clear_just_pressed(KeyCode::Space);
        // } 
        // let val = pause_queue_cmd;
        // pause_queue_cmd2(world);
    }
}

// pub fn pause_queue_cmd2(world: &mut World) {
//     let mut state = SystemState::<(Commands, Query<Entity, With<CurrentTextState>>)>::new(world);
//     let (mut cmds, query) = state.get_mut(world);
//     let entity = query.get_single();
    
//     let keyboard_input = world.get_resource::<Input<KeyCode>>().unwrap();

//     if keyboard_input.just_pressed(KeyCode::Space) {
//         println!("unpause");
//         let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
//         cc.completed = true;
//         world.get_resource_mut::<Input<KeyCode>>().unwrap().clear_just_pressed(KeyCode::Space);
//     } 
// }

pub fn pause_queue_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let keyboard_input = world.get_resource::<Input<KeyCode>>().unwrap();

    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
        world.get_resource_mut::<Input<KeyCode>>().unwrap().clear_just_pressed(KeyCode::Space);
    } 
}