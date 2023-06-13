use bevy::prelude::*;
use iyes_loopless::prelude::FixedTimesteps;

use crate::{VoxelVelocity, VoxAnim, seldom_pixel_prep::*, game_setup::load_assets::ImageAssets};

pub fn cycle_ynyn_anim(
    qry: Query<&VoxelVelocity>,
    mut commands: Commands, 
    mut set: ParamSet<(
        Query<(Entity, &YNYNIdleLComp, &mut VoxAnim, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &VoxelVelocity)>,
        Query<(Entity, &YNYNWalkLComp, &mut VoxAnim, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &VoxelVelocity)>,
        Query<(Entity, &YNYNSitComp, &mut VoxAnim, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &VoxelVelocity)>,
        Query<(Entity, &YNYNWriteComp, &mut VoxAnim, &mut Handle<Mesh>, &mut Handle<StandardMaterial>, &VoxelVelocity)>,
    )>,
    assets: Res<ImageAssets>,
    time: Res<FixedTimesteps>,
) {
    
    for (entity, comp, mut vox_anim, mut mesh, mut mat, veloc) in &mut set.p0() {
        
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
        // let prnt = prnt.get();
        // if let Ok(veloc) = qry.get_component::<VoxelVelocity>(prnt.get()) {
            if veloc.vector.x != 0. || veloc.vector.z != 0. {
                // println!("Be moving!");
                commands.entity(entity).insert(YNYNWalkLComp);
            } else {
                // println!("Be idling");
                // commands.entity(entity).insert(YNYNIdleLComp);
            }
        // }
        
    }

    for (entity, comp, mut vox_anim, mut mesh, mut mat, veloc) in &mut set.p1() {
        
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
        // if let Ok(veloc) = qry.get_component::<VoxelVelocity>(prnt.get()) {
            if veloc.vector.x != 0. || veloc.vector.z != 0. {
                // println!("Be moving!");
                // commands.entity(entity).insert(YNYNWalkLComp);
            } else {
                // println!("Be idling");
                commands.entity(entity).insert(YNYNIdleLComp);
            }
        // }
        
    }

    for (entity, comp, mut vox_anim, mut mesh, mut mat, veloc) in &mut set.p2() {
        // println!("Be sitting!");
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
        
    }

    for (entity, comp, mut vox_anim, mut mesh, mut mat, veloc) in &mut set.p3() {
        // println!("Be writing!");
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
    }
}
