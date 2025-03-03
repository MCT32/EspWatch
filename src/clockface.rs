use core::fmt::Debug;

use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::{DrawTarget, Point}, text::{Alignment, Baseline, Text, TextStyleBuilder}, Drawable};
use no_std_strings::{str8, str_format};


use crate::font;

pub fn render_clockface<D: DrawTarget<Color = BinaryColor>>(display: &mut D, hour: u32, minute: u32, second: u32) where <D as DrawTarget>::Error: Debug {
    let main_str = str_format!(str8, "{:0>2}:{:0>2}", hour, minute);
    let main_style = MonoTextStyle::new(&font::SEVEN_SEG_FONT, BinaryColor::On);

    let seconds_str = str_format!(str8, "{:0>2}", second);
    let seconds_style = MonoTextStyle::new(&font::SEVEN_SEG_SMALL_FONT, BinaryColor::On);

    Text::with_text_style(
        main_str.as_str(),
        Point::new((display.bounding_box().size.width / 2).try_into().unwrap(), 0),
        main_style,
        TextStyleBuilder::new()
            .baseline(Baseline::Top)
            .alignment(Alignment::Center)
            .build()
    )
    .draw(display).unwrap();

    Text::with_alignment(
        seconds_str.as_str(),
        display.bounding_box().bottom_right().unwrap(),
        seconds_style,
        Alignment::Right,
    )
    .draw(display).unwrap();
}
