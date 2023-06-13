


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