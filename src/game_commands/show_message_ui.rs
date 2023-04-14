use bevy::{prelude::*, ecs::system::SystemState};
use kayak_ui::{prelude::*, widgets::*};

use crate::{ImageAssets, ui::{CurrentText, CurrentTextState, ui_message_current_percent_render, UIMessageWindow, CurrentTextBundle}, mandoqueue::MandoParam, components::Person};
use crate::components::CommandCompleteIndicator;

pub struct ShowMessageUICommand {

}

impl bevy::ecs::system::Command for ShowMessageUICommand {
    fn write(self, world: &mut World) {
        
        // A method of gettin the commands
        // let mut queue = CommandQueue::default();
        // let mut commands = Commands::new(&mut queue, &world);
        
        // Another method of getting the commands and this also allows for getting queries
        let mut state = SystemState::<(Commands, ParamSet<(
            Res<ImageAssets>,
            ResMut<FontMapping>,
            )>)>::new(world);
        let (mut commands, mut set) = state.get_mut(world);
        let panel = set.p0().panel.clone();
        let font = set.p0().kfont.clone();
        let mut fontMapper = set.p1();
        fontMapper.set_default(font);

        let mut widget_context = KayakRootContext::new();
        widget_context.add_plugin(KayakWidgetsContextPlugin);
        let parent_id = None;
        widget_context.add_widget_data::<CurrentText, CurrentTextState>();
        widget_context.add_widget_system(
            CurrentText::default().get_name(),
            widget_update::<CurrentText, CurrentTextState>,
            ui_message_current_percent_render,
        );
    
        rsx! {
            <KayakAppBundle>
                <NinePatchBundle
                    nine_patch={NinePatch {
                        handle: panel,
                        border: Edge::all(20.0),
                    }}
                    styles={KStyle {
                        width: StyleProp::Value(Units::Percentage(80.0)),
                        left: StyleProp::Value(Units::Percentage(10.0)),
                        height: StyleProp::Value(Units::Percentage(35.0)),
                        ..KStyle::default()
                    }}
                >
                    <CurrentTextBundle />
                </NinePatchBundle>
            </KayakAppBundle>
            
        };
    
        commands.spawn((UIMessageWindow,UICameraBundle::new(widget_context)));
        state.apply(world);

        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
    }
}

pub fn show_message_ui_cmd(mp: Vec<MandoParam>, world: &mut World, delta: u128, elapsed_time: u128) {
    
            // A method of gettin the commands
        // let mut queue = CommandQueue::default();
        // let mut commands = Commands::new(&mut queue, &world);
        
        // Another method of getting the commands and this also allows for getting queries
        let mut state = SystemState::<(Commands, ParamSet<(
            Res<ImageAssets>,
            ResMut<FontMapping>,
            )>)>::new(world);
        let (mut commands, mut set) = state.get_mut(world);
        let panel = set.p0().panel.clone();
        let font = set.p0().kfont.clone();
        let mut fontMapper = set.p1();
        fontMapper.set_default(font);

        let mut widget_context = KayakRootContext::new();
        widget_context.add_plugin(KayakWidgetsContextPlugin);
        let parent_id = None;
        widget_context.add_widget_data::<CurrentText, CurrentTextState>();
        widget_context.add_widget_system(
            CurrentText::default().get_name(),
            widget_update::<CurrentText, CurrentTextState>,
            ui_message_current_percent_render,
        );
    
        rsx! {
            <KayakAppBundle>
                <NinePatchBundle
                    nine_patch={NinePatch {
                        handle: panel,
                        border: Edge::all(20.0),
                    }}
                    styles={KStyle {
                        width: StyleProp::Value(Units::Percentage(80.0)),
                        left: StyleProp::Value(Units::Percentage(10.0)),
                        height: StyleProp::Value(Units::Percentage(35.0)),
                        ..KStyle::default()
                    }}
                >
                    <CurrentTextBundle />
                </NinePatchBundle>
            </KayakAppBundle>
            
        };
    
        commands.spawn((UIMessageWindow,UICameraBundle::new(widget_context)));
        state.apply(world);

        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;

}