use bevy::{prelude::*, ecs::system::SystemState};
use crate::ui::CurrentTextState;

use super::CommandCompleteIndicator;

pub struct PauseQueueCommand;

impl bevy::ecs::system::Command for PauseQueueCommand {
    fn write(self, world: &mut World) {
        
        let mut state = SystemState::<(Commands, Query<Entity, With<CurrentTextState>>)>::new(world);
        let (mut cmds, query) = state.get_mut(world);
        let entity = query.get_single();

        let keyboard_input = world.get_resource::<Input<KeyCode>>().unwrap();

        if keyboard_input.just_pressed(KeyCode::Space) {
            println!("unpause");
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
            world.get_resource_mut::<Input<KeyCode>>().unwrap().clear_just_pressed(KeyCode::Space);
        } 
    }
}
