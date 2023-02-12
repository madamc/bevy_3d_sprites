use std::thread::current;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::utils::Duration;
use bevy_inspector_egui::egui::Key;
use kayak_ui::{prelude::FontMapping};
use kayak_ui::prelude::{widgets::*, *};
use bevy::ecs::{
    // component::Component,
    // entity::Entity,
    system::{CommandQueue, SystemState},
    // world::World,
};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy_sprite3d::*;
use rand::{prelude::SliceRandom, Rng};
use iyes_loopless::prelude::*;

use crate::components::Animation;
use crate::{ImageAssets, GameState};
use crate::mandoqueue::{Mando, MandoQueue};
use crate::ui::{UIMessageWindow, ui_message_current_percent_render, CurrentText, CurrentTextState, CurrentTextBundle, MainMenuWidget, MenuButton, MenuButtonBundle, menu_button_render, ButtonState2, create_message, message_to_str};


#[derive(Debug, Clone, Default)]
pub struct CommandCompletedEvent;

#[derive(Debug, Clone, Default)]
pub struct MenuStartEvent;

#[derive(Resource)]
pub struct CommandCompleteIndicator {
    pub completed: bool
}

impl Default for CommandCompleteIndicator {
    fn default() -> Self {
        CommandCompleteIndicator { completed: false }
    }
}

struct AddToMandoQueueCommand{
    mandos: Vec<Mando>,
}

impl bevy::ecs::system::Command for AddToMandoQueueCommand {
    fn write(self, world: &mut World) {
        let mut vecParams: Vec<Mando> = Vec::new();
        let mut mq = world.get_resource_mut::<MandoQueue>().unwrap();
        mq.mandos.push_back(vecParams);
    }
}

struct PlayAnimOnceCommand{
    entity: Entity
}

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const ATLAS_COLUMNS: usize = 8;
const TIME_STEP: f32 = 10.0;
const PLYR_SPEED: f32 = 2.0;

impl bevy::ecs::system::Command for PlayAnimOnceCommand {
    fn write(self, world: &mut World) {
    
    //     images: Res<ImageAssets>,
    // mut sprite_params: Sprite3dParams,

    let mut state = SystemState::<(Commands, ParamSet<(
        Res<FixedTimesteps>,
        // ResMut<FontMapping>,
        Query<(&mut Animation, &mut AtlasSprite3dComponent)>
        )>)>::new(world);
    let (mut commands, mut set) = state.get_mut(world);

        // time: Res<FixedTimesteps>,
        
        let step = set.p0().get_current().unwrap().step;
        let mut query = set.p1();
        let (mut animation, mut sprite) = query.get_mut(self.entity).unwrap();

            if animation.current + 1 < animation.frames.len() {
                animation.timer.tick(step);
                if animation.timer.just_finished() {
                    sprite.index = animation.frames[animation.current];
                    animation.current += 1;
                }
            } else {
                let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
                cc.completed = true;
            }

        
        // for (mut animation, mut sprite) in set.p1().iter_mut() {
        //     if animation.current + 1 < animation.frames.len() {
        //         animation.timer.tick(step);
        //         if animation.timer.just_finished() {
        //             sprite.index = animation.frames[animation.current];
        //             animation.current += 1;
        //         }
        //     }
        // }
    }
}

struct MainMenuCommand{
}

impl bevy::ecs::system::Command for MainMenuCommand {
    fn write(self, world: &mut World) {
        
        // A method of gettin the commands
        // let mut queue = CommandQueue::default();
        // let mut commands = Commands::new(&mut queue, &world);
        
        // Another method of getting the commands and this also allows for getting queries
        let mut state = SystemState::<(Commands, ParamSet<(
            Res<ImageAssets>,
            ResMut<FontMapping>,
            )>)>::new(world);
        let (mut commands, mut set) = state.get_mut(world);

    //     let panel = set.p0().panel.clone();
    //     let panel_btn = set.p0().panel_btn.clone();
    //     // let panel_btn_hov = set.p0().panel_btn_hov.clone();
    //     // let panel_btn_clk = set.p0().panel_btn_clk.clone();
    //     let font = set.p0().kfont.clone();
    //     let mut fontMapper = set.p1();//world.get_resource_mut::<FontMapping>().unwrap();
    //     fontMapper.set_default(font);

    // let mut widget_context = KayakRootContext::new();
    // widget_context.add_plugin(KayakWidgetsContextPlugin);
    // widget_context.add_widget_data::<MenuButton, ButtonState2>();
    // widget_context.add_widget_system(
    //     MenuButton::default().get_name(),
    //     widget_update::<MenuButton, ButtonState2>,
    //     menu_button_render,
    // );

    // let handle_click_close = OnEvent::new(
    //     move |In((event_dispatcher_context, _, event, _entity)): In<(
    //         EventDispatcherContext,
    //         WidgetState,
    //         Event,
    //         Entity,
    //     )>,
    //           mut exit: EventWriter<AppExit>| {
    //         match event.event_type {
    //             EventType::Click(..) => {
    //                 exit.send(AppExit);
    //             }
    //             _ => {}
    //         }
    //         (event_dispatcher_context, event)
    //     },
    // );
    // let handle_click_start = OnEvent::new(
    //     move |In((event_dispatcher_context, _, event, _entity)): In<(
    //         EventDispatcherContext,
    //         WidgetState,
    //         Event,
    //         Entity,
    //     )>,
    //           mut complete: EventWriter<CommandCompletedEvent>| {
    //         match event.event_type {
    //             EventType::Click(..) => {
    //                 complete.send(CommandCompletedEvent);
    //             }
    //             _ => {}
    //         }
    //         (event_dispatcher_context, event)
    //     },
    // );

    // let parent_id = None;
    // rsx! {
    //     <KayakAppBundle>
    //         <NinePatchBundle
    //             nine_patch={NinePatch {
    //                 handle: panel,
    //                 border: Edge::all(25.0),
    //             }}
    //             styles={KStyle {
    //                 width: Units::Pixels(450.0).into(),
    //                 height: Units::Pixels(512.0).into(),
    //                 left: Units::Stretch(0.5).into(),
    //                 right: Units::Stretch(0.5).into(),
    //                 top: Units::Stretch(1.0).into(),
    //                 bottom: Units::Stretch(1.0).into(),
    //                 padding: Edge::new(
    //                     Units::Pixels(20.0),
    //                     Units::Pixels(20.0),
    //                     Units::Pixels(50.0),
    //                     Units::Pixels(20.0),
    //                 ).into(),
    //                 ..KStyle::default()
    //             }}
    //         >
    //             <TextWidgetBundle
    //             styles={KStyle {
    //                 // top: Units::Stretch(1.0).into(),
    //                 bottom: Units::Stretch(1.0).into(),
    //                 ..Default::default()
    //             }}
    //             text={TextProps {
    //                 alignment: Alignment::Middle,
    //                 content: "Happy Birthday, Mom".to_owned() ,
    //                 size: 24.0,
    //                 ..Default::default()
    //             }}
    //             />
    //             <MenuButtonBundle button={MenuButton { text: "Start".into() }} 
    //                 on_event={handle_click_start}
    //             />
    //             <MenuButtonBundle
    //                 button={MenuButton { text: "Quit".into() }}
    //                 on_event={handle_click_close}
    //             />
    //         </NinePatchBundle>
    //     </KayakAppBundle>
    // };

    // commands.spawn((MainMenuWidget, UICameraBundle::new(widget_context)));
    // state.apply(world);
    // let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    // cc.completed = true;

    }
}

struct SetupSceneCommand{
    mandos: Vec<Mando>,
}

impl bevy::ecs::system::Command for SetupSceneCommand {
    fn write(self, world: &mut World) {
        let mut vecParams: Vec<Mando> = Vec::new();
        let mut mq = world.get_resource_mut::<MandoQueue>().unwrap();
        mq.mandos.push_back(vecParams);
    }
}

struct StringCommmand(String);
impl bevy::ecs::system::Command for StringCommmand {
    fn write(self, world: &mut World) {
        println!("{}", self.0)
    }
}

struct MoveToLoc3DCommand {
    delta: u128,
    elapsedTime: u128,
    duration: f32,
    location: Vec3,
    destination: Vec3,
    entity: Entity,
}

impl bevy::ecs::system::Command for MoveToLoc3DCommand {
    fn write(self, world: &mut World) {
        
        let elapsedTime_sec= (self.elapsedTime as f32) / 1000.0;

        let percentage: f32 = if (elapsedTime_sec as f32 > self.duration) {1.0} else {elapsedTime_sec / self.duration};
       
        let mut ynynTF = world.get_mut::<Transform>(self.entity).unwrap();
        let newPos: Vec3 = self.location.lerp(self.destination, percentage);

        ynynTF.translation = newPos;
        
        if (percentage >= 1.0) {
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
        } 
    }
}

struct ShowMessageUICommand {

}

impl bevy::ecs::system::Command for ShowMessageUICommand {
    fn write(self, world: &mut World) {
        
        // A method of gettin the commands
        // let mut queue = CommandQueue::default();
        // let mut commands = Commands::new(&mut queue, &world);
        
        // Another method of getting the commands and this also allows for getting queries
        let mut state = SystemState::<(Commands, ParamSet<(
            Res<ImageAssets>,
            ResMut<FontMapping>,
            )>)>::new(world);
        let (mut commands, mut set) = state.get_mut(world);
        let panel = set.p0().panel.clone();
        let font = set.p0().kfont.clone();
        let mut fontMapper = set.p1();
        fontMapper.set_default(font);

        let mut widget_context = KayakRootContext::new();
        widget_context.add_plugin(KayakWidgetsContextPlugin);
        let parent_id = None;
        widget_context.add_widget_data::<CurrentText, CurrentTextState>();
        widget_context.add_widget_system(
            CurrentText::default().get_name(),
            widget_update::<CurrentText, CurrentTextState>,
            ui_message_current_percent_render,
        );
    
        rsx! {
            <KayakAppBundle>
                <NinePatchBundle
                    nine_patch={NinePatch {
                        handle: panel,
                        border: Edge::all(20.0),
                    }}
                    styles={KStyle {
                        width: StyleProp::Value(Units::Percentage(80.0)),
                        left: StyleProp::Value(Units::Percentage(10.0)),
                        height: StyleProp::Value(Units::Percentage(35.0)),
                        ..KStyle::default()
                    }}
                >
                    <CurrentTextBundle />
                </NinePatchBundle>
            </KayakAppBundle>
            
        };
    
        commands.spawn((UIMessageWindow,UICameraBundle::new(widget_context)));
        state.apply(world);

        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
    }
}

struct HideMainMenuCommand {

}

impl bevy::ecs::system::Command for HideMainMenuCommand {
    fn write(self, world: &mut World) {
        
        let mut state = SystemState::<(Commands, Query<Entity, With<MainMenuWidget>>)>::new(world);
        let (mut cmds, query) = state.get_mut(world);
        println!("items {}", query.iter().len());
        if query.iter().len() > 0 {
            println!("emptying the menu items");
            for item in query.iter() {
                println!("dump");
                cmds.entity(item).despawn_recursive();
                cmds.entity(item).despawn();
                cmds.entity(item).despawn_descendants();
            }
        }
        else {
            println!("all gone!");
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
        }

        state.apply(world);
    }
}


struct HideMessageUICommand {

}

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

struct PauseQueueCommand {

}

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

struct AffectTypeWriterCommand {
    elapsed_time: u128,
    message: String,
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

struct FillerMandoCommand {

}

impl bevy::ecs::system::Command for FillerMandoCommand {
    fn write(self, world: &mut World) {
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
    }
}

struct HolderMandoCommand; 

impl bevy::ecs::system::Command for HolderMandoCommand {
    fn write(self, world: &mut World) {
        //depends on the Event 
    }
}

pub trait GameCommandsExt {
    fn add_to_mando_queue(&mut self, params: Vec<Mando>);
    fn print_message(&mut self, msg: String);
    fn show_main_menu(&mut self);
    fn hide_main_menu(&mut self);
    fn setup_scene1(&mut self, entity: Entity);
    fn move_to_loc_3d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec3, destination: Vec3, entity: Entity ); 
    fn spawn_message_ui(&mut self);
    fn despawn_message_ui(&mut self);
    fn pause_queue(&mut self);
    fn affect_typewriter(&mut self, elapsed_time: u128, message: &str);
    fn filler_mando(&mut self);
    fn holder_mando(&mut self);
    // fn get_currrent_mando() -> Mando;
}

impl<'w, 's> GameCommandsExt for Commands<'w, 's> {

    fn add_to_mando_queue(&mut self, mandos: Vec<Mando>) {
        self.add(AddToMandoQueueCommand {mandos: mandos});
    }
    fn print_message(&mut self, msg: String) {
        self.add(StringCommmand(msg));
    }

    fn show_main_menu(&mut self) {
        self.add(MainMenuCommand {});
    }

    fn hide_main_menu(&mut self) {
        self.add(HideMainMenuCommand {});
    }

    fn setup_scene1(&mut self, entity: Entity) {
        self.add(PlayAnimOnceCommand {entity: entity});
    }

    fn move_to_loc_3d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec3, destination: Vec3, entity: Entity ) {// mParams: Vec<MandoParam>) {
        self.add(MoveToLoc3DCommand { delta: delta, elapsedTime: elapsedTime, duration: duration, location: location, destination: destination, entity: entity });
    }

    fn spawn_message_ui(&mut self) {
        self.add(ShowMessageUICommand {});
    }
    fn despawn_message_ui(&mut self) {
        self.add(HideMessageUICommand {});
    }

    fn pause_queue(&mut self) {
        self.add(PauseQueueCommand {});
    }

    fn affect_typewriter(&mut self, elapsed_time: u128, message: &str) {
        self.add(AffectTypeWriterCommand {elapsed_time: elapsed_time, message: message.to_owned()})
    }

    fn filler_mando(&mut self) {
        self.add(FillerMandoCommand {});
    }
    fn holder_mando(&mut self) {
        self.add(HolderMandoCommand);
    }
    // fn get_currrent_mando() -> Mando {
    //     // self.add()
    // }
}

