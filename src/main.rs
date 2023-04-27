use bevy::{
    prelude::*, 
    time::{FixedTimestep, Stopwatch}, ecs::system::Command, app::AppExit, render::render_resource::Face,
};
use bevy_asset_loader::prelude::*;
use bevy::utils::Duration;
// use syn::token::At;
use iyes_loopless::prelude::*;
use kayak_ui::prelude::{widgets::*, *};
use kayak_ui::{prelude::FontMapping};
use kayak_font::KayakFont;
use seldom_state::prelude::*;
use bevy_obj::*;
// use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

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
    #[asset(path = "art/monkobj/monk-1.obj")]
    monk_idle_1: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-2.obj")]
    monk_idle_2: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-3.obj")]
    monk_idle_3: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-4.obj")]
    monk_idle_4: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-5.obj")]
    monk_idle_5: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-6.obj")]
    monk_idle_6: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-7.obj")]
    monk_idle_7: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-8.obj")]
    monk_idle_8: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-1.png")]
    monk_idle_1_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-2.png")]
    monk_idle_2_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-3.png")]
    monk_idle_3_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-4.png")]
    monk_idle_4_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-5.png")]
    monk_idle_5_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-6.png")]
    monk_idle_6_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-7.png")]
    monk_idle_7_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-8.png")]
    monk_idle_8_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-9.obj")]
    monk_walk_1: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-10.obj")]
    monk_walk_2: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-11.obj")]
    monk_walk_3: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-12.obj")]
    monk_walk_4: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-13.obj")]
    monk_walk_5: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-14.obj")]
    monk_walk_6: Handle<Mesh>,
    #[asset(path = "art/monkobj/monk-9.png")]
    monk_walk_1_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-10.png")]
    monk_walk_2_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-11.png")]
    monk_walk_3_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-12.png")]
    monk_walk_4_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-13.png")]
    monk_walk_5_png: Handle<Image>,
    #[asset(path = "art/monkobj/monk-14.png")]
    monk_walk_6_png: Handle<Image>,
    #[asset(path = "art/monkobj/monastery-back2-shelf-1.obj")]
    monastery_back_shelf: Handle<Mesh>,
    #[asset(path = "art/monkobj/monastery-back2-shelf-1.png")]
    monastery_back_shelf_png: Handle<Image>,
    #[asset(path = "art/monkobj/monastery-back-1.obj")]
    monastery_back: Handle<Mesh>,
    #[asset(path = "art/monkobj/monastery-back-1.png")]
    monastery_back_png: Handle<Image>,
    #[asset(path = "art/monkobj/greywall-1.obj")]
    greywall: Handle<Mesh>,
    #[asset(path = "art/monkobj/greyWall-1.png")]
    greywall_png: Handle<Image>,
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
    #[asset(path = "art/monkobj/monk-1.obj")]
    monk_1_idle_mesh: Handle<Mesh>,

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugin(WorldInspectorPlugin)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_plugin(StateMachinePlugin)
        .add_plugin(ObjPlugin)
        .add_plugin(TriggerPlugin::<YNYNIdleLTrigger>::default())
        .add_plugin(TriggerPlugin::<YNYNWalkLTrigger>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
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
        .add_enter_system(GameState::Staging, scene_setup.label("spawn_sprites"))
        .add_enter_system(GameState::Ready, fill_mando_queue2.label("fill_mando_queue"))
        .add_fixed_timestep_system("my_fixed_update", 0, operate_mando_queue2.run_in_state(GameState::Ready).after("fill_mando_queue"))
        .add_fixed_timestep_system("my_fixed_update", 0, ynyn_walk_l.run_in_state(GameState::Ready))
        .add_fixed_timestep_system("my_fixed_update", 0, move_plyr.run_in_state(GameState::Ready))
        // .add_system_set(
        //     ConditionSet::new()
        //         .run_in_state(GameState::Ready)
        //         .with_system(check_completed_mando)

        //         .into()
        // )
        
        .run();

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
    mut meshes: ResMut<Assets<Scene>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    
}

fn main_menu(
    mut commands: Commands, 
    mut set: ParamSet<(
        Res<ImageAssets>,
        ResMut<FontMapping>,
        )>,
) {
    println!("Gapho");
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

fn scene_setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut stdmats: ResMut<Assets<StandardMaterial>>,
    assets: Res<ImageAssets>,
    assetos: Res<AssetServer>,
) {
    println!("Bapho");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.0, 150.0),//.looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection { fov: 0.05, aspect_ratio: 1.778, near: 0.1, far: 1000.0 }),
        ..Default::default()
    });


    // smaller backgrounds

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-2.0, 0.0, -5.0) * Transform::from_scale((02.101, 02.101, 02.101).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });//        .insert(Collider::Cubo);
    // // .insert(Restitution::coefficient(0.7))
    // // .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));;

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(2.0, 0.0, -5.0) * Transform::from_scale((02.101, 02.101, 02.101).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(6.0, 0.0, -5.0) * Transform::from_scale((02.101, 02.101, 02.101).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_shelf.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_shelf_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    // // big backgrounds
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-10.0, 0.0, -30.0) * Transform::from_scale((7.001, 7.001, 7.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, -30.0) * Transform::from_scale((7.001, 7.001, 7.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(10.0, 0.0, -30.0) * Transform::from_scale((7.001, 7.001, 7.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(12.0, 0.0, -10.0) * Transform::from_scale((7.001, 7.001, 7.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI/2.)),
        mesh: assets.monastery_back_shelf.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_shelf_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-2.0, 0.0, -10.0) * Transform::from_scale((7.001, 7.001, 7.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI/2.)),
        mesh: assets.monastery_back_shelf.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_shelf_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(
        StateMachine::new(YNYNIdleLState)
        .trans::<YNYNIdleLState>(YNYNWalkLTrigger, YNYNWalkLState)
        .trans::<YNYNWalkLState>(YNYNIdleLTrigger, YNYNIdleLState)
        .insert_on_enter::<YNYNWalkLState>(VoxAnimBundle {
            voxel_mesh: assets.monk_walk_1.clone(),
            voxel_mat: stdmats.add(StandardMaterial {
                base_color_texture: Some(assets.monk_walk_1_png.clone()),
                perceptual_roughness: 1.0,
                cull_mode: Some(Face::Front),
                ..default()
            }),
            animation: VoxAnim {
                frame_duration: 250,
                frame_time_lapsed: 0,
                frame_count: 8,
                meshes: vec![
                assets.monk_walk_1.clone(), 
                assets.monk_walk_2.clone(), 
                assets.monk_walk_3.clone(), 
                assets.monk_walk_4.clone(), 
                assets.monk_walk_5.clone(), 
                assets.monk_walk_6.clone()
                ],
                materials: vec![
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_2_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_3_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_4_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_5_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_6_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    ],
            },
        })
        .insert_on_enter::<YNYNIdleLState>(VoxAnimBundle {
            voxel_mesh: assets.monk_idle_1.clone(),
            voxel_mat: stdmats.add(StandardMaterial {
                base_color_texture: Some(assets.monk_idle_1_png.clone()),
                perceptual_roughness: 1.0,
                cull_mode: Some(Face::Front),
                ..default()
            }),
            animation: VoxAnim {
                frame_duration: 250,
                frame_time_lapsed: 0,
                frame_count: 6,
                meshes: vec![
                assets.monk_idle_1.clone(), 
                assets.monk_idle_1.clone(), 
                assets.monk_idle_1.clone(), 
                assets.monk_idle_1.clone(), 
                assets.monk_idle_2.clone(), 
                assets.monk_idle_3.clone(), 
                assets.monk_idle_4.clone(),
                assets.monk_idle_5.clone(), 
                assets.monk_idle_5.clone(), 
                assets.monk_idle_5.clone(), 
                assets.monk_idle_5.clone(), 
                assets.monk_idle_6.clone(), 
                assets.monk_idle_7.clone(),  
                assets.monk_idle_8.clone(),
                ],
                materials: vec![
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_2_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_3_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_4_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }),
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_5_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_5_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_5_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_5_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_6_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_7_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }),  
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_8_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        ..default()
                    }),
                ],
            },
        })
        .remove_on_exit::<YNYNWalkLState, (VoxAnim, Handle<Mesh>, YNYNWalkLComp)>()
        .remove_on_exit::<YNYNIdleLState, (VoxAnim, Handle<Mesh>, YNYNIdleLComp)>()
    ).insert(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0) * Transform::from_scale((01.07, 01.07, 01.07).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monk_idle_1.clone(),//assetos.load("art/idle/monk-idle-1n5.vox"),//assets.monk_idle_1n5.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monk_idle_1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    }).insert(VoxelChar {
        name: "monk".to_owned()
    }).insert(VoxelVelocity {
        vector: Vec3::ZERO
    }).insert(YNYNIdleLComp);

    
    
    commands.insert_resource(NextState(GameState::Ready));

}

fn move_plyr(
    keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet<(
        Query<(&mut Transform, &mut VoxelVelocity, Entity), With<VoxelChar>>,
        Query<&mut Transform, With<Camera3d>>, 
    )>,
) {
    let player_is_empty = set.p0().is_empty();
    let camera_is_empty = set.p1().is_empty();
        if !player_is_empty && !camera_is_empty {
        // let form = set.p0().single_mut();

        let mut x_velocity = 0.;
        let mut z_velocity = 0.;
        let mut x_direction = 0.;
        let mut z_direction = 0.;

        if keyboard_input.pressed(KeyCode::Left) {
            x_velocity -= 0.01; 
            x_direction = -1.;
        }
        
        if keyboard_input.pressed(KeyCode::Right) {
            x_velocity += 0.01;
            x_direction = 1.;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            z_velocity -= 0.08; 
            z_direction = -1.;
        }
        
        if keyboard_input.pressed(KeyCode::Down) {
            z_velocity += 0.08;
            z_direction = 1.;
        }
        
        let entity = set.p0().single_mut().2;
        let velocity_vec = Vec3 { x: x_direction, y: 0., z: z_direction};
        
        let mut cam_transform = set.p1().single_mut().translation;
        let cam_vec = Vec3 { x: cam_transform.x + x_velocity, y: cam_transform.y, z: cam_transform.z + z_velocity};
        set.p1().single_mut().translation = cam_vec;

        
        let plyr_transform = set.p0().single_mut().0.translation.clone();
        let plyr_vec = Vec3 { x: plyr_transform.x + x_velocity, y: plyr_transform.y, z: plyr_transform.z + z_velocity };
        let mut plyr_rot = set.p0().single_mut().0.rotation;
        if velocity_vec.x < 0. && velocity_vec.z == 0. {
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI + PI/2.);
        } else if velocity_vec.x == 0. && velocity_vec.z > 0. {
            plyr_rot = Quat::from_axis_angle(Vec3::Y, 0.);
        } else if velocity_vec.x == 0. && velocity_vec.z < 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI);
        } else if velocity_vec.x > 0. && velocity_vec.z == 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI/2.);
        } else if velocity_vec.x < 0. && velocity_vec.z < 0. {
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI + PI/4.);
        } else if velocity_vec.x < 0. && velocity_vec.z > 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI + PI/2. + PI/4.);
        } else if velocity_vec.x > 0. && velocity_vec.z < 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI/2. + PI/4.);
        } else if velocity_vec.x > 0. && velocity_vec.z > 0.{
            plyr_rot = Quat::from_axis_angle(Vec3::Y, PI/4.);
        }

        
        // let left_bound = LEFT_WALL;
        // let right_bound = RIGHT_WALL;

        set.p0().single_mut().0.translation = plyr_vec;
        set.p0().single_mut().0.rotation = plyr_rot;
        set.p0().single_mut().1.vector = velocity_vec; //Vec3 { x: x_direction, y: 0., z: z_direction};
        
    }   
}
    
fn ynyn_walk_l(
    // mut qry: Query<(Entity, &YNYNIdleLComp, &mut VoxAnim, &mut Handle<Scene>)>,
    // mut qry2: Query<(Entity, &YNYNWalkLComp, &mut VoxAnim, &mut Handle<Scene>)>,
    mut commands: Commands, 
    mut set: ParamSet<(
        Query<(Entity, &YNYNIdleLComp, &mut VoxAnim, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &VoxelVelocity)>,
        Query<(Entity, &YNYNWalkLComp, &mut VoxAnim, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &VoxelVelocity)>,
    )>,
    assets: Res<ImageAssets>,
    time: Res<FixedTimesteps>,
) {
    for (entity, comp, mut vox_anim, mut mesh, mut mat, velo) in &mut set.p0() {
        // println!("Cooperkins");
        let step = time.get_current().unwrap().step;
        let index = vox_anim.frame_time_lapsed / vox_anim.frame_duration;
        if vox_anim.frame_time_lapsed > vox_anim.frame_count * vox_anim.frame_duration {
            vox_anim.frame_time_lapsed = 0;
        }
        if index <= vox_anim.meshes.len() - 1 && index >= 0 {
            *mesh = vox_anim.meshes[index].clone();
            *mat = vox_anim.materials[index].clone();
        }
        if vox_anim.frame_time_lapsed > vox_anim.frame_duration * vox_anim.meshes.len() {
            vox_anim.frame_time_lapsed = 0;
        } 
        vox_anim.frame_time_lapsed += step.as_millis() as usize;
        // println!("Idling x {} z {}", velo.vector.x, velo.vector.z);
        if velo.vector.x != 0. || velo.vector.z != 0. {
            // println!("Be moving!");
            commands.entity(entity).insert(YNYNWalkLComp);
        } else {
            // println!("Be idling");
            // commands.entity(entity).insert(YNYNIdleLComp);
        }
    }

    for (entity, comp, mut vox_anim, mut mesh, mut mat, velo) in &mut set.p1() {
        // println!("Cooperkins");
        let step = time.get_current().unwrap().step;
        let index = vox_anim.frame_time_lapsed / vox_anim.frame_duration;
        if vox_anim.frame_time_lapsed > vox_anim.frame_count * vox_anim.frame_duration {
            vox_anim.frame_time_lapsed = 0;
        }
        if index <= vox_anim.meshes.len() - 1 && index >= 0 {
            *mesh = vox_anim.meshes[index].clone();
            *mat = vox_anim.materials[index].clone();
        }
        if vox_anim.frame_time_lapsed > vox_anim.frame_duration * vox_anim.meshes.len() {
            vox_anim.frame_time_lapsed = 0;
        } 
        vox_anim.frame_time_lapsed += step.as_millis() as usize;
        // println!("Walkin x {} z {}", velo.vector.x, velo.vector.z);
        if velo.vector.x != 0. || velo.vector.z != 0. {
            // println!("Be moving!");
            // commands.entity(entity).insert(YNYNWalkLComp);
        } else {
            // println!("Be idling");
            commands.entity(entity).insert(YNYNIdleLComp);
        }
    }
}
