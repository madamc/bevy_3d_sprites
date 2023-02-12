use bevy::{
    prelude::*, 
    time::{FixedTimestep, Stopwatch}, ecs::system::Command, app::AppExit,
};
use bevy_sprite3d::*;
use bevy_asset_loader::prelude::*;
use bevy::utils::Duration;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::render::camera::PerspectiveProjection;
use rand::{prelude::SliceRandom, Rng};
use syn::token::At;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_loopless::prelude::*;
use kayak_ui::prelude::{widgets::*, *};
use kayak_ui::{prelude::FontMapping};
use kayak_font::KayakFont;

mod messages;
mod mandoqueue;
mod game_commands;
mod ui;
mod audio;
mod type_writer;
mod components;
pub mod Components;

use type_writer::*;
use ui::*;
use audio::GameAudioPlugin;
use mandoqueue::*;
use mandoqueue::fill_mando_queue;
use mandoqueue::operate_mando_queue;
use game_commands::*;
use components::*;

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const ATLAS_COLUMNS: usize = 8;
const TIME_STEP: f32 = 10.0;
const PLYR_SPEED: f32 = 2.0;


#[derive(AssetCollection, Resource)]
pub struct ImageAssets { 
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., 
            columns = 8, rows = 7, padding_x = 0., padding_y = 0.))]
    #[asset(path = "art\\momcard1.png")]
    tileset: Handle<TextureAtlas>,
    #[asset(path = "art\\apartment.png")]
    sprite: Handle<Image>,
    #[asset(path = "art\\panel-bb.png")]
    panel: Handle<Image>,
    #[asset(path = "art\\panel--button-bb.png")]
    panel_btn: Handle<Image>,
    #[asset(path = "art\\panel--button-bb-hover.png")]
    panel_btn_hov: Handle<Image>,
    #[asset(path = "art\\panel--button-bb-clicked.png")]
    panel_btn_clk: Handle<Image>,
    #[asset(path = "ui\\pcsenior.kttf")]
    kfont: Handle<KayakFont>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugin(WorldInspectorPlugin)
        .add_plugin(TypeWriterTextPlugin)
        .add_plugin(Sprite3dPlugin)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
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
        .add_enter_system(GameState::Staging, spawn_sprites.label("spawn_sprites").after("setup"))
        .add_enter_system(GameState::Ready, fill_mando_queue.label("fill_mando_queue"))
        

        
        .add_fixed_timestep_system("my_fixed_update", 0, operate_mando_queue.run_in_state(GameState::Ready).after("fill_mando_queue"))
        .add_fixed_timestep_system("my_fixed_update", 0, animate_sprites.run_in_state(GameState::Ready))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Ready)
                .with_system(check_completed_mando)

                .into()
        )
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::WHITE.into()),
    //     transform: Transform::from_xyz(-0.9, 0.5, -3.1),
    //     ..default()
    // });
    // // sphere
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.6, subdivisions: 20 })),
    //     material: materials.add(Color::WHITE.into()),
    //     transform: Transform::from_xyz(-0.9, 0.5, -4.2),
    //     ..default()
    // });

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
    // state.apply(world);
    // let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    // cc.completed = true;
}

fn spawn_sprites(
    mut commands: Commands, 
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {

    commands.spawn((Camera3dBundle {
        //choosing 3 arbitrarily, as long as it's greater than 0 so the kayak UI camera will render first.
        camera: Camera { priority: 3, hdr: false, ..Default::default() },
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        projection: bevy::prelude::Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 2.0,
            ..Default::default()
        }),
        ..Default::default()
    },
    BloomSettings { // just a complete guess at configuration settings.
        intensity: 0.9,
        knee: 0.5,
        ..Default::default()
        },
    ));

    let mut map = vec![
        vec![(0,0), (0,0)],
        vec![(0,0), (0,0)],
        vec![(0,0), (0,0)],
        vec![(0,0), (0,0)],
        vec![(0,0), (0,0)],
        vec![(0,0), (0,0)],
        vec![(0,0), (0,0)],
        vec![(0,0), (0,0)],
    ];

    // add zero padding to the map
    map.insert(0, vec![(0,0); map[0].len()]);
    map.push(vec![(0,0); map[0].len()]);
    for row in map.iter_mut() {
        row.insert(0, (0,0));
        row.push((0,0));
    }

    // first horizontally, then vertically, scan along the map. If we find
    // a point transitioning from (0,0) to something else, add a wall there.

    let mut rng = rand::thread_rng();

    // --------------------- characters, enemies, props ---------------------

    let mut entity = |(x, y), tile_x, tile_y, height, frames| {
        let mut timer = Timer::from_seconds(0.4, TimerMode::Repeating);
        timer.set_elapsed(Duration::from_secs_f32(rng.gen_range(0.0..0.4)));

        for i in 0usize..height {
            let mut c = commands.spawn((AtlasSprite3d {
                    atlas: images.tileset.clone(),
                    pixels_per_metre: 32.,
                    index: (tile_x + (tile_y - i) * ATLAS_COLUMNS) as usize,
                    transform: Transform::from_xyz(x as f32, i as f32 + -0.25, y),
                    ..default()
                }.bundle(&mut sprite_params),
                FaceCamera {},
            ));
            
            if frames > 1 {
                c.insert(Animation {
                    frames: (0..frames).map(|j| {let coor = j + tile_x + (tile_y - i) * ATLAS_COLUMNS as usize; println!( "Harbo {}", coor); coor}).collect(),
                    current: 0,
                    timer: timer.clone(),
                });
            }
            println!("Added entity");
            if tile_x == 0 && tile_y == 1 {
                println!("Added poisen");
                c.insert(Person{});
            }
            else if tile_x == 2 && tile_y == 1 {
                println!("Added Door");
                c.insert(Door);
            }
        }
    };

    // 3 humans
    entity((2.5, -2.509), 0, 1, 1, 2); // mom
    entity((0.0, -2.5095), 2, 1, 1, 6); // door
    entity((1.5, -2.51), 0, 3, 1, 2); // Gandalf
    entity((0.0, -2.51),  2, 3, 1, 2); // Lydia

    commands.spawn((Sprite3d {
        image: images.sprite.clone(),

        pixels_per_metre: 32.,

        partial_alpha: true,

        transform: Transform::from_xyz(0., 0., -2.5101),
        double_sided: true,

        pivot: Some(Vec2::new(0.5, 0.5)),

        ..default()
    }.bundle(&mut sprite_params),
        FaceCamera {}
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20000.0,
            color: Color::rgb(1.0, 231./255., 221./255.),
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, -1.8),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20000.0,
            color: Color::rgb(1.0, 231./255., 221./255.),
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, -8.0, -1.8),
        ..default()
    });

    commands.insert_resource(NextState(GameState::Ready));

}

fn animate_sprites(
    time: Res<FixedTimesteps>,
    mut query: Query<(&mut Animation, &mut AtlasSprite3dComponent), Without<Door>>,
) {
    let step = time.get_current().unwrap().step;
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(step);
        if animation.timer.just_finished() {
            sprite.index = animation.frames[animation.current];
            animation.current += 1;
            animation.current %= animation.frames.len();
        }
    }
}


fn move_plyr(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Person>>,
) {
    
        let mut plyr_transform = query.single_mut();
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0; 
        }
            
    

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        // Calculate the new horizontal paddle position based on player input
        let new_plyr_position = plyr_transform.translation.x + direction * PLYR_SPEED * (1 as f32 / 90 as f32);

        // Update the paddle position,
        // making sure it doesn't cause the paddle to leave the arena
        let left_bound = LEFT_WALL;
        let right_bound = RIGHT_WALL;

        plyr_transform.translation.x = new_plyr_position.clamp(left_bound, right_bound);
    // }   
}
    

