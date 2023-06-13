use bevy::{prelude::*, ecs::system::SystemState};
use kayak_ui::prelude::PreviousWidget;

use crate::{ui::*};

use super::CommandCompleteIndicator;
use super::mando_queue::*;

pub fn type_writer_effect_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let message = mps(&mp[0]);
    let line_width: i32 = mpf(&mp[1]) as i32;
    let line_count: i32 = mpf(&mp[2]) as i32;

    
    // get an array of strings that span the width
    let str_vec = create_message(message, line_width);

    //break the string array into chunks that span the desired height within the textbox. This allows us to process messages regardless of length without spilling out of the text box.
    let str_slices: Vec<&[String]> = str_vec.chunks(line_count as usize).collect();
    let keyboard_input = world.get_resource::<Input<KeyCode>>().unwrap();
    let keyboard_event = keyboard_input.just_pressed(KeyCode::Space);
    // remember if you are accessing state this way, you need to manually apply state.apply(world) if you are making any command calls.
    // let mut state = SystemState::<(Commands, Query<&mut CurrentTextState, Without<PreviousWidget>>)>::new(world);
    let mut state = SystemState::<(Commands, ParamSet<(
        Query<&mut CurrentTextState, Without<PreviousWidget>>,
        ResMut<MandoQueue>,
        Query<&mut PortraitAtlasState, Without<PreviousWidget>>,
        )>)>::new(world);
    let (mut cmds, mut set) = state.get_mut(world);
    let mut query = set.p0();
    let current_text_opt = query.get_single_mut();
    // These two flags are created to avoid angering the borrow checker. The reset_timer is used to help render a long string. 
    //  If the string exceeds the size of the box, reset_timer is set to true, and the text box empties out the text, and starts
    //   over from where the string chunks left off. Completed is used to detect when the entire string is finished.
    let mut reset_timer = false;
    let mut completed = false;

    if let Ok(mut current_text) = current_text_opt {
        
        let text_to_render_on_box = message_to_str(str_slices[current_text.iter as usize].clone().to_vec());
        current_text.text = text_to_render_on_box.to_owned();
        let cps = 30; // characters per second
        let elapsed_time_for_iter = elapsed_time as f32 - current_text.elapsed_time as f32;
        let mut chars = ((elapsed_time as f32 / 1000.0) * cps as f32) as u128;    
        // println!("chars, {} et {} ct et {} et fi {}", current_text.chars, elapsed_time, current_text.elapsed_time, elapsed_time_for_iter);

        if current_text.chars >= text_to_render_on_box.len() as u128 {
            if current_text.iter >= (str_slices.len() - 1) as i8 && keyboard_event && !completed {
                completed = true;
                current_text.iter = 0;
                current_text.elapsed_time = 0;
                current_text.chars = 0;
            }
            else if current_text.iter < (str_slices.len() - 1) as i8 && keyboard_event {
                //since taking this out of the commands impl, this is no longer resetting the timer for whatever reason...not sure why
                reset_timer = true;
                current_text.elapsed_time = elapsed_time;

                current_text.chars = 0;
                current_text.iter += 1;
            }
        }  else {
            if keyboard_event {
                chars = text_to_render_on_box.len() as u128;
            }
            let numb = elapsed_time as f32 * current_text.iter as f32;
            current_text.chars = chars;
        }
        
    } else {

    }
    let mut query = set.p2();
    let portrait_opt = query.get_single_mut();
    if let Ok(mut portrait) = portrait_opt {
        portrait.elapsed_time = elapsed_time;
    }

    if reset_timer {
        world.get_resource_mut::<MandoQueue>().unwrap().timer.reset();
    }
    if completed {
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;        
    }
    world.get_resource_mut::<Input<KeyCode>>().unwrap().clear_just_pressed(KeyCode::Space);
    state.apply(world);
}