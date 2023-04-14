use bevy::{prelude::*, ecs::system::SystemState};
use kayak_ui::prelude::PreviousWidget;

use crate::{mandoqueue::{MandoQueue, MandoParam, mps}, ui::*};

use super::CommandCompleteIndicator;

pub struct AffectTypeWriterCommand {
    pub elapsed_time: u128,
    pub message: String,
}

impl bevy::ecs::system::Command for AffectTypeWriterCommand {
    fn write(self, world: &mut World) {
        // Res<Windows>,
        let screenwidth = world.get_resource::<Windows>().unwrap().get_primary().unwrap().width();
        let screenheight = world.get_resource::<Windows>().unwrap().get_primary().unwrap().height();
        
        //This needs to correspond with the width of the ui window defined in the showmessageui game command, then subtract the padding added in ui when the text is styled and rendered
        let message_width = (screenwidth * 0.7) as i32;  // .8 - .1
        let message_height = (screenheight * 0.3) as i32;  // .35 - .05

        let str_vec = create_message(&self.message, message_width);
        let line_count = message_height / 32; // this should correspond with the line height
        let iter_count = ((str_vec.len() as f32 / line_count as f32) + 1.0) as i8;
        //create a chunk to break down the array
        
        let str_slices: Vec<&[String]> = str_vec.chunks(line_count as usize).collect();
        // println!("size {} line {} len {} height {}", iter_count, line_count, str_vec.len(), message_height);
        let keyboard_input = world.get_resource::<Input<KeyCode>>().unwrap();
        let keyboard_event = keyboard_input.just_pressed(KeyCode::Space);
        // remember if you are accessing state this way, you need to manually apply state.apply(world) if you are making any command calls.
        // let mut state = SystemState::<(Commands, Query<&mut CurrentTextState, Without<PreviousWidget>>)>::new(world);
        let mut state = SystemState::<(Commands, ParamSet<(
            Query<&mut CurrentTextState, Without<PreviousWidget>>,
            ResMut<MandoQueue>,
            )>)>::new(world);
        // let (mut cmds, mut query) = state.get_mut(world);
        let (mut cmds, mut set) = state.get_mut(world);
        let mut query = set.p0();
        let current_text_opt = query.get_single_mut();
        let mut reset_timer = false;
        let mut completed = false;

        if let Ok(mut current_text) = current_text_opt {

            let varb = message_to_str(str_slices[current_text.iter as usize].clone().to_vec());
            current_text.text = varb.to_owned();
            let cps = 30; // characters per second
            let mut chars = (((self.elapsed_time as f32) / 1000.0) * cps as f32) as u128;    
 
            if current_text.chars >= varb.len() as u128{
                if current_text.iter >= iter_count - 1 && keyboard_event && !completed {
                    completed = true;
                    current_text.iter = 0;
                }
                else if current_text.iter < iter_count - 1 && keyboard_event {
                    reset_timer = true;
                    current_text.chars = 0;
                    current_text.iter += 1;
                }
            }  else {
                current_text.chars = chars;
            }
            
        } else {

        }
        if reset_timer {
             world.get_resource_mut::<MandoQueue>().unwrap().timer.reset();
        }
        if completed {
                
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
            
        }
        world.get_resource_mut::<Input<KeyCode>>().unwrap().clear_just_pressed(KeyCode::Space);

    }
}

pub fn type_writer_effect_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let message = mps(&mp[0]);
    let screen_width = world.get_resource::<Windows>().unwrap().get_primary().unwrap().width();
    let screen_height = world.get_resource::<Windows>().unwrap().get_primary().unwrap().height();
    
    //This needs to correspond with the width of the ui window defined in the showmessageui game command, then subtract the padding added in ui when the text is styled and rendered
    let message_width = (screen_width * 0.7) as i32;  // .8 - .1
    let message_height = (screen_height * 0.3) as i32;  // .35 - .05

    let str_vec = create_message(message, message_width);
    let line_count = message_height / 32; // this should correspond with the line height
    let iter_count = ((str_vec.len() as f32 / line_count as f32) + 1.0) as i8;

    //create a chunk to break down the array
    let str_slices: Vec<&[String]> = str_vec.chunks(line_count as usize).collect();
    let keyboard_input = world.get_resource::<Input<KeyCode>>().unwrap();
    let keyboard_event = keyboard_input.just_pressed(KeyCode::Space);
    // remember if you are accessing state this way, you need to manually apply state.apply(world) if you are making any command calls.
    // let mut state = SystemState::<(Commands, Query<&mut CurrentTextState, Without<PreviousWidget>>)>::new(world);
    let mut state = SystemState::<(Commands, ParamSet<(
        Query<&mut CurrentTextState, Without<PreviousWidget>>,
        ResMut<MandoQueue>,
        )>)>::new(world);
    // let (mut cmds, mut query) = state.get_mut(world);
    let (mut cmds, mut set) = state.get_mut(world);
    let mut query = set.p0();
    let current_text_opt = query.get_single_mut();
    let mut reset_timer = false;
    let mut completed = false;

    if let Ok(mut current_text) = current_text_opt {

        let varb = message_to_str(str_slices[current_text.iter as usize].clone().to_vec());
        current_text.text = varb.to_owned();
        let cps = 30; // characters per second
        let elapsed_time_for_iter = elapsed_time as f32 - current_text.elapsed_time as f32;
        let mut chars = ((elapsed_time_for_iter / 1000.0) * cps as f32) as u128;    

        if current_text.chars >= varb.len() as u128 {
            if current_text.iter >= iter_count - 1 && keyboard_event && !completed {
                completed = true;
                current_text.iter = 0;
                current_text.elapsed_time = 0;
                current_text.chars = 0;
            }
            else if current_text.iter < iter_count - 1 && keyboard_event {
                //since taking this out of the commands impl, this is no longer resetting the timer for whatever reason...not sure why
                reset_timer = true;
                current_text.elapsed_time = elapsed_time;
                current_text.chars = 0;
                current_text.iter += 1;
            }
        }  else {
            if keyboard_event {
                chars = varb.len() as u128;
            }
            let numb = elapsed_time as f32 * current_text.iter as f32;
            current_text.chars = chars;
        }
        
    } else {

    }
    if reset_timer {
        println!("2 Parksy Rarsky {}", world.get_resource_mut::<MandoQueue>().unwrap().timer.elapsed().as_millis());
        world.get_resource_mut::<MandoQueue>().unwrap().timer.reset();
        println!("3 Parksy Rarsky {}", world.get_resource_mut::<MandoQueue>().unwrap().timer.elapsed().as_millis());
    }
    if completed {
            
        println!("2 Parksy Rarsky {}", world.get_resource_mut::<MandoQueue>().unwrap().timer.elapsed().as_millis());
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
        println!("2 Parksy Rarsky {}", world.get_resource_mut::<MandoQueue>().unwrap().timer.elapsed().as_millis());
        
    }
    world.get_resource_mut::<Input<KeyCode>>().unwrap().clear_just_pressed(KeyCode::Space);
    state.apply(world);
}