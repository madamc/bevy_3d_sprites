use bevy::{prelude::*, ecs::system::SystemState};

use crate::{ui::UIMessageWindow, game_commands::CommandCompleteIndicator, YNYNWalkLComp, YNYNIdleLComp, YNYNWalkRComp, YNYNIdleRComp, mandoqueue::{MandoParam, mpe}};

pub struct YNYNWalkLCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNWalkLCMD {
    fn write(self, world: &mut World) {
    
        // ynyn_walk_l_cmd(world, entity)
    }
}
fn ynyn_walk_l_cmd2(world: &mut World, entity: Entity) {
    let mut state = SystemState::<(Commands)>::new(world);
    let mut cmds = state.get_mut(world);
    cmds.entity(entity).insert(YNYNWalkLComp);
    state.apply(world);
    let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    cc.completed = true;
}

pub fn ynyn_walk_l_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    println!("Add walk");
    let mut entity = mpe(&mp[0]);

    let mut state = SystemState::<(Commands)>::new(world);
    let mut cmds = state.get_mut(world);
    cmds.entity(entity).insert(YNYNWalkLComp);
    state.apply(world);
    let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    cc.completed = true;
}

pub struct YNYNIdleLCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNIdleLCMD {
    fn write(self, world: &mut World) {
        let mut state = SystemState::<(Commands)>::new(world);
        let mut cmds = state.get_mut(world);
        cmds.entity(self.entity).insert(YNYNIdleLComp);
        state.apply(world);
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
        print!("scroomScroom");
    }
}

pub fn ynyn_idle_l_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let mut entity = mpe(&mp[0]);
    println!("Add idle");
    let mut state = SystemState::<(Commands)>::new(world);
    let mut cmds = state.get_mut(world);
    cmds.entity(entity).insert(YNYNIdleLComp);
    state.apply(world);
    let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    cc.completed = true;
}

pub struct YNYNWalkRCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNWalkRCMD {
    fn write(self, world: &mut World) {
        let mut state = SystemState::<(Commands)>::new(world);
        let mut cmds = state.get_mut(world);
        cmds.entity(self.entity).insert(YNYNWalkRComp);
        state.apply(world);
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;

    }
}

pub fn ynyn_walk_r_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let mut entity = mpe(&mp[0]);

    let mut state = SystemState::<(Commands)>::new(world);
    let mut cmds = state.get_mut(world);
    cmds.entity(entity).insert(YNYNIdleLComp);
    state.apply(world);
    let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    cc.completed = true;
}

pub struct YNYNIdleRCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNIdleRCMD {
    fn write(self, world: &mut World) {
        let mut state = SystemState::<(Commands)>::new(world);
        let mut cmds = state.get_mut(world);
        cmds.entity(self.entity).insert(YNYNIdleRComp);
        state.apply(world);
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
    }
}

pub fn ynyn_idle_r_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    let mut entity = mpe(&mp[0]);

    let mut state = SystemState::<(Commands)>::new(world);
    let mut cmds = state.get_mut(world);
    cmds.entity(entity).insert(YNYNIdleRComp);
    state.apply(world);
    let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
    cc.completed = true;
}