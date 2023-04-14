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

// What will calling this macro look like?
// go_mando[mandoType, ]

// This macro needs to do the below for every unique command:
/***
 * Define a custom command struct
 * impl Command for the custom Command struct
 * be able to receive a parameterized function
 * 
 * Create a trait that lists all of the functions that will be used
 * implement that trait for Commands and call the custom command stuct's function
 * 
 * Add an enum to the mandotype enum that corresponds with the custom command struct
 * 
 * Modify the engine to check for this new mandotype call the associated function
 * 
 * Receive a list of parameters that the user will want to provide to the method
 * 
 * challenge: How will a user be able to script a sequence of commands if they're defined by the macro?
 */