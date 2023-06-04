use crate::example_ui::assets::font_handle;
use crate::prelude::DialogueOption;
use bevy::prelude::*;

pub(crate) fn ui_setup_plugin(app: &mut App) {
    app.add_system(setup.on_startup());
}

#[derive(Debug, Default, Component)]
pub(crate) struct UiRootNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNode;

#[derive(Debug, Default, Component)]
pub(crate) struct OptionsNode;

fn setup(mut commands: Commands) {
    // root node
    commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
            ..default()
        }, UiRootNode))
        .insert(Visibility::Hidden)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(DIALOG_WIDTH)),
                        min_size: Size::height(Val::Px(150.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::all(Val::Px(TEXT_BORDER)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Dialog itself
                    parent.spawn((
                        TextBundle::from_section(String::new(), text_style::standard()).with_style(style::standard()),
                        DialogueNode,
                        Label,
                    ));
                }).with_children(|parent| {
                    // Options
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                // display: Display::None,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::FlexStart,
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        }, OptionsNode)).with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_sections([
                                        TextSection { value: "1: ".to_string(), style: text_style::option_id() }, TextSection { value: "Do stuffs".to_string(), style: text_style::option_text() },
                                ]).with_style(style::options()),
                                Label,
                            ));
                            parent.spawn((
                                TextBundle::from_sections([
                                        TextSection { value: "2: ".to_string(), style: text_style::option_id() }, TextSection { value: "Do MORE Stuffs! Lorem ipsum dolor sit amet, consetetur sadipscing elitr!!".to_string(),
                                        style: text_style::option_text() },
                                ]).with_style(style::options()),
                                Label,
                            ));
                        });
            });
        });
}

pub(crate) fn create_dialog_text<'a>(
    name: impl Into<Option<&'a str>>,
    text: impl Into<String>,
    invisible: impl Into<String>,
) -> Text {
    let mut sections = Vec::new();
    if let Some(name) = name.into() {
        sections.push(TextSection {
            value: format!("{name}: "),
            style: text_style::name(),
        });
    }
    sections.extend([
        TextSection {
            value: text.into(),
            style: text_style::standard(),
        },
        TextSection {
            value: invisible.into(),
            style: TextStyle {
                color: Color::NONE,
                ..text_style::standard()
            },
        },
    ]);
    Text::from_sections(sections)
}

pub(crate) fn create_options<'a, O>(options: O) -> impl Iterator<Item = TextBundle> + 'a
where
    O: IntoIterator<Item = &'a DialogueOption>,
    <O as IntoIterator>::IntoIter: 'a,
{
    options.into_iter().enumerate().map(|(i, option)| {
        let sections = [
            TextSection {
                value: format!("{}: ", i + 1),
                style: text_style::option_id(),
            },
            TextSection {
                value: option.line.text.clone(),
                style: text_style::option_text(),
            },
        ];
        TextBundle::from_sections(sections).with_style(style::options())
    })
}

const DIALOG_WIDTH: f32 = 800.0 * 0.7;
const TEXT_BORDER: f32 = 10.0;

mod style {
    use super::*;
    pub(crate) fn standard() -> Style {
        Style {
            max_size: Size {
                width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER),
                height: Val::Undefined,
            },
            ..default()
        }
    }
    pub(crate) fn options() -> Style {
        Style {
            margin: UiRect::horizontal(Val::Px(TEXT_BORDER)),
            max_size: Size {
                width: Val::Px(DIALOG_WIDTH - 4.0 * TEXT_BORDER),
                height: Val::Undefined,
            },
            ..default()
        }
    }
}

mod text_style {
    use super::*;
    pub(crate) fn standard() -> TextStyle {
        TextStyle {
            font: font_handle::MEDIUM.typed(),
            font_size: 20.0,
            color: Color::BLACK,
        }
    }
    pub(crate) fn name() -> TextStyle {
        TextStyle {
            font: font_handle::BOLD.typed(),
            ..standard()
        }
    }

    pub(crate) fn option_id() -> TextStyle {
        TextStyle {
            font: font_handle::BOLD.typed(),
            ..option_text()
        }
    }

    pub(crate) fn option_text() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            color: Color::DARK_GRAY,
            ..standard()
        }
    }
}
