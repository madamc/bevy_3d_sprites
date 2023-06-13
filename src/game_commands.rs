use bevy::prelude::*;

pub mod show_message;
pub mod show_small_message;
pub mod show_portrait_message;
pub mod hide_message_ui;
pub mod move_to_loc_3d;
pub mod move_to_loc_2d;
pub mod move_to_loc_2d_i;
pub mod play_anim_once;
pub mod change_anim;
pub mod ynyn_anim_state_chgs;
pub mod type_writer_effect;
pub mod pause_queue;
pub mod mando_queue;

use crate::components::CommandCompleteIndicator;
// use crate::mandoqueue::{Mando_old, MandoQueue_old};
// use crate::mando_queue::{Mando, MandoQueue, MandoParam};


use self::mando_queue::*;
use self::move_to_loc_3d::MoveToLoc3DCommand;
use self::move_to_loc_2d::MoveToLoc2DCommand;
use self::play_anim_once::PlayAnimOnceCommand;
use self::pause_queue::{pause_queue_cmd};
use self::ynyn_anim_state_chgs::{YNYNIdleLCMD, YNYNWalkLCMD, YNYNWalkRCMD, YNYNIdleRCMD};

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const ATLAS_COLUMNS: usize = 8;
const TIME_STEP: f32 = 10.0;
const PLYR_SPEED: f32 = 2.0;

//TODO: Integrate this into the mandoqueue adding, or make a macro instead.
// struct AddToMandoQueueCommand{
//     mandos: Vec<Mando_old>,
// }

// impl bevy::ecs::system::Command for AddToMandoQueueCommand {
//     fn write(self, world: &mut World) {
//         let mut vecParams: Vec<Mando_old> = Vec::new();
//         let mut mq = world.get_resource_mut::<MandoQueue_old>().unwrap();
//         mq.mandos.push_back(vecParams);
//     }
// }

struct StringCommmand(String);
impl bevy::ecs::system::Command for StringCommmand {
    fn write(self, world: &mut World) {
        println!("{}", self.0)
    }
}

struct FillerMandoCommand {
    mando: Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>, 
    delta: u128, 
    elapsed_time: u128, 
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl bevy::ecs::system::Command for FillerMandoCommand {
    fn write(self, world: &mut World) {
        
        let mut mq = world.get_resource_mut::<MandoQueue>().unwrap();
        //  println!("Reibe");
        // println!("mq size {}", mq.currentMando.);
        // let val: fn(std::vec::Vec<MandoParam>, &mut World, u128, u128, &str) = mq.currentMando[0].mandoFunc; //self.mando.mandoFunc;
        // print_type_of(&val);
        // val.type_name();
        // println!("Squibbo {}", val.to_owned());
        // let nexval = val.to_owned();
        // nexval(self.mando.params, world, self.delta, self.elapsedTime);
        // let nuthval = nexval.clone();
        // nuthval(self.mando.params, world, self.delta, self.elapsedTime);
        // let mut new_mando: Mando2<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128, &str)> = Mando2 { mandoFunc: pause_queue_cmd, params: vec![MandoParam::Null]};
        // mq.currentMando.push(new_mando);
        // let val: fn(std::vec::Vec<MandoParam>, &mut World, u128, u128, &str) = mq.currentMando[0].mandoFunc; //self.mando.mandoFunc;
        // let nuthmand = new_mando.clone();
        // (new_mando.mandoFunc)(self.mando.params, world, self.delta, self.elapsedTime, "Haldo");
        // val(self.mando.params, world, self.delta, self.elapsedTime, "Haldo");
        // let neval = val(self.mando.params, world, self.delta, self.elapsedTime);
        (self.mando.mando_func)(self.mando.params, world, self.delta, self.elapsed_time);
        // (self.mando.mandoFunc)(self.mando.params, world, self.delta, self.elapsedTime, "Crildo");
        // let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        // cc.completed = true;
    }
}

struct HolderMandoCommand; 

impl bevy::ecs::system::Command for HolderMandoCommand {
    fn write(self, world: &mut World) {
        //depends on the Event 
                let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
    }
}

pub trait GameCommandsExt {
    // fn add_to_mando_queue(&mut self, params: Vec<Mando_old>);
    fn print_message(&mut self, msg: String);
    fn play_anim_once(&mut self, entity: Entity);
    fn ynyn_walk_l(&mut self, entity: Entity);
    fn ynyn_Idle_l(&mut self, entity: Entity);
    fn ynyn_walk_r(&mut self, entity: Entity);
    fn ynyn_Idle_r(&mut self, entity: Entity);
    fn change_anim(&mut self);
    fn move_to_loc_3d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec3, destination: Vec3, entity: Entity ); 
    fn move_to_loc_2d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec2, destination: Vec2, entity: Entity ); 
    fn move_to_loc_2d_i(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: IVec2, destination: IVec2, entity: Entity ); 
    fn move_to_loc_2d_i2(&mut self, params: Vec<MandoParam>); 
    fn spawn_message_ui(&mut self);
    fn despawn_message_ui(&mut self);
    fn pause_queue(&mut self);
    fn affect_typewriter(&mut self, elapsed_time: u128, message: &str);
    fn filler_mando(&mut self, mando: Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>, delta: u128, elapsedTime: u128);
    fn holder_mando(&mut self);
    // fn get_currrent_mando() -> Mando;
}

impl<'w, 's> GameCommandsExt for Commands<'w, 's> {

    // fn add_to_mando_queue(&mut self, mandos: Vec<Mando_old>) {
    //     self.add(AddToMandoQueueCommand {mandos: mandos});
    // }
    fn print_message(&mut self, msg: String) {
        self.add(StringCommmand(msg));
    }

    fn play_anim_once(&mut self, entity: Entity) {
        self.add(PlayAnimOnceCommand {entity: entity});
    }

    fn ynyn_walk_l(&mut self, entity: Entity) {
        self.add(YNYNWalkLCMD {entity: entity});
    }

    fn ynyn_Idle_l(&mut self, entity: Entity) {
        self.add(YNYNIdleLCMD {entity: entity});
    }

    fn ynyn_walk_r(&mut self, entity: Entity) {
        self.add(YNYNWalkRCMD {entity: entity});
    }

    fn ynyn_Idle_r(&mut self, entity: Entity) {
        self.add(YNYNIdleRCMD {entity: entity});
    }

    fn change_anim(&mut self) {

    }

    fn move_to_loc_3d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec3, destination: Vec3, entity: Entity ) {// mParams: Vec<MandoParam>) {
        self.add(MoveToLoc3DCommand { delta: delta, elapsedTime: elapsedTime, duration: duration, location: location, destination: destination, entity: entity });
    }

    fn move_to_loc_2d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec2, destination: Vec2, entity: Entity ) {// mParams: Vec<MandoParam>) {
        self.add(MoveToLoc2DCommand { delta: delta, elapsedTime: elapsedTime, duration: duration, location: location, destination: destination, entity: entity });
    }

    fn move_to_loc_2d_i(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: IVec2, destination: IVec2, entity: Entity ) {// mParams: Vec<MandoParam>) {
        
    }

    fn move_to_loc_2d_i2(&mut self, params: Vec<MandoParam>) {
        // self.add(MoveToLoc2DIv2Command {                     step.as_millis(),
        //     elapsedTime, 
        //     params[0], // duration
        //     params[1], // location
        //     params[2], // destination 
        //     params[3]); // entity  });
    }

    fn spawn_message_ui(&mut self) {
    }
    fn despawn_message_ui(&mut self) {

    }

    fn pause_queue(&mut self) {

    }

    fn affect_typewriter(&mut self, elapsed_time: u128, message: &str) {
        // self.add(AffectTypeWriterCommand {elapsed_time: elapsed_time, message: message.to_owned()})
    }

    fn filler_mando(&mut self, mando: Mando<fn(std::vec::Vec<MandoParam>, &mut World, u128, u128)>, delta: u128, elapsedTime: u128) {
        self.add(FillerMandoCommand {mando: mando, delta: delta, elapsed_time: elapsedTime});
    }
    fn holder_mando(&mut self) {
        self.add(HolderMandoCommand);
    }
    // fn get_currrent_mando() -> Mando {
    //     // self.add()
    // }
}