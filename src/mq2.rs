use bevy::{
    prelude::*, 
    time::Stopwatch,
};
use seldom_pixel::prelude::PxPosition;
use std::collections::VecDeque;
use iyes_loopless::prelude::*;

use crate::{mandoqueue::MandoParam, components::{Person, Door}, ui::UIMessageWindow, mando, game_commands::{ynyn_anim_state_chgs::{ynyn_walk_l_cmd, ynyn_idle_l_cmd}, move_to_loc_2d_i::move_to_loc_2d_iv2_cmd, show_message_ui::show_message_ui_cmd, type_writer_effect::type_writer_effect_cmd, hide_message_ui::{self, hide_message_ui_cmd}, pause_queue::change_vox_model}, YNYN_Y, messages::{Message1, Message2, Message3, Message4, Message5, Message6}, VoxelChar, ImageAssets};
use crate::game_commands::GameCommandsExt;
use crate::components::CommandCompleteIndicator;
use crate::game_commands::pause_queue::pause_queue_cmd;

#[derive(Resource)]

pub struct MandoQueue2 {
    pub mandos: VecDeque<Vec<Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>>>,
    pub current_mando: Vec<Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>>,
    pub timer: Stopwatch,
}

impl Default for MandoQueue2 {
    fn default() -> Self {
        let mando: Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)> = Mando2 { mando_func: test_funco, params: (vec![MandoParam::Int(1)]) };
        MandoQueue2 { mandos: VecDeque::new(), current_mando: vec![mando], timer: Stopwatch::new() }
    }
}

#[derive(Clone)]
pub struct Mando2<F>
where F: Fn(std::vec::Vec<MandoParam>, &mut World, u128, u128) {
    pub mando_func: F,
    pub params: Vec<MandoParam>,
}

//test function to keep handy for making function objects
pub fn test_funco(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let first = mp.get(0);
    if let Some(f_e) = first {
        if let MandoParam::Float(f_v) = f_e {
            println!("The value is {}", f_v);
        }
    }
}

type Mp = MandoParam;

pub fn fill_mando_queue2(
    mut commands: Commands,
    mut mando_queue: ResMut<MandoQueue2>,
    assets: Res<ImageAssets>,
    mut set: ParamSet<(
        Query<Entity, With<VoxelChar>>,
        Query<Entity, With<UIMessageWindow>>,
        Query<Entity, With<Door>>,
    )>,

) {
    let plyr_entity = set.p0().single();

    // mando_queue.mandos.push_back(mando!(pause_queue_cmd; vec![Mp::Null]));
    // mando_queue.mandos.push_back(mando!(ynyn_walk_l_cmd; vec![Mp::BevyEntity(plyr_entity)]));
    mando_queue.mandos.push_back(mando!(pause_queue_cmd; vec![Mp::Null]));

    // mando_queue.mandos.push_back(mando!(ynyn_idle_l_cmd; vec![Mp::BevyEntity(plyr_entity)]));
    // mando_queue.mandos.push_back(mando!(pause_queue_cmd; vec![Mp::Null]));

    

    mando_queue.mandos.push_back(mando!(show_message_ui_cmd; vec![Mp::Null]));

    mando_queue.mandos.push_back(mando!(type_writer_effect_cmd; vec![Mp::String(Message3.to_owned())]));

    mando_queue.mandos.push_back(mando!(type_writer_effect_cmd; vec![Mp::String(Message4.to_owned())]));

    mando_queue.mandos.push_back(mando!(type_writer_effect_cmd; vec![Mp::String(Message5.to_owned())]));

    mando_queue.mandos.push_back(mando!(type_writer_effect_cmd; vec![Mp::String(Message6.to_owned())]));

    mando_queue.mandos.push_back(mando!(hide_message_ui_cmd; vec![Mp::Null]));

    mando_queue.mandos.push_back(mando!(pause_queue_cmd; vec![Mp::Null]));

    commands.holder_mando();

}

pub fn operate_mando_queue2 (
    mut commands: Commands,
    mut mando_queue: ResMut<MandoQueue2>,
    mut set: ParamSet<(
        ResMut<CommandCompleteIndicator>,
        Res<FixedTimesteps>, 
    )>, 
) 
{   
    if set.p0().completed && mando_queue.mandos.len() > 0 {
        let first_mando: Vec<Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>> = mando_queue.mandos.pop_front().unwrap();

        mando_queue.current_mando = first_mando;

        set.p0().completed = false;
        mando_queue.timer.reset();
        println!("Next Mando");
       
    }
    if set.p0().completed && mando_queue.mandos.len() == 0 {

    } else {
    let step = set.p1().get_current().unwrap().step;
    mando_queue.timer.tick(step);
    let elapsed_time = mando_queue.timer.elapsed().as_millis();
    // println!("Yarno {}", elapsed_time);
    let mut completeCommand = false;
    for mando in &mando_queue.current_mando {
        commands.filler_mando(mando.clone(), step.as_millis(), elapsed_time);
    }

    if completeCommand {
        set.p0().completed = true;
    }
    }
}

// Can be used as an alternative to the declarative macro if it doesn't work out
fn add_mando(mand_func: fn(std::vec::Vec<MandoParam>, &mut World, u128, u128), params: Vec<MandoParam>) -> Vec<Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>> {
    {
        let mut mando_vector: Vec<Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>> = Vec::new();
        let mando: Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)> = Mando2 { mando_func: mand_func, params: params };
        mando_vector.push(mando);
        mando_vector
    }
}
