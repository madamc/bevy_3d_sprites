use bevy::{prelude::*, ecs::system::SystemState};
use crate::mandoqueue::MandoParam;

use crate::{ui::UIMessageWindow, game_commands::CommandCompleteIndicator};

pub struct HideMessageUICommand;

impl bevy::ecs::system::Command for HideMessageUICommand {
    fn write(self, world: &mut World) {
        
        let mut state = SystemState::<(Commands, Query<Entity, With<UIMessageWindow>>)>::new(world);
        let (mut cmds, query) = state.get_mut(world);
        let entity = query.get_single();

        if let Ok(mut message_ui_entity) = entity {
            cmds.entity(message_ui_entity).despawn_recursive();
            cmds.entity(message_ui_entity).despawn();
            cmds.entity(message_ui_entity).despawn_descendants();
            println!("hide!");

            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;

        } else
        {
            println!("nothing to hide!");
        }
        state.apply(world);
    }
}

pub fn hide_message_ui_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    
    let mut state = SystemState::<(Commands, Query<Entity, With<UIMessageWindow>>)>::new(world);
    let (mut cmds, query) = state.get_mut(world);
    let entity = query.get_single();

    if let Ok(mut message_ui_entity) = entity {
        cmds.entity(message_ui_entity).despawn_recursive();
        cmds.entity(message_ui_entity).despawn();
        cmds.entity(message_ui_entity).despawn_descendants();
        println!("hide!");

        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;

    } else
    {
        println!("nothing to hide!");
    }
    state.apply(world); 
}