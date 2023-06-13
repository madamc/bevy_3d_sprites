use bevy::{prelude::*, ecs::system::SystemState};
use kayak_ui::{prelude::*, widgets::*};

use crate::{ImageAssets, ui::{CurrentText, CurrentTextState, message_current_percent_render, UIMessageWindow, CurrentTextBundle, PortraitAtlasState, Portrait, portrait_anim_render, PortaitBundle}, components::Person};
use crate::components::CommandCompleteIndicator;

use super::mando_queue::*;

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
            message_current_percent_render,
        );
        //Adding portrait render func
        widget_context.add_widget_system(
            Portrait::default().get_name(),
            widget_update::<Portrait, PortraitAtlasState>,
            portrait_anim_render,
        );
    
        rsx! {
            <KayakAppBundle>
                <NinePatchBundle
                    nine_patch={NinePatch {
                        handle: panel,
                        border: Edge::all(20.0),
                    }}
                    styles={KStyle {
                        width: StyleProp::Value(Units::Percentage(45.0)),
                        left: StyleProp::Value(Units::Percentage(27.5)),
                        top: StyleProp::Value(Units::Percentage(0.0)),
                        height: StyleProp::Value(Units::Percentage(35.0)),
                        ..KStyle::default()
                    }}
                >
                <ElementBundle
                styles={KStyle{
                    // layout_type: LayoutType::Row.into(),
                    // grid_rows: vec![Units::Stretch(1.0), Units::Stretch(2.0), Units::Stretch(5.0)].into(),
                    // grid_cols: vec![Units::Stretch(1.0), Units::Stretch(1.0)].into(),
                    ..default()
                }}
                >
                    <CurrentTextBundle />
                </ElementBundle>
                </NinePatchBundle>
            </KayakAppBundle>
            
        };
    
        commands.spawn((UIMessageWindow,UICameraBundle::new(widget_context)));
        state.apply(world);

        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;

}