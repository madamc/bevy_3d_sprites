use bevy::{
    prelude::*, 
    time::Stopwatch,
};
use seldom_pixel::prelude::PxPosition;
use std::collections::VecDeque;
use iyes_loopless::prelude::*;

use crate::{ components::{Person, Door}, 
            ui::UIMessageWindow, mando, 
            game_commands::{
                ynyn_anim_state_chgs::{ynyn_walk_l_cmd, ynyn_idle_l_cmd}, 
                move_to_loc_2d_i::move_to_loc_2d_iv2_cmd, 
                show_message::show_message_ui_cmd, 
                type_writer_effect::type_writer_effect_cmd, 
                hide_message_ui::{self, hide_message_ui_cmd}, 
                pause_queue::change_vox_model, show_small_message::show_small_message_ui_cmd, show_portrait_message::show_portrait_message_cmd}, 
            YNYN_Y, 
            messages::{Message1, Message2, Message3, Message4, Message5, Message6, Message7, Message8}, VoxelChar, 
            };
use crate::game_setup::load_assets::ImageAssets;
use crate::game_commands::GameCommandsExt;
use crate::components::CommandCompleteIndicator;
use crate::game_commands::pause_queue::pause_queue_cmd;

#[derive(Resource)]

pub struct MandoQueue {
    pub mandos: VecDeque<Vec<Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>>>,
    pub current_mando: Vec<Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>>,
    pub timer: Stopwatch,
}

impl Default for MandoQueue {
    fn default() -> Self {
        let mando: Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)> = Mando { mando_func: test_funco, params: (vec![MandoParam::Int(1)]) };
        MandoQueue { mandos: VecDeque::new(), current_mando: vec![mando], timer: Stopwatch::new() }
    }
}

#[derive(Clone)]
pub enum MandoParam {
    Null,
    Int(u32),
    Float(f32),
    IVec2(IVec2),
    Vector2(Vec2),
    Vector3(Vec3),
    String(String),
    BevyEntity(Entity),
    MeshE(Handle<Mesh>),
    BevyEntities(Vec<Entity>),
}

#[derive(Clone)]
pub struct Mando<F>
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

pub fn fill_mando_queue(
    mut commands: Commands,
    mut mando_queue: ResMut<MandoQueue>,
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

    

    mando_queue.mandos.push_back(mando!(show_portrait_message_cmd; vec![Mp::Null]));

    mando_queue.mandos.push_back(mando!(type_writer_effect_cmd; vec![Mp::String(Message7.to_owned()), Mp::Float(760.0), Mp::Float(9.0)]));

    mando_queue.mandos.push_back(mando!(hide_message_ui_cmd; vec![Mp::Null]));


    mando_queue.mandos.push_back(mando!(show_small_message_ui_cmd; vec![Mp::Null]));

    mando_queue.mandos.push_back(mando!(type_writer_effect_cmd; vec![Mp::String(Message8.to_owned()), Mp::Float(1100.0), Mp::Float(1.0)]));

    mando_queue.mandos.push_back(mando!(hide_message_ui_cmd; vec![Mp::Null]));


    mando_queue.mandos.push_back(mando!(pause_queue_cmd; vec![Mp::Null]));

    commands.holder_mando();

}

pub fn operate_mando_queue (
    mut commands: Commands,
    mut mando_queue: ResMut<MandoQueue>,
    mut set: ParamSet<(
        ResMut<CommandCompleteIndicator>,
        Res<FixedTimesteps>, 
    )>, 
) 
{   
    if set.p0().completed && mando_queue.mandos.len() > 0 {
        let first_mando: Vec<Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>> = mando_queue.mandos.pop_front().unwrap();

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
fn add_mando(mand_func: fn(std::vec::Vec<MandoParam>, &mut World, u128, u128), params: Vec<MandoParam>) -> Vec<Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>> {
    {
        let mut mando_vector: Vec<Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>> = Vec::new();
        let mando: Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)> = Mando { mando_func: mand_func, params: params };
        mando_vector.push(mando);
        mando_vector
    }
}

pub fn mpf(mp: &MandoParam) -> f32 {
    if let MandoParam::Float(a) = mp {
        return *a
    } 
    panic!("This isn't an mpf!");
    // return -999.999
}
pub fn mpiv2(mp: &MandoParam) -> IVec2 {
    if let MandoParam::IVec2(a) = mp {
        return *a
    } 
    panic!("This isn't an mpv3!");
    // return Vec3 { x: -999.999, y: -999.999, z: -999.999 }
}
pub fn mpfv2(mp: &MandoParam) -> Vec2 {
    if let MandoParam::Vector2(a) = mp {
        return *a
    } 
    panic!("This isn't an mpv3!");
    // return Vec3 { x: -999.999, y: -999.999, z: -999.999 }
}
pub fn mpfv3(mp: &MandoParam) -> Vec3 {
    if let MandoParam::Vector3(a) = mp {
        return *a
    } 
    panic!("This isn't an mpv3!");
    // return Vec3 { x: -999.999, y: -999.999, z: -999.999 }
}
pub fn mps(mp: &MandoParam) -> &str {
    if let MandoParam::String(a) = mp {
        return a
    } 
    panic!("This isn't a string!");
}
pub fn mpe(mp: &MandoParam) -> Entity {
    if let MandoParam::BevyEntity(a) = mp {
        return *a
    } 
    panic!("This isn't an mpe!");
    // return Vec3 { x: -999.999, y: -999.999, z: -999.999 }
}
pub fn mpa(mp: &MandoParam) -> Handle<Mesh> {
    if let MandoParam::MeshE(a) = mp {
        return a.clone()
    } 
    panic!("This isn't an mpa!");
    // return Vec3 { x: -999.999, y: -999.999, z: -999.999 }
}