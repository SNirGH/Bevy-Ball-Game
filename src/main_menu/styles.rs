use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub const BUTTON_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Px(200.0);
    node.height = Val::Px(80.0);
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node
};
