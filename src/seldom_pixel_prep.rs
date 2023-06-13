use bevy::{prelude::*, ecs::system::SystemState};
use seldom_state::prelude::Trigger;
use seldom_pixel::{sprite::PxSprite, prelude::{PxAnimationBundle, px_layer}};

#[px_layer]
pub struct Seld_layer;

#[px_layer]
pub struct Bckrnd_layer;


#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct YNYNIdleLState;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct YNYNWalkLState;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct YNYNSitState;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct YNYNWriteState;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct YNYNIdleRState;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct YNYNWalkRState;


#[derive(Clone, Component)]
pub struct YNYNWalkLComp;

#[derive(Clone, Component)]
pub struct YNYNIdleLComp;

#[derive(Clone, Component)]
pub struct YNYNSitComp;

#[derive(Clone, Component)]
pub struct YNYNWriteComp;

#[derive(Clone, Component)]
pub struct YNYNWalkRComp;

#[derive(Clone, Component)]
pub struct YNYNIdleRComp;


#[derive(FromReflect, Reflect)]
pub struct YNYNWalkLTrigger;
impl Trigger for YNYNWalkLTrigger {
    type Param<'w, 's> = (Commands<'w, 's>, Query<'w, 's, Entity, Added<YNYNWalkLComp>>);

    fn trigger(&self, _: Entity, added: &Self::Param<'_, '_>) -> bool {
        // println!("not triggered walk");
        for ent in added.1.iter() {
            // println!("triggered walk");
            return true
        }
        return false
    }
}
#[derive(FromReflect, Reflect)]
pub struct YNYNIdleLTrigger;
impl Trigger for YNYNIdleLTrigger {
    type Param<'w, 's> = (Commands<'w, 's>, Query<'w, 's, Entity, Added<YNYNIdleLComp>>);

    fn trigger(&self, _: Entity, added: &Self::Param<'_, '_>) -> bool {
        // println!("not triggered idle");
        for ent in added.1.iter() {
            // println!("triggered idle");
            return true
        }
        return false
    }
}

#[derive(FromReflect, Reflect)]
pub struct YNYNSitTrigger;
impl Trigger for YNYNSitTrigger {
    type Param<'w, 's> = (Commands<'w, 's>, Query<'w, 's, Entity, Added<YNYNSitComp>>);

    fn trigger(&self, _: Entity, added: &Self::Param<'_, '_>) -> bool {
        // println!("not triggered not");
        for ent in added.1.iter() {
            // println!("triggered sit");
            return true
        }
        return false
    }
}

#[derive(FromReflect, Reflect)]
pub struct YNYNWriteTrigger;
impl Trigger for YNYNWriteTrigger {
    type Param<'w, 's> = (Commands<'w, 's>, Query<'w, 's, Entity, Added<YNYNWriteComp>>);

    fn trigger(&self, _: Entity, added: &Self::Param<'_, '_>) -> bool {
        // println!("not triggered write");
        for ent in added.1.iter() {
            // println!("triggered wwrite");
            return true
        }
        return false
    }
}
#[derive(FromReflect, Reflect)]
pub struct YNYNWalkRTrigger;
impl Trigger for YNYNWalkRTrigger {
    type Param<'w, 's> = (Commands<'w, 's>, Query<'w, 's, Entity, Added<YNYNWalkRComp>>);

    fn trigger(&self, _: Entity, added: &Self::Param<'_, '_>) -> bool {
        for ent in added.1.iter() {
            return true
        }
        return false
    }
}
#[derive(FromReflect, Reflect)]
pub struct YNYNIdleRTrigger;
impl Trigger for YNYNIdleRTrigger {
    type Param<'w, 's> = (Commands<'w, 's>, Query<'w, 's, Entity, Added<YNYNIdleRComp>>);

    fn trigger(&self, _: Entity, added: &Self::Param<'_, '_>) -> bool {
        for ent in added.1.iter() {
            return true
        }
        return false
    }
}

#[derive(Bundle, Clone)]
pub struct AnimBundle {
    pub sprite: Handle<PxSprite>,
    // #[bundle]
    pub animation: PxAnimationBundle,
}

#[derive(Bundle, Clone)]
pub struct VoxAnimBundle {
    pub voxel_mesh: Handle<Mesh>,
    pub voxel_mat: Handle<StandardMaterial>,
    #[bundle]
    pub animation: VoxAnim,
}

#[derive(Bundle, Clone)]
pub struct VoxAnimBundle2 {
    pub voxel_mesh: Handle<Mesh>,
    #[bundle]
    pub animation: VoxAnim2,
}

#[derive(Component, Clone)]
pub struct VoxAnim {
    pub frame_duration: usize,
    pub frame_time_lapsed: usize,
    pub frame_count: usize,
    pub meshes: Vec<Handle<Mesh>>,
    pub materials: Vec<Handle<StandardMaterial>>,
}

#[derive(Component, Clone)]
pub struct VoxAnim2 {
    pub frame_duration: usize,
    pub frame_time_lapsed: usize,
    pub frame_count: usize,
    pub meshes: Vec<Handle<Mesh>>,
}
