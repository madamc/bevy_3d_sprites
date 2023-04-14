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
pub struct YNYNIdleRState;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct YNYNWalkRState;


#[derive(Clone, Component)]
pub struct YNYNWalkLComp;

#[derive(Clone, Component)]
pub struct YNYNIdleLComp;

#[derive(Clone, Component)]
pub struct YNYNWalkRComp;

#[derive(Clone, Component)]
pub struct YNYNIdleRComp;


#[derive(FromReflect, Reflect)]
pub struct YNYNWalkLTrigger;
impl Trigger for YNYNWalkLTrigger {
    type Param<'w, 's> = (Commands<'w, 's>, Query<'w, 's, Entity, Added<YNYNWalkLComp>>);

    fn trigger(&self, _: Entity, added: &Self::Param<'_, '_>) -> bool {
    
        for ent in added.1.iter() {
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
        for ent in added.1.iter() {
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
    #[bundle]
    pub animation: PxAnimationBundle,
}