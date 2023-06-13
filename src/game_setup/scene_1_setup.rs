
use std::f32::consts::PI;

use bevy::{prelude::*, render::render_resource::Face};
use iyes_loopless::state::NextState;
use seldom_state::prelude::StateMachine;
use crate::{seldom_pixel_prep::*, VoxelChar, VoxelVelocity, components::{GameState, Panel, PanelChar}};

use super::load_assets::ImageAssets;

const WALL_MULTER: f32 = 1.6;

pub fn scene_setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut stdmats: ResMut<Assets<StandardMaterial>>,
    assets: Res<ImageAssets>,
    assetos: Res<AssetServer>,
) {
    println!("Bapho");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 3.55, 150.0),
        projection: Projection::Perspective(PerspectiveProjection { fov: 0.05, aspect_ratio: 1.778, near: 0.1, far: 1000.0 }),
        ..Default::default()
    });

    // tier 1
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-5.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((01.0, 01.0, 01.0).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_1.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-4.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((01.0, 01.0, 01.0).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_2.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_2_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    // // big backgrounds
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-3.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_3.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_3_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-2.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_4.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_4_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-1.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
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
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-0.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_1.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(1.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_2.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_2_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(2.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_3.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_3_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(3.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_4.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_4_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(4.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_1.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(5.0*WALL_MULTER, 0.0, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_2.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_2_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-5.0*WALL_MULTER, 0.0, -5.0+(1.0*WALL_MULTER)) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI/2.)),
        mesh: assets.monastery_back_1.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(6.0*WALL_MULTER, 0.0, -5.0+(1.0*WALL_MULTER)) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI/2.)),
        mesh: assets.monastery_back_1.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });




    //tier 2
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-5.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((01.0, 01.0, 01.0).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_1_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-4.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((01.0, 01.0, 01.0).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_2_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_2_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-3.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_3_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_3_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-2.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_4_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_4_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-1.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_2_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_2_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-0.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_1_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(1.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_2_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_2_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(2.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_3_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_3_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(3.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_4_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_4_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(4.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_1_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(5.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monastery_back_2_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_2_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-5.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0+(1.0*WALL_MULTER)) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI/2.)),
        mesh: assets.monastery_back_1_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(6.0*WALL_MULTER, 1.0*WALL_MULTER, -5.0+(1.0*WALL_MULTER)) * Transform::from_scale((1.001, 1.001, 1.001).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI/2.)),
        mesh: assets.monastery_back_1_top.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.monastery_back_1_top_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    //furniture
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.65*WALL_MULTER, 0.0*WALL_MULTER, -4.0) * Transform::from_scale((-1.00, 0.48, 1.00).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI/2.0)),
        mesh: assets.chair.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.chair_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0*WALL_MULTER, 0.0*WALL_MULTER, -4.0) * Transform::from_scale((-1.00, 0.60, 1.00).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.table.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.table_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    });

    // panel

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(8.0*WALL_MULTER, 2.2*WALL_MULTER, -10.1) * Transform::from_scale((-0.3, 0.3, 0.3).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.panel_bg1.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.panel_bg1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    }).insert(Panel);

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(8.0*WALL_MULTER, 2.2*WALL_MULTER, -10.0) * Transform::from_scale((-0.3, 0.3, 0.3).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.panel_chars1.clone(),
        material: stdmats.add(StandardMaterial {
            base_color_texture: Some(assets.panel_chars1_png.clone()),
            perceptual_roughness: 1.0,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        ..Default::default()
    }).insert(PanelChar);

    // player
    commands.spawn(
        StateMachine::new(YNYNIdleLState)
        .trans::<YNYNIdleLState>(YNYNWalkLTrigger, YNYNWalkLState)
        .trans::<YNYNWalkLState>(YNYNIdleLTrigger, YNYNIdleLState)
        .trans::<YNYNIdleLState>(YNYNSitTrigger, YNYNSitState)
        .trans::<YNYNWalkLState>(YNYNSitTrigger, YNYNSitState)
        .trans::<YNYNWriteState>(YNYNSitTrigger, YNYNSitState)
        .trans::<YNYNSitState>(YNYNWriteTrigger, YNYNWriteState)
        .insert_on_enter::<YNYNWalkLState>(VoxAnimBundle {
            voxel_mesh: assets.monk_walk_1.clone(),
            voxel_mat: stdmats.add(StandardMaterial {
                base_color_texture: Some(assets.monk_walk_1_png.clone()),
                perceptual_roughness: 1.0,
                cull_mode: Some(Face::Back),
                ..default()
            }),
            animation: VoxAnim {
                frame_duration: 250,
                frame_time_lapsed: 0,
                frame_count: 8,
                meshes: vec![
                assets.monk_walk_1.clone(), 
                assets.monk_walk_2.clone(), 
                assets.monk_walk_4.clone(), 
                assets.monk_walk_3.clone(), 
                assets.monk_walk_4.clone(), 
                assets.monk_walk_2.clone(),
                ],
                materials: vec![
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_2_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_4_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_3_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_4_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_walk_2_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
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
                cull_mode: Some(Face::Back),
                ..default()
            }),
            animation: VoxAnim {
                frame_duration: 250,
                frame_time_lapsed: 0,
                frame_count: 6,
                meshes: vec![
                assets.monk_idle_0.clone(),  
                assets.monk_idle_1.clone(), 
                assets.monk_idle_0.clone(), 
                assets.monk_idle_2.clone(), 
                ],
                materials: vec![
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_0_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_0_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_idle_2_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                ],
            },
        })
        .insert_on_enter::<YNYNSitState>(VoxAnimBundle {
            voxel_mesh: assets.monk_sit_0.clone(),
            voxel_mat: stdmats.add(StandardMaterial {
                base_color_texture: Some(assets.monk_sit_0_png.clone()),
                perceptual_roughness: 1.0,
                cull_mode: Some(Face::Back),
                ..default()
            }),
            animation: VoxAnim {
                frame_duration: 250,
                frame_time_lapsed: 0,
                frame_count: 8,
                meshes: vec![
                assets.monk_sit_0.clone(), 
                ],
                materials: vec![
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_sit_0_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    ],
            },
        })
        .insert_on_enter::<YNYNWriteState>(VoxAnimBundle {
            voxel_mesh: assets.monk_write_1.clone(),
            voxel_mat: stdmats.add(StandardMaterial {
                base_color_texture: Some(assets.monk_write_1_png.clone()),
                perceptual_roughness: 1.0,
                cull_mode: Some(Face::Back),
                ..default()
            }),
            animation: VoxAnim {
                frame_duration: 250,
                frame_time_lapsed: 0,
                frame_count: 6,
                meshes: vec![
                assets.monk_write_1.clone(),  
                assets.monk_write_2.clone(), 
                assets.monk_write_3.clone(), 
                assets.monk_write_4.clone(), 
                ],
                materials: vec![
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_write_1_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_write_2_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_write_3_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                    stdmats.add(StandardMaterial {
                        base_color_texture: Some(assets.monk_write_4_png.clone()),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Back),
                        ..default()
                    }), 
                ],
            },
        })
        .remove_on_exit::<YNYNWalkLState, (VoxAnim, Handle<Mesh>, YNYNWalkLComp)>()
        .remove_on_exit::<YNYNIdleLState, (VoxAnim, Handle<Mesh>, YNYNIdleLComp)>()
        .remove_on_exit::<YNYNSitState, (VoxAnim, Handle<Mesh>, YNYNSitComp)>()
        .remove_on_exit::<YNYNWriteState, (VoxAnim, Handle<Mesh>, YNYNWriteComp)>()
    ).insert(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0) * Transform::from_scale((-1.0, 1.0, 1.0).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.monk_idle_1.clone(),
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