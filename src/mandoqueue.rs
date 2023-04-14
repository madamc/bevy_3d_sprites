use bevy::{
    prelude::*, 
    time::{Stopwatch}
};
use seldom_pixel::prelude::PxPosition;
use std::collections::VecDeque;
use iyes_loopless::prelude::*;
use kayak_ui::{prelude::FontMapping};

use crate::{ui::{UIMessageWindow, CurrentText}, YNYN_Y};
use crate::{Person};
use crate::game_commands::{ GameCommandsExt };
use crate::components::*;
use crate::messages::*;

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
    BevyEntities(Vec<Entity>),
}

#[derive(Clone, PartialEq, Copy)]
pub enum MandoType {
    PlayAnimOnce,
    YNYNWalkL,
    YNYNIdleL,
    YNYNWalkR,
    YNYNIdleR,
    ChangeAnim,
    MoveToLoc3D,
    MoveToLoc2D,
    MoveToLoc2DIv2,
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
        let mando = Mando { mandoType: MandoType::HolderMando, mandoParams: (vec![MandoParam::Int(1)]) };
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
        Query<(Entity, &PxPosition), With<Person>>,
        Query<(Entity), With<UIMessageWindow>>,
        Query<(Entity), With<Door>>,
    )>,

) {

    

    let mut vecParams: Vec<Mando> = Vec::new();
    let mut mando = Mando {mandoType: MandoType::FillerMando, mandoParams: vec![]};
    let plyr_location = set.p1().single().1.0;
    let plyr_entity = set.p1().single().0;
    // let ui_entity = set.p2().get_single();
    // let door_entity = set.p3().single();
    
    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::PauseQueue, mandoParams: vec![]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);


    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::YNYNWalkL, mandoParams: vec![
        MandoParam::BevyEntity(plyr_entity)   
    ]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    // vecParams = Vec::new();
    // mando =  Mando{ mandoType: MandoType::MoveToLoc, mandoParams: vec![
    //     MandoParam::Float(3.50),                             // duration
    //     MandoParam::Vector3(translation),                   // location
    //     MandoParam::Vector3(Vec3{x: -3.0, y:-0.25, z: -2.509}),  // destination
    //     MandoParam::BevyEntity(plyr_entity)                 // entity
    // ]}; 
    // vecParams.push(mando); 
    // set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando =  Mando{ mandoType: MandoType::MoveToLoc2DIv2, mandoParams: vec![
        MandoParam::Float(5.),                             // duration
        MandoParam::IVec2(plyr_location),                   // location
        MandoParam::IVec2(IVec2 {x: 30, y:YNYN_Y}),  // destination
        MandoParam::BevyEntity(plyr_entity)                 // entity
    ]}; 
    vecParams.push(mando); 
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::YNYNIdleL, mandoParams: vec![
        MandoParam::BevyEntity(plyr_entity)   
    ]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::PauseQueue, mandoParams: vec![]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::ShowUIMessage, mandoParams: vec![]};
    vecParams.push(mando);
    set.p0().mandos.push_back(vecParams);

    vecParams = Vec::new();
    mando = Mando {mandoType: MandoType::AffectTypeWriter, mandoParams: vec![
        MandoParam::String(Message1.to_owned())]};
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
    mando = Mando {mandoType: MandoType::AffectTypeWriter, mandoParams: vec![
        MandoParam::String(Message2.to_owned())]};
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
) 
{
    // let snep = set.p2();
    if set.p1().completed && set.p0().mandos.len() > 0 {
        let firstMando = set.p0().mandos.pop_front().unwrap();
        set.p0().currentMando = firstMando;

        set.p1().completed = false;
        set.p0().timer.reset();
        
        // for debugging
        for mando in &set.p0().currentMando {
        match mando.mandoType {
            MandoType::MoveToLoc3D => {
                println!("shmoop1");
                info!("shmoop1");
            },
            MandoType::MoveToLoc2D => {
                println!("shmoop9");
                info!("shmoop9");
            },
            MandoType::MoveToLoc2DIv2 => {
                println!("shmoop10");
                info!("shmoop10");
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
            MandoType::PlayAnimOnce => {
                println!("shmoop8");
                info!("shmoop8");
            }
            MandoType::YNYNIdleL => {
                println!("shmoop12");
                info!("shmoop12");
            }
            MandoType::YNYNWalkL => {
                println!("shmoop13");
                info!("shmoop13");
            }
            MandoType::YNYNIdleR => {
                println!("shmoop14");
                info!("shmoop14");
            }
            MandoType::YNYNWalkR => {
                println!("shmoop15");
                info!("shmoop15");
            }
            MandoType::ChangeAnim => {
                println!("shmoop11");
                info!("shmoop11");
            }
            }
        }
        // for debugging
    }
    if set.p1().completed && set.p0().mandos.len() == 0 {

    } else {
    // let ptes = set.p2()
    let step = set.p2().get_current().unwrap().step;
    set.p0().timer.tick(step);
    let elapsedTime = set.p0().timer.elapsed().as_millis();
    let mut completeCommand = false;
    for mando in &set.p0().currentMando {
        match mando.mandoType {
            MandoType::MoveToLoc3D => {
                commands.move_to_loc_3d(            
                    step.as_millis(),
                    elapsedTime, 
                    mpf(&mando.mandoParams[0]), // duration
                    mpfv3(&mando.mandoParams[1]), // location
                    mpfv3(&mando.mandoParams[2]), // destination 
                    mpe(&mando.mandoParams[3])); // entity 
            },
            MandoType::MoveToLoc2D => {
                commands.move_to_loc_3d(            
                    step.as_millis(),
                    elapsedTime, 
                    mpf(&mando.mandoParams[0]), // duration
                    mpfv3(&mando.mandoParams[1]), // location
                    mpfv3(&mando.mandoParams[2]), // destination 
                    mpe(&mando.mandoParams[3])); // entity 
            },
            MandoType::MoveToLoc2DIv2 => {
                commands.move_to_loc_2d_i(            
                    step.as_millis(),
                    elapsedTime, 
                    mpf(&mando.mandoParams[0]), // duration
                    mpiv2(&mando.mandoParams[1]), // location
                    mpiv2(&mando.mandoParams[2]), // destination 
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
                // commands.filler_mando();
            }
            MandoType::PlayAnimOnce => {
                commands.play_anim_once(
                    mpe(&mando.mandoParams[0])); // entity
            }
            MandoType::YNYNIdleL => {
                commands.ynyn_Idle_l(
                    mpe(&mando.mandoParams[0])); // entity
            }
            MandoType::YNYNWalkL => {
                commands.ynyn_walk_l(
                    mpe(&mando.mandoParams[0])); // entity
            }
            MandoType::YNYNIdleR => {
                commands.ynyn_walk_r(
                    mpe(&mando.mandoParams[0])); // entity
            }
            MandoType::YNYNWalkR => {
                commands.ynyn_walk_r(
                    mpe(&mando.mandoParams[0])); // entity
            }
            MandoType::ChangeAnim => {
                commands.change_anim();
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