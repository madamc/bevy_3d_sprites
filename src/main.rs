use bevy::{
    prelude::*, 
    time::{FixedTimestep, Stopwatch}, ecs::system::Command, app::AppExit, render::render_resource::Face,
};
use bevy_asset_loader::prelude::*;
//use bevy-inspector-egui
use bevy::utils::Duration;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_loopless::prelude::*;
use kayak_ui::prelude::{widgets::*, *};
use kayak_ui::{prelude::FontMapping};
use seldom_state::prelude::*;
use bevy_obj::*;
use bevy_vox_mesh::VoxMeshPlugin;
use std::f32::consts::PI;
use bevy_kira_audio::*;

mod macros;
// mod mando_queue;
mod messages;
mod seldom_pixel_prep;
mod game_setup;
mod game_commands;
mod movement;
mod animations;

mod ui;
mod audio;
mod components;

use ui::*;
use game_setup::{{load_assets::ImageAssets, scene_1_setup}, {scene_1_setup::scene_setup}};
use audio::GameAudioPlugin;
use components::*;
use movement::{*, move_player::move_plyr, move_panel::move_panel};
use animations::cycle_ynyn_anims::cycle_ynyn_anim;

use crate::{game_commands::mando_queue::{fill_mando_queue, operate_mando_queue, MandoQueue}};
use crate::{seldom_pixel_prep::*};

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const ATLAS_COLUMNS: usize = 8;
const TIME_STEP: f32 = 10.0;
const PLYR_SPEED: i32 = 2;
const YNYN_Y: i32 = 4;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
            window: WindowDescriptor {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            },
            add_primary_window: true,
            exit_on_all_closed: true,
            close_when_requested: true,
        })
        )
        // .add_plugin(WorldInspectorPlugin)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_plugin(StateMachinePlugin)
        .add_plugin(ObjPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(VoxMeshPlugin::default())
        .add_plugin(TriggerPlugin::<YNYNIdleLTrigger>::default())
        .add_plugin(TriggerPlugin::<YNYNWalkLTrigger>::default())
        .add_plugin(TriggerPlugin::<YNYNSitTrigger>::default())
        .add_plugin(TriggerPlugin::<YNYNWriteTrigger>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2.0,
        })
        .init_resource::<MandoQueue>()
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
        .add_enter_system(GameState::MainMenu, main_menu.label("main_menu"))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(menu_start)
                .into()
        )
        .add_enter_system(GameState::Staging, scene_setup.label("spawn_sprites"))
        .add_enter_system(GameState::Ready, fill_mando_queue.label("fill_mando_queue"))
        .add_fixed_timestep_system("my_fixed_update", 0, operate_mando_queue.run_in_state(GameState::Ready).after("fill_mando_queue"))
        .add_fixed_timestep_system("my_fixed_update", 0, cycle_ynyn_anim.run_in_state(GameState::Ready))
        .add_fixed_timestep_system("my_fixed_update", 0, move_plyr.run_in_state(GameState::Ready))
        .add_fixed_timestep_system("my_fixed_update", 0, move_panel.run_in_state(GameState::Ready))
        
        .run();

}

fn menu_start (
    mut commands: Commands,
    mut ev_start: EventReader<MenuStartEvent>,
    query: Query<Entity, With<MainMenuWidget>>,
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

fn main_menu(
    mut commands: Commands, 
    mut set: ParamSet<(
        Res<ImageAssets>,
        ResMut<FontMapping>,
        )>,
) {
    // println!("Gapho");
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

#[derive(Component)]
pub struct VoxelChar {
    name: String
}

#[derive(Component)]
pub struct VoxelVelocity {
    vector: Vec3
}
