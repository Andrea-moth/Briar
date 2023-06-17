use std::{fmt::format, default};

use bevy::{
    prelude::{
        App,
        Plugin,
        With, 
        Res, 
        Commands,
        NodeBundle,
        Style,
        Size,
        Val,
        UiRect, 
        Camera2dBundle, 
        BuildChildren, 
        Component, 
        Query, 
        Text, 
        TextBundle,
        TextSection,
        TextStyle, 
        SystemSet, AssetServer, Handle, EventWriter, EventReader, Children, Entity, DespawnRecursiveExt,
    }, 
    ui::{Overflow, Node}, input::mouse::{MouseWheel, MouseScrollUnit}
};
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};

use crate::engine::{assets::*, GameState};

pub enum StoryEvent {
    ClearStory,
    NewSection(String),
    Set(String)
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32
}

#[derive(Component)]
pub struct Story;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::InGame)
                .with_collection::<OpenDyslexicFonts>()
        )
        .add_event::<StoryEvent>()
        .add_state(GameState::Loading)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(ui_setup))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(story)
                //.with_system(story_system)
                .with_system(story_scroll)
        );
    }
}

fn ui_setup(
    mut commands: Commands,
    font_assets: Res<OpenDyslexicFonts>
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn(framing())
         .with_children(|parent|{
            parent.spawn(wireframe())
                .with_children(|parent| {
                    parent.spawn(story_space_border())
                        .with_children(|parent| {
                            parent.spawn((story_space(), Story))
                                .with_children(|parent| {
                                    parent.spawn(story_text(font_assets));
                                });
                        });

                    parent.spawn(story_image_framing())
                        .with_children(|parent| {
                            parent.spawn(story_image());
                        });

                    parent.spawn(quick_menu_framing())
                        .with_children(|parent| {
                            parent.spawn(quick_menu());
                        });
                });
        });

}

fn framing() -> NodeBundle{
    NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100.0),
                Val::Percent(100.0),
            ),
            padding: UiRect::all(Val::Percent(1.0)),
            ..Default::default()
        },
        background_color: BACKGROUND_COLOUR.into(),
        ..Default::default()
    }
}

fn wireframe() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100.0),
                Val::Percent(100.0),
            ),
            ..Default::default()
        },
        background_color: WIREFRAME_COLOUR.into(),
        ..Default::default()
    }
}

fn story_space_border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size{
                width: Val::Percent(35.0),
                height: Val::Percent(100.0)
            },
            padding: UiRect{
                top: Val::Percent(0.3),
                bottom: Val::Percent(0.3),
                right: Val::Percent(0.15),
               left: Val::Percent(0.3)
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn story_space() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size{
                width: Val::Percent(100.0), 
                height: Val::Percent(100.0)
            },
            padding: UiRect{ 
                left: Val::Percent(2.0),
                right: Val::Percent(2.0),
                top: Val::Percent(1.0),
                bottom: Val::Percent(1.0)
            },
            ..Default::default()
        },
        background_color: BACKGROUND_COLOUR.into(),
        ..Default::default()
    }
}

fn story_text(font_assets: Res<OpenDyslexicFonts>) -> TextBundle {
    TextBundle::from_section(
        "Story",
        TextStyle {
            font: font_assets.regular.clone(),
            font_size: 30.0,
            color: TEXTCOLOUR,
        }
    )
    .with_style(
        Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0)
            },
            overflow: Overflow::Visible,
            ..Default::default()
    })
}

fn story_image_framing() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(55.0),
                height: Val::Percent(100.0)
            },
            padding: UiRect{
                top: Val::Percent(0.3),
                bottom: Val::Percent(0.3),
                right: Val::Percent(0.15),
                left: Val::Percent(0.15)
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn story_image() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size { 
                width: Val::Percent(100.0),
                height: Val::Percent(100.0) 
            },
            ..Default::default()
        },
        background_color: BACKGROUND_COLOUR.into(),
        ..Default::default()
    }
}

fn quick_menu_framing() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(10.0),
                height: Val::Percent(100.0)
            },
            padding: UiRect{
                top: Val::Percent(0.3),
                bottom: Val::Percent(0.3),
                right: Val::Percent(0.3),
                left: Val::Percent(0.15)
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn quick_menu() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0)
            },
            ..Default::default()
        },
        background_color: BACKGROUND_COLOUR.into(),
        ..Default::default()
    }
}

fn story(
    mut ew: EventWriter<StoryEvent>
) {
    ew.send(StoryEvent::NewSection("Hello dere!".to_string()));
}

fn story_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Children),
        With<Story>
    >,
    mut el: EventReader<StoryEvent>
) {
    for event in el.iter() {
        for (item, children) in &mut query {
            match event {
                StoryEvent::ClearStory => {commands.entity(item).despawn_recursive()},
                StoryEvent::NewSection(section) => {commands.entity(item).add_child(child)},
                StoryEvent::Set(section) => {},
            };
        }
    }
}

fn story_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}