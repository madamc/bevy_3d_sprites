use bevy::{
    prelude::*, 
    time::{FixedTimestep, Stopwatch}, ecs::system::Command, app::AppExit,
};
use bevy_asset_loader::prelude::*;
use bevy::utils::Duration;
// use syn::token::At;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_loopless::prelude::*;
use kayak_ui::prelude::{widgets::*, *};
use kayak_ui::{prelude::FontMapping};
use kayak_font::KayakFont;
use seldom_pixel::prelude::*;
use seldom_state::prelude::*;

// mod macros_help;
mod macros;
mod mq2;
mod messages;
mod mandoqueue;
mod seldom_pixel_prep;
// mod move_to_loc_2DIv2;
mod game_commands;
mod ui;
mod audio;
mod type_writer;
mod components;

use type_writer::*;
use ui::*;
use audio::GameAudioPlugin;
use mandoqueue::*;
use mandoqueue::fill_mando_queue;
use mandoqueue::operate_mando_queue;
use game_commands::*;
use components::*;

use crate::{seldom_pixel_prep::*, mq2::{fill_mando_queue2, operate_mando_queue2, MandoQueue2}};

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const ATLAS_COLUMNS: usize = 8;
const TIME_STEP: f32 = 10.0;
const PLYR_SPEED: i32 = 2;
const YNYN_Y: i32 = 4;


#[derive(AssetCollection, Resource)]
pub struct ImageAssets { 
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., 
            columns = 8, rows = 7, padding_x = 0., padding_y = 0.))]
    #[asset(path = "art/momcard1.png")]
    tileset: Handle<TextureAtlas>,
    #[asset(path = "art/apartment.png")]
    sprite: Handle<Image>,
    #[asset(path = "art/mom-Sheet.png")]
    mom_spr: Handle<Image>,
    #[asset(path = "art/panel-bb.png")]
    panel: Handle<Image>,
    #[asset(path = "art/panel--button-bb.png")]
    panel_btn: Handle<Image>,
    #[asset(path = "art/panel--button-bb-hover.png")]
    panel_btn_hov: Handle<Image>,
    #[asset(path = "art/panel--button-bb-clicked.png")]
    panel_btn_clk: Handle<Image>,
    #[asset(path = "ui/pcsenior.kttf")]
    kfont: Handle<KayakFont>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(TypeWriterTextPlugin)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_plugin(PxPlugin::<Seld_layer>::new(
            UVec2::new(256, 144),
            "palette/palette_4.png".into(),
        ))
        .add_plugin(StateMachinePlugin)
        .add_plugin(TriggerPlugin::<YNYNIdleLTrigger>::default())
        .add_plugin(TriggerPlugin::<YNYNWalkRTrigger>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<MandoQueue>()
        .init_resource::<MandoQueue2>()
        .init_resource::<CommandCompleteIndicator>()
        .add_event::<MenuStartEvent>()
        .add_event::<CommandCompletedEvent>()
        .add_loopless_state(GameState::Loading)
        .add_loading_state(LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::MainMenu)
            .with_collection::<ImageAssets>())
        .add_fixed_timestep(
            Duration::from_millis(TIME_STEP as u64),
            // give it a label
            "my_fixed_update",
        )
        // .add_system(move_plyr)
        .add_enter_system(GameState::MainMenu, main_menu.label("main_menu"))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(menu_start)
                .into()
        )
        .add_enter_system(GameState::Staging, spawn_sprites.label("spawn_sprites").after("setup"))
        .add_enter_system(GameState::Ready, fill_mando_queue2.label("fill_mando_queue"))
        // .add_enter_system(GameState::Ready, fill_mando_queue2.label("fill_mando_queue2"))
        // .add_enter_system(GameState::Ready, operate_mando_queue2.label("fill_mando_queuewsx2"))
        // .add_fixed_timestep_system("my_fixed_update", 0, )
        .add_fixed_timestep_system("my_fixed_update", 0, operate_mando_queue2.run_in_state(GameState::Ready).after("fill_mando_queue"))
        // .add_fixed_timestep_system("my_fixed_update", 0, animate_sprites.run_in_state(GameState::Ready))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Ready)
                .with_system(check_completed_mando)

                .into()
        )
        .run();
    println!("argo");

}

fn menu_start (
    mut commands: Commands,
    mut ev_start: EventReader<MenuStartEvent>,
    query: Query<Entity, With<MainMenuWidget>>,
    // mut cc: ResMut<CommandCompleteIndicator>,
) {
    let result = query.get_single();
    for ev in ev_start.iter() {
        if let Ok(entity) = result {
            commands.entity(entity).despawn_recursive();
            commands.entity(entity).despawn();
            commands.entity(entity).despawn_descendants();
            commands.insert_resource(NextState(GameState::Staging));
        }
    }
}


fn check_completed_mando(
    mut ev_start: EventReader<CommandCompletedEvent>,
    mut cc: ResMut<CommandCompleteIndicator>,
) {
    for ev in ev_start.iter() {
        cc.completed = true;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    println!("Gapho");
}

fn main_menu(
    mut commands: Commands, 
    mut set: ParamSet<(
        Res<ImageAssets>,
        ResMut<FontMapping>,
        )>,
) {
    
    let panel = set.p0().panel.clone();
        let panel_btn = set.p0().panel_btn.clone();
        let font = set.p0().kfont.clone();
        let mut fontMapper = set.p1();
        fontMapper.set_default(font);
        
    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<MenuButton, ButtonState2>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState2>,
        menu_button_render,
    );

    let handle_click_close = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut exit: EventWriter<AppExit>| {
            match event.event_type {
                EventType::Click(..) => {
                    exit.send(AppExit);
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );
    let handle_click_start = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut complete: EventWriter<MenuStartEvent>| {
            match event.event_type {
                EventType::Click(..) => {
                    complete.send(MenuStartEvent);
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );
    
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: panel,
                    border: Edge::all(25.0),
                }}
                styles={KStyle {
                    width: Units::Pixels(450.0).into(),
                    height: Units::Pixels(512.0).into(),
                    left: Units::Stretch(0.5).into(),
                    right: Units::Stretch(0.5).into(),
                    top: Units::Stretch(1.0).into(),
                    bottom: Units::Stretch(1.0).into(),
                    padding: Edge::new(
                        Units::Pixels(20.0),
                        Units::Pixels(20.0),
                        Units::Pixels(50.0),
                        Units::Pixels(20.0),
                    ).into(),
                    ..KStyle::default()
                }}
            >
                <TextWidgetBundle
                styles={KStyle {
                    // top: Units::Stretch(1.0).into(),
                    bottom: Units::Stretch(1.0).into(),
                    ..Default::default()
                }}
                text={TextProps {
                    alignment: Alignment::Middle,
                    content: "Happy Birthday, Mom".to_owned() ,
                    size: 24.0,
                    ..Default::default()
                }}
                />
                <MenuButtonBundle button={MenuButton { text: "Start".into() }} 
                    on_event={handle_click_start}
                />
                <MenuButtonBundle
                    button={MenuButton { text: "Quit".into() }}
                    on_event={handle_click_close}
                />
            </NinePatchBundle>
        </KayakAppBundle>
    };
    
    commands.spawn((MainMenuWidget, UICameraBundle::new(widget_context)));
}

#[derive(FromReflect, Reflect)]
struct ChangeAnimTrigger {
    cast_member: YNYNCast
}

impl Trigger for ChangeAnimTrigger {
    type Param<'w, 's> = Res<'w, Input<KeyCode>>;

    fn trigger(&self, _: Entity, keys: &Self::Param<'_, '_>) -> bool {
        keys.just_pressed(KeyCode::Space)
    }
}

#[derive(Clone, PartialEq, Copy, Reflect, FromReflect)]
enum YNYNCast {
    Prop,
    Director,
    Mom,
    Lydia,
    Mounty_j,
    Tom,
    Gandalf,
}

fn spawn_sprites(
    mut commands: Commands, 
    images: Res<ImageAssets>,
    mut sprites: PxAssets<PxSprite>,
    // mut sprite_params: Sprite3dParams,
) {

    commands.spawn(Camera2dBundle::default());
    // mom-Sheet.png
    
    let runner = sprites.load("art/mom-walk.png");

    commands.spawn((
        PxSpriteBundle::<Seld_layer> {
            sprite: runner.clone(),
            position: IVec2::new(100, YNYN_Y).into(),
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        StateMachine::new((YNYNIdleLState,))
        .trans::<(YNYNIdleLState,)>(YNYNWalkRTrigger, (YNYNWalkLState,))
        .insert_on_enter::<(YNYNWalkLState,)>(AnimBundle {
            sprite: sprites.load_animated("art/mom-walk.png", 2),
            animation: PxAnimationBundle {
                duration: PxAnimationDuration::millis_per_animation(2000),
                on_finish: PxAnimationFinishBehavior::Loop,
                ..default()
            },
        })
        .remove_on_exit::<(YNYNWalkLState,), (PxAnimationBundle, YNYNWalkLComp)>()
        .trans::<(YNYNWalkLState,)>(YNYNIdleLTrigger, (YNYNIdleLState,))
        .insert_on_enter::<(YNYNIdleLState,)>(AnimBundle {
            sprite: sprites.load_animated("art/mom-idle.png", 2),
            animation: PxAnimationBundle {
                duration: PxAnimationDuration::millis_per_animation(2000),
                on_finish: PxAnimationFinishBehavior::Loop,
                ..default()
            },
        })
        .remove_on_exit::<(YNYNIdleLState,), (PxAnimationBundle, YNYNIdleLComp)>()
        .trans::<(YNYNWalkRState,)>(YNYNIdleRTrigger, (YNYNIdleRState,))
        .insert_on_enter::<(YNYNIdleRState,)>(AnimBundle {
            sprite: sprites.load_animated("art/mom-idle-r.png", 2),
            animation: PxAnimationBundle {
                duration: PxAnimationDuration::millis_per_animation(2000),
                on_finish: PxAnimationFinishBehavior::Loop,
                ..default()
            },
        })
        .remove_on_exit::<(YNYNIdleRState,), (PxAnimationBundle, YNYNIdleRComp)>()
        .trans::<(YNYNIdleRState,)>(YNYNWalkRTrigger, (YNYNWalkRState,))
        .insert_on_enter::<(YNYNWalkRState,)>(AnimBundle {
            sprite: sprites.load_animated("art/mom-walk-r.png", 2),
            animation: PxAnimationBundle {
                duration: PxAnimationDuration::millis_per_animation(2000),
                on_finish: PxAnimationFinishBehavior::Loop,
                ..default()
            },
        })
        .remove_on_exit::<(YNYNWalkRState,), (PxAnimationBundle, YNYNWalkRComp)>()
        .trans::<(YNYNIdleLState,)>(YNYNIdleRTrigger, (YNYNIdleRState,))
        .trans::<(YNYNIdleLState,)>(YNYNWalkRTrigger, (YNYNWalkRState,))
        .trans::<(YNYNIdleRState,)>(YNYNWalkRTrigger, (YNYNIdleLState,))
        .trans::<(YNYNIdleRState,)>(YNYNWalkRTrigger, (YNYNWalkLState,))
        .trans::<(YNYNWalkRState,)>(YNYNWalkRTrigger, (YNYNWalkLState,))
        .trans::<(YNYNWalkRState,)>(YNYNWalkRTrigger, (YNYNIdleLState,))
        .trans::<(YNYNWalkLState,)>(YNYNWalkRTrigger, (YNYNWalkRState,))
        .trans::<(YNYNWalkLState,)>(YNYNWalkRTrigger, (YNYNIdleRState,)),)
    ).insert(Person);

    let bg = sprites.load("art/apartment.png");
    commands.spawn(
        PxSpriteBundle::<Seld_layer> {
            sprite: bg.clone(),
            position: IVec2::new(0, 0).into(),
            anchor: PxAnchor::BottomLeft,
            ..default()
        }
    );
    
    commands.insert_resource(NextState(GameState::Ready));

}

// fn animate_sprites(
//     time: Res<FixedTimesteps>,
//     mut query: Query<(&mut Animation, &mut AtlasSprite3dComponent), Without<Door>>,
// ) {
//     let step = time.get_current().unwrap().step;
//     for (mut animation, mut sprite) in query.iter_mut() {
//         animation.timer.tick(step);
//         if animation.timer.just_finished() {
//             sprite.index = animation.frames[animation.current];
//             animation.current += 1;
//             animation.current %= animation.frames.len();
//         }
//     }
// }


fn move_plyr(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PxPosition, With<Person>>,
) {
        if !query.is_empty() {
        let mut plyr_transform = query.single_mut();
        let mut direction = 0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1; 
        }
            
    

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1;
        }

        // Calculate the new horizontal paddle position based on player input
        let new_plyr_position = plyr_transform.x + direction;// * PLYR_SPEED;// * (1 as i32 / 90 as i32);
        // println!("pos {}", new_plyr_position);
        let ivec = IVec2::new(new_plyr_position, 77);

        // Update the paddle position,
        // making sure it doesn't cause the paddle to leave the arena
        let left_bound = LEFT_WALL;
        let right_bound = RIGHT_WALL;

        plyr_transform.0 = ivec;  //.x = new_plyr_position.clamp(left_bound as i32, right_bound as i32);
    }   
}
    

