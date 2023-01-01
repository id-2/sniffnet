//! Module defining the constants used for aesthetic purposes (colors, borders...)

use crate::{get_colors, StyleType};
use iced::{Color, Font};
use plotters::style::RGBColor;

use crate::structs::colors::{to_rgb_color, Colors};

pub const COLOR_CHART_MIX: f64 = 1.0;

// night theme
const PRIMARY_NIGHT: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const SECONDARY_NIGHT: Color = Color {
    r: 0.7,
    g: 0.35,
    b: 0.0,
    a: 1.0,
};
const BUTTONS_NIGHT: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};
pub const NIGHT_STYLE: Colors = Colors {
    primary: PRIMARY_NIGHT,
    secondary: SECONDARY_NIGHT,
    buttons: BUTTONS_NIGHT,
    incoming: SECONDARY_NIGHT,
    outgoing: SECONDARY_DAY,
    text_headers: Color::BLACK,
    text_body: Color::WHITE,
    round_borders: Color::BLACK,
};

// day theme
const PRIMARY_DAY: Color = Color::WHITE;
const SECONDARY_DAY: Color = Color {
    r: 0.0,
    g: 0.35,
    b: 0.7,
    a: 1.0,
};
const BUTTONS_DAY: Color = Color {
    r: 0.8,
    g: 0.8,
    b: 0.8,
    a: 1.0,
};
pub const DAY_STYLE: Colors = Colors {
    primary: PRIMARY_DAY,
    secondary: SECONDARY_DAY,
    buttons: BUTTONS_DAY,
    incoming: SECONDARY_DAY,
    outgoing: SECONDARY_NIGHT,
    text_headers: Color::WHITE,
    text_body: Color::BLACK,
    round_borders: Color::BLACK,
};

// try theme
const PRIMARY_TRY: Color = Color {
    r: 28.0 / 255.0,
    g: 49.0 / 255.0,
    b: 94.0 / 255.0,
    a: 1.0,
};
const SECONDARY_TRY: Color = Color {
    r: 34.0 / 255.0,
    g: 124.0 / 255.0,
    b: 112.0 / 255.0,
    a: 1.0,
};
const BUTTONS_TRY: Color = Color {
    r: 48.0 / 255.0,
    g: 71.0 / 255.0,
    b: 94.0 / 255.0,
    a: 1.0,
};
const OUTGOING_TRY: Color = Color {
    r: 230.0 / 255.0,
    g: 226.0 / 255.0,
    b: 195.0 / 255.0,
    a: 1.0,
};
pub const TRY_STYLE: Colors = Colors {
    primary: PRIMARY_TRY,
    secondary: SECONDARY_TRY,
    buttons: BUTTONS_TRY,
    incoming: SECONDARY_TRY,
    outgoing: OUTGOING_TRY,
    text_headers: Color::BLACK,
    text_body: Color::WHITE,
    round_borders: Color::BLACK,
};

// red theme
const SECONDARY_RED: Color = Color {
    r: 127.0 / 255.0,
    g: 102.0 / 255.0,
    b: 157.0 / 255.0,
    a: 1.0,
};
const PRIMARY_RED: Color = Color {
    r: 245.0 / 255.0,
    g: 245.0 / 255.0,
    b: 220.0 / 255.0,
    a: 1.0,
};
const BUTTONS_RED: Color = Color {
    r: 222.0 / 255.0,
    g: 186.0 / 255.0,
    b: 206.0 / 255.0,
    a: 1.0,
};
const OUTGOING_RED: Color = Color {
    r: 90.0 / 255.0,
    g: 164.0 / 255.0,
    b: 105.0 / 255.0,
    a: 1.0,
};
pub const RED_STYLE: Colors = Colors {
    primary: PRIMARY_RED,
    secondary: SECONDARY_RED,
    buttons: BUTTONS_RED,
    incoming: SECONDARY_RED,
    outgoing: OUTGOING_RED,
    text_headers: Color::WHITE,
    text_body: Color::BLACK,
    round_borders: Color::BLACK,
};

// almond theme
const PRIMARY_ALMOND: Color = Color {
    r: 139.0 / 255.0,
    g: 126.0 / 255.0,
    b: 116.0 / 255.0,
    a: 1.0,
};
const SECONDARY_ALMOND: Color = Color {
    r: 54.0 / 255.0,
    g: 39.0 / 255.0,
    b: 6.0 / 255.0,
    a: 1.0,
};
const BUTTONS_ALMOND: Color = OUTGOING_ALMOND;
const INCOMING_ALMOND: Color = SECONDARY_ALMOND;
const OUTGOING_ALMOND: Color = Color {
    r: 241.0 / 255.0,
    g: 211.0 / 255.0,
    b: 179.0 / 255.0,
    a: 1.0,
};
pub const ALMOND_STYLE: Colors = Colors {
    primary: PRIMARY_ALMOND,
    secondary: SECONDARY_ALMOND,
    buttons: BUTTONS_ALMOND,
    incoming: INCOMING_ALMOND,
    outgoing: OUTGOING_ALMOND,
    text_headers: Color::WHITE,
    text_body: Color::BLACK,
    round_borders: SECONDARY_ALMOND,
};

// gui Text fonts
pub const COURIER_PRIME: Font = Font::External {
    name: "CourierPrime",
    bytes: include_bytes!("../../fonts/CourierPrime.ttf"),
};
pub const COURIER_PRIME_BOLD: Font = Font::External {
    name: "CourierPrimeBold",
    bytes: include_bytes!("../../fonts/CourierPrimeBold.ttf"),
};
pub const COURIER_PRIME_ITALIC: Font = Font::External {
    name: "CourierPrimeItalic",
    bytes: include_bytes!("../../fonts/CourierPrimeItalic.ttf"),
};
pub const COURIER_PRIME_BOLD_ITALIC: Font = Font::External {
    name: "CourierPrimeBoldItalic",
    bytes: include_bytes!("../../fonts/CourierPrimeBoldItalic.ttf"),
};

pub fn get_font(style: StyleType) -> Font {
    match to_rgb_color(get_colors(style).text_body) {
        RGBColor(255, 255, 255) => COURIER_PRIME,
        _ => COURIER_PRIME_BOLD,
    }
}

// gui charts fonts
pub const NOTOSANS: Font = Font::External {
    name: "Notosans",
    bytes: include_bytes!("../../fonts/notosans-regular.ttf"),
};
pub const NOTOSANS_BOLD: Font = Font::External {
    name: "NotosansBold",
    bytes: include_bytes!("../../fonts/notosans-bold.ttf"),
};

//font to display icons
pub const ICONS: Font = Font::External {
    name: "icons",
    bytes: include_bytes!("../../fonts/icons.ttf"),
};

// font sizes
pub const FONT_SIZE_FOOTER: u16 = 14;
pub const FONT_SIZE_BODY: u16 = 16;
pub const FONT_SIZE_SUBTITLE: u16 = 19;
pub const FONT_SIZE_TITLE: u16 = 22;

// border styles
pub const BORDER_WIDTH: f32 = 2.0;
pub const BORDER_WIDTH_TABS: f32 = 3.3;
pub const CHARTS_LINE_BORDER: u32 = 1;
pub const BORDER_ROUNDED_RADIUS: f32 = 15.0;
pub const BORDER_BUTTON_RADIUS: f32 = 180.0;

// body proportions
pub const HEIGHT_HEADER: u16 = 2;
pub const HEIGHT_BODY: u16 = 12;
pub const HEIGHT_FOOTER: u16 = 1;
