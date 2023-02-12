use bevy::{
    prelude::*, 
    time::{FixedTimestep, Stopwatch}, ecs::system::Command,
};
use std::collections::VecDeque;
use iyes_loopless::prelude::*;
use kayak_ui::{prelude::FontMapping};

use crate::ui::{UIMessageWindow, CurrentText};
use crate::{Person};
use crate::game_commands::{CommandCompleteIndicator, GameCommandsExt };
use crate::components::*;
use crate::messages::*;

#[derive(Clone)]
pub enum MandoParam {
    Int(u32),
    Float(f32),
    Vector2(Vec2),
    Vector3(Vec3),
    String(String),
    BevyEntity(Entity),
    BevyEntities(Vec<Entity>),
}

#[derive(Clone, PartialEq, Copy)]
pub enum MandoType {
    ShowMainMenu,
    HideMainMenu,
    PlayAnimOnce,
    MoveToLoc,
    PauseQueue,
    ShowUIMessage,
    HideUIMessage,
    AffectTypeWriter,
    FillerMando,
    HolderMando,
}

#[derive(Resource)]

pub struct MandoQueue {
    pub mandos: VecDeque<Vec<Mando>>,
    pub currentMando: Vec<Mando>,
    pub timer: Stopwatch,
}

impl Default for MandoQueue {
    fn default() -> Self {
        let mando = Mando { mandoType: MandoType::FillerMando, mandoParams: (vec![MandoParam::Int(1)]) };
        MandoQueue { mandos: VecDeque::new(), currentMando: vec![mando], timer: Stopwatch::new() }
    }
}

#[derive(Clone)]
pub struct Mando {
    pub mandoType: MandoType,
    pub mandoParams: Vec<MandoParam>,
}

pub fn fill_mando_queue(
    mut commands: Commands,
    mut set: ParamSet<(
        ResMut<MandoQueue>,
        Query<(Entity, &Transform), With<Person>>,
        Query<(Entity), With<UIMessageWindow>>,
        Query<(Entity), With<Door>>,
    )>,

) {

    

    let mut vecParams: Vec<Mando> = Vec::new();
    let mut mando = Mando {mandoType: MandoType::FillerMando, mandoParams: vec![]};
    let translation = set.p1().single().1.translation;
    let plyr_entity = set.p1().single().0;
    let ui_entity = set.p2().get_single();
    let door_entity = set.p3().single();
    
    // vecParams = Vec::new();
    // mando = Mando {mandoType: MandoType::ShowMainMenu, mandoParams: vec![]};
    // vecParams.push(mando);
    // set.p0().mandos.push_back(vecParams);
    
    // vecParams = Vec::new();
    // mando = Mando {mandoType: MandoType::HolderMando, mandoParams: vec![]};
    // vecParams.push(mando);
    // set.p0().mandos.push_back(vecParams);
    

    // vecParams = Vec::new();
    // mando = Mando {mandoType: MandoType::HideMainMenu, mandoParams: vec![]};
    // vecParams.push(mando);
    // set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::PlayAnimOnce, mandoParams: vec![
        MandoParam::BevyEntity(door_entity)   
    ]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);
    
    vecParams = Vec::new();
    mando =  Mando{ mandoType: MandoType::MoveToLoc, mandoParams: vec![
        MandoParam::Float(3.50),                             // duration
        MandoParam::Vector3(translation),                   // location
        MandoParam::Vector3(Vec3{x: -3.0, y:-0.25, z: -2.509}),  // destination
        MandoParam::BevyEntity(plyr_entity)                 // entity
    ]}; 
    vecParams.push(mando); 
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::PauseQueue, mandoParams: vec![]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::ShowUIMessage, mandoParams: vec![
        // MandoParam::String(create_message(Message1))//Message1.to_owned()))
    ]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::AffectTypeWriter, mandoParams: vec![MandoParam::String(Message1.to_owned())]};
        // MandoParam::BevyEntity(ui_message_percentage_entity),
    // ]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    // vecParams = Vec::new();
    // mando = Mando {mandoType: MandoType::PauseQueue, mandoParams: vec![]};
    // vecParams.push(mando);
    // set.p0().mandos.push_back(vecParams);
    // info!("Vimrim");

    // vecParams = Vec::new();
    // mando = Mando {mandoType: MandoType::PauseQueue, mandoParams: vec![]};
    // vecParams.push(mando);
    // set.p0().mandos.push_back(vecParams);


    // vecParams = Vec::new();
    // mando = Mando {mandoType: MandoType::ShowUIMessage, mandoParams: vec![
    //     // MandoParam::String(create_message(Message2))
    // ]};
    // vecParams.push(mando);
    // set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::AffectTypeWriter, mandoParams: vec![MandoParam::String(Message2.to_owned())]};
        // MandoParam::BevyEntity(ui_message_percentage_entity),
    // ]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::HideUIMessage, mandoParams: vec![]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);


    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::PauseQueue, mandoParams: vec![]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);
    println!("fillerup");

}

pub fn operate_mando_queue (
    mut commands: Commands,
    mut set: ParamSet<(
        ResMut<MandoQueue>,
        ResMut<CommandCompleteIndicator>,
        Res<FixedTimesteps>,
    )>,
    mut set2: ParamSet<(
        ResMut<FontMapping>,
        Res<AssetServer>,
        Query<Entity, With<UIMessageWindow>>,
    )>,
) 
{
    if set.p1().completed && set.p0().mandos.len() > 0 {
        let firstMando = set.p0().mandos.pop_front().unwrap();

        set.p0().currentMando = firstMando;

        set.p1().completed = false;
        set.p0().timer.reset();
        
        // for debugging
        for mando in &set.p0().currentMando {
        match mando.mandoType {
            MandoType::MoveToLoc => {
                println!("shmoop1");
                info!("shmoop1");
            },
            MandoType::PauseQueue => {
                println!("shmoop2");
                info!("shmoop2");
            },
            MandoType::HideUIMessage => {
                println!("shmoop3");
                info!("shmoop3");
            },
            MandoType::ShowUIMessage => {
                println!("shmoop4");
                info!("shmoop4");
            },
            MandoType::AffectTypeWriter => {
                println!("shmoop5");
                info!("shmoop5");
            }
            MandoType::FillerMando => {
                println!("shmoop6");
                info!("shmoop6");
            }
            MandoType::HolderMando => {
                println!("shmoop7");
                info!("shmoop7");
            }
            MandoType::ShowMainMenu => {
                println!("shmoop8");
                info!("shmoop8");
            }
            MandoType::HideMainMenu => {
                println!("shmoop9");
                info!("shmoop9");
            }
            MandoType::PlayAnimOnce => {
                println!("shmoop10");
                info!("shmoop10");
            }
            }
        }
        // for debugging
    }
    if set.p1().completed && set.p0().mandos.len() == 0 {

    } else {
    let step = set.p2().get_current().unwrap().step;
    set.p0().timer.tick(step);
    let elapsedTime = set.p0().timer.elapsed().as_millis();
    let mut completeCommand = false;
    
    for mando in &set.p0().currentMando {
        match mando.mandoType {
            MandoType::MoveToLoc => {
                commands.move_to_loc_3d(            
                    step.as_millis(),
                    elapsedTime, 
                    mpf(&mando.mandoParams[0]), // duration
                    mpv3(&mando.mandoParams[1]), // location
                    mpv3(&mando.mandoParams[2]), // destination 
                    mpe(&mando.mandoParams[3])); // entity 
            },
            MandoType::PauseQueue => {
                commands.pause_queue();
            },
            MandoType::HideUIMessage => {
                commands.despawn_message_ui();

            },
            MandoType::ShowUIMessage => {
                commands.spawn_message_ui();
            },
            MandoType::AffectTypeWriter => {

                commands.affect_typewriter(elapsedTime, mps(&mando.mandoParams[0]));
            },
            MandoType::FillerMando => {
                commands.filler_mando();
            }
            MandoType::ShowMainMenu => {
                commands.show_main_menu();
            }
            MandoType::HideMainMenu => {
                commands.hide_main_menu();
            }
            MandoType::PlayAnimOnce => {
                commands.setup_scene1(
                    mpe(&mando.mandoParams[0])); // entity
            }
            MandoType::HolderMando => {
                commands.holder_mando();
            }
            
        }
    } 
    if completeCommand {
        set.p1().completed = true;
    }
    }
    

}

pub fn mpf(mp: &MandoParam) -> f32 {
    if let MandoParam::Float(a) = mp {
        return *a
    } 
    panic!("This isn't an mpf!");
    // return -999.999
}
pub fn mpv3(mp: &MandoParam) -> Vec3 {
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