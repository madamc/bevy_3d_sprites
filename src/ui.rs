use std::str::SplitWhitespace;

use bevy::app::AppExit;
use bevy::prelude::*; 
use kayak_ui::prelude::{widgets::*, *};
use substring::Substring;

use crate::game_setup::load_assets::ImageAssets;
use crate::components::CommandCompleteIndicator;
use crate::game_commands::{GameCommandsExt, mando_queue::{mps}};

#[derive(Component)]
pub struct MainMenuWidget;

#[derive(Component)]
pub struct UIMessageWindow;

#[derive(Component, Default, PartialEq, Clone)]
pub struct CurrentText;

impl Widget for CurrentText {}

#[derive(Component, Default, PartialEq, Clone)]
pub struct CurrentTextState {
    pub chars: u128,
    pub text: String,
    pub iter: i8,
    //elapsed time for iter
    pub elapsed_time: u128,
}

#[derive(Component, Default, PartialEq, Clone)]
pub struct Portrait;

impl Widget for Portrait {}

#[derive(Component, Default, PartialEq, Clone)]
pub struct PortraitAtlasState {
    // pub chars: u128,
    // pub text: String,
    pub counter: i8,
    pub index: i8,
    //elapsed time for iter
    pub elapsed_time: u128,
}

#[derive(Bundle)]
pub struct PortaitBundle {
    pub portrait: Portrait,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
    // pub type_writer: TypeWriterText,
    // pub colors: TypeWriterTextColors
}

impl Default for PortaitBundle {
    fn default() -> Self {
        Self {
            portrait: Portrait::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: Portrait::default().get_name(),
            // type_writer: TypeWriterText::default(),
            // colors: TypeWriterTextColors::default()
        }
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct ButtonState2 {
    pub hovering: bool,
    pub clicked: bool,
}

//relook at the typewriter stuff when you can render multiple styles in a text widget.

#[derive(Bundle)]
pub struct CurrentTextBundle {
    pub percentage: CurrentText,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
    // pub type_writer: TypeWriterText,
    // pub colors: TypeWriterTextColors
}

impl Default for CurrentTextBundle {
    fn default() -> Self {
        Self {
            percentage: CurrentText::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            widget_name: CurrentText::default().get_name(),
            // type_writer: TypeWriterText::default(),
            // colors: TypeWriterTextColors::default()
        }
    }
}



// impl Default for TypeWriterText {
//     fn default() -> Self {
//         Self {
//             cursor_index: 0,
//             delay: 0.01,
//             timer: 0.01,
//             cursor_color: Color::RED
//         }
//     }
// }

// impl Default for TypeWriterTextColors {
//     fn default() -> Self {
//         Self {
//             colors: vec![]
//         }
//     }
// }

pub fn startup_txt(
    mut commands: Commands,
    mut set: ParamSet<(
        ResMut<FontMapping>,
        Res<AssetServer>,
        Query<Entity, With<UIMessageWindow>>,
    )>,
) {
    
}
// Add logic to take a string and seperate it up into different messages. I don't want to clip messages (no cutting off mid-word), and I don't want messages to bleed out of the message panel.
// this returns an array of strings that span the width provided. It doesn't take into account the height of a message
pub fn create_message(text: &str, line_width: i32) -> Vec<String> {
    let mut owned_string = text.to_owned();
    let str_col_ws: SplitWhitespace = owned_string.split_whitespace();
    // let str_col: Vec<String> = owned_string.split_whitespace().map(str::to_string).collect();
    let mut str_vec: Vec<String> = vec![];
    // adjust the line_limit to be whatever suits the look best, depending on the font, this can vary a lot.
    // the final number (maybe?) should match the font size
    let line_limit = (line_width as f32 * 0.75 ) as i32 /28;
    let mut line_counter = 0;

    for (idx,bit) in str_col_ws.into_iter().enumerate() {
        let mut owned_bit = (bit as &str).to_owned();
        if ((bit as &str).substring(0,((bit as &str).len() as usize)/2).len()) + line_counter >= line_limit as usize || ((bit as &str).len() as usize) + line_counter >= line_limit as usize {
            // owned_bit = str_col[idx].to_owned();
            line_counter = 0;
            // println!("Wowy {}, {}, {}", owned_bit, (bit as &str).len(), ((bit as &str).len() as usize)/2);
            if let Some(mut last_word) = str_vec.pop() {
                
                line_counter = 0;
                line_counter += ((bit as &str).len() as usize);
                line_counter += 1;
                last_word.push_str("\n");
                str_vec.push(last_word);
            }
        // }
        // else if ((bit as &str).len() as usize) + line_counter >= line_limit as usize{
        //     line_counter = 0;
        //     owned_bit.push_str("\n");
            
        } else {
            // add the length of the letter to the line counter and then add one more for the space
            line_counter += ((bit as &str).len() as usize);
            line_counter += 1;
        }
        
        str_vec.push(owned_bit);
    }
    message_to_str_vec(str_vec)
}

pub fn message_to_str_vec(str_vec: Vec<String>) -> Vec<String> {
    let mut owned = "".to_owned();
    let mut line_vec: Vec<String> = Vec::new();
    let last_str = str_vec.last().unwrap().clone();
    for str in str_vec {
        owned.push_str(&str);
        if !(str.ends_with("\n")) {
            owned.push_str(" ");
        }
        else {
            line_vec.push(owned);
            owned = "".to_owned();
        }
        
        if str == last_str {
            line_vec.push(owned);
            owned = "".to_owned();
        }
    }
    
    return line_vec;
}

pub fn message_to_str(sws: Vec<String>) -> String {
    let mut owned = "".to_owned();
    for bit in sws {
        owned.push_str(&bit);
        if !(bit.ends_with("\n")) {
            owned.push_str(" ");
        }
        
    }
    
    return owned;
}

const FONT_SIZE: f32 = 20.0;

pub fn message_current_percent_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    mut set: ParamSet<(
        Query<&mut CurrentTextState>,
        &World,
        ResMut<CommandCompleteIndicator>,
    )>
) -> bool {

    let state_entity = widget_context.use_state(&mut commands, entity, CurrentTextState::default());
  
    if let Ok(mut current_text) = set.p0().get_mut(state_entity) {
        
        let parent_id = Some(entity);

        rsx! {
            // <ElementBundle>
            // <ElementBundle
            // styles={KStyle{
            //     width: StyleProp::Value(Units::Stretch(7.)),
            //     ..default()
            // }}
            // >
                <TextWidgetBundle
                    text={
                        TextProps {
                            content: format!("{}", current_text.text.substring(0, current_text.chars as usize)), 
                            //ideal is 32
                            size: FONT_SIZE,//24.0,
                            //ideal is 40
                            line_height: Some(2.0 * FONT_SIZE),
                            ..Default::default()
                        }}
                    styles={ KStyle { 
                        color: StyleProp::Value(Color::Rgba { red: 0.277, green: 0.281, blue: 0.3, alpha: 1.0 }), 
                        left: StyleProp::Value(Units::Pixels(20.)),//(31.0)),
                        width: StyleProp::Value(Units::Percentage(100.)),
                        // right: StyleProp::Value(Units::Percentage(10.)),
                        // font_size: StyleProp::Value(30.0),
                        ..Default::default()    
                    }}
                    computed_styles={ ComputedStyles(KStyle { 
                        color: StyleProp::Value(Color::Rgba { red: 0.277, green: 0.281, blue: 0.3, alpha: 1.0 }), 
                        // left: StyleProp::Value(Units::Percentage(10.)),//(31.0)),
                        width: StyleProp::Value(Units::Percentage(100.)),
                        // right: StyleProp::Value(Units::Percentage(10.)),
                        // font_size: StyleProp::Value(30.0),
                        ..Default::default()    
                    })}                      
                />
            // </ElementBundle>
        };


    }

    true
}

pub fn portrait_message_current_percent_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    mut set: ParamSet<(
        Query<&mut CurrentTextState>,
        &World,
        ResMut<CommandCompleteIndicator>,
    )>
) -> bool {

    let state_entity = widget_context.use_state(&mut commands, entity, CurrentTextState::default());
  
    if let Ok(mut current_text) = set.p0().get_mut(state_entity) {
        
        let parent_id = Some(entity);

        rsx! {
            <ElementBundle>
                <TextWidgetBundle
                    text={
                        TextProps {
                            content: format!("{}", current_text.text.substring(0, current_text.chars as usize)), 
                            //ideal is 32
                            size: 14.0,
                            //ideal is 40
                            line_height: Some(28.0),
                            ..Default::default()
                        }}
                    styles={ KStyle { 
                        color: StyleProp::Value(Color::Rgba { red: 0.277, green: 0.281, blue: 0.3, alpha: 1.0 }), 
                        left: StyleProp::Value(Units::Percentage(-38.)),//(31.0)),
                        // font_size: StyleProp::Value(30.0),
                        ..Default::default()    
                    }}                    
                />
            </ElementBundle>
        };


    }

    true
}

pub fn portrait_anim_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    mut set: ParamSet<(
        Query<&mut PortraitAtlasState>,
        &World,
        Res<ImageAssets>,
    )>
) -> bool {
    // let mq = set.p1().get_resource::<MandoQueue>().unwrap();

    let state_entity = widget_context.use_state(&mut commands, entity, PortraitAtlasState::default());
    let image = set.p2().monk_face_talk_png.clone();
    if let Ok(mut portrait) = set.p0().get_mut(state_entity) {
        
        let parent_id = Some(entity);

        
        let atlas = TextureAtlas::from_grid(
            image.clone(),
            Vec2 { x: 42., y: 64. },
            8,
            1,
            None,
            None,
        );
        let atlas_styles = KStyle {
            position_type: StyleProp::Value(KPositionType::ParentDirected),
            // width: StyleProp::Value(Units::Percentage(100.0)),
            // width: StyleProp::Value(Units::Percentage(100.0)),//Pixels(164.0)),
            // height: StyleProp::Value(Units::Percentage(100.0)),//Pixels(244.0)),
            left: StyleProp::Value(Units::Percentage(5.0)),
            top: StyleProp::Value(Units::Percentage(15.0)),
            // right: StyleProp::Value(Units::Percentage(3.0)),
            bottom: StyleProp::Value(Units::Percentage(15.0)),
            ..KStyle::default()
        };
        
        portrait.counter += 1;
        if portrait.counter > 100 {
            // println!("Kerblum {}", face_index);
            portrait.index += 1;
            if portrait.index as usize >= atlas.textures.len() {
                // println!("Perblum {}", face_index);
                portrait.index = 0;
            }
            portrait.counter = 0;
            // println!("Derblum {}", face_index);
        }
        let mut face_index = portrait.index;
        // let face_index = portrait.elapsed_time as usize  % atlas.textures.len();
        // let face_index = 0;
        
        let rect = atlas.textures[face_index as usize];
        let portrait_position = rect.min;
        let portrait_size = rect.max - rect.min;

        rsx! {
            // <ElementBundle>
            // <ElementBundle
            // styles={KStyle{
            //     width: StyleProp::Value(Units::Stretch(0.5)),
            //     ..default()
            // }}
            // >
                <TextureAtlasBundle
                atlas={TextureAtlasProps {
                    handle: image.clone(),
                    position: portrait_position,
                    tile_size: portrait_size,
                }}
                styles={atlas_styles.clone()}
                computed_styles= {ComputedStyles(atlas_styles.clone())}
                />
            // </ElementBundle>
        };


    }

    true
}



#[derive(Default, Clone, PartialEq, Component)]
pub struct MenuButton {
    pub text: String,
}

impl Widget for MenuButton {}

#[derive(Bundle)]
pub struct MenuButtonBundle {
    pub button: MenuButton,
    pub styles: KStyle,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            styles: KStyle {
                bottom: Units::Pixels(20.0).into(),
                cursor: KCursorIcon(CursorIcon::Hand).into(),
                ..Default::default()
            },
            on_event: OnEvent::default(),
            widget_name: MenuButton::default().get_name(),
        }
    }
}

pub fn menu_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    assets: Res<ImageAssets>,
    font: ResMut<FontMapping>,
    menu_button_query: Query<&MenuButton>,
    state_query: Query<&ButtonState2>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState2 { hovering: false, clicked: false });

    let button_text = menu_button_query.get(entity).unwrap().text.clone();
    let button_image = assets.panel_btn.clone();
    let button_image_hover = assets.panel_btn_hov.clone();
    let button_image_clicked = assets.panel_btn_clk.clone();

    let on_event = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState2>| {
            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    }
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    }
                    EventType::MouseDown(..) => {
                        button.clicked = true;
                    }
                    EventType::MouseUp(..) => {
                        button.clicked = false;
                    }
                    _ => {}
                }
            }
            (event_dispatcher_context, event)
        },
    );

    if let Ok(button_state) = state_query.get(state_entity) {
        let button_image_handle = if button_state.clicked {
            button_image_clicked
        } else if button_state.hovering {
            button_image_hover
        } else {
            button_image
        };

        let parent_id = Some(entity);
        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: button_image_handle,
                    border: Edge::all(10.0),
                }}
                styles={KStyle {
                    width: Units::Stretch(1.0).into(),
                    height: Units::Pixels(40.0).into(),
                    bottom: Units::Pixels(30.0).into(),
                    left: Units::Pixels(50.0).into(),
                    right: Units::Pixels(50.0).into(),
                    ..KStyle::default()
                }}
                on_event={on_event}
            >
                <TextWidgetBundle
                    styles={KStyle {
                        top: Units::Percentage(-30.0).into(),
                        // bottom: Units::Stretch(1.0).into(),
                        ..Default::default()
                    }}
                    text={TextProps {
                        alignment: Alignment::Middle,
                        content: button_text,
                        size: 24.0,
                        ..Default::default()
                    }}
                />
            </NinePatchBundle>
        };
    }
    true
}

#[derive(Default, Resource)]
pub struct PreloadResource {
    images: Vec<Handle<Image>>,
}
