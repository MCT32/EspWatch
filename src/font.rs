use embedded_graphics::{image::ImageRaw, mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont}, prelude::Size};
use lazy_static::lazy_static;
use tinybmp::RawBmp;

const SEVEN_SEG_DATA: &[u8] = include_bytes!("assets/seven-segment-font.bmp");
const SEVEN_SEG_GLYPHMAP: StrGlyphMapping<'_> = StrGlyphMapping::new("0123456789:", 0);
lazy_static! {
    static ref SEVEN_SEG_BMP: RawBmp<'static> = RawBmp::from_slice(SEVEN_SEG_DATA).unwrap();
    pub static ref SEVEN_SEG_FONT: MonoFont<'static> = MonoFont {
        image: ImageRaw::new(&SEVEN_SEG_BMP.image_data(), 256),
        glyph_mapping: &SEVEN_SEG_GLYPHMAP,
        character_size: Size::new(22, 40),
        character_spacing: 4,
        baseline: 7,
        underline: DecorationDimensions::default_underline(40),
        strikethrough: DecorationDimensions::default_strikethrough(40),
    };
}

const SEVEN_SEG_SMALL_DATA: &[u8] = include_bytes!("assets/seven-segment-font-small.bmp");
const SEVEN_SEG_SMALL_GLYPHMAP: StrGlyphMapping<'_> = StrGlyphMapping::new("0123456789", 0);
lazy_static! {
    static ref SEVEN_SEG_SMALL_BMP: RawBmp<'static> = RawBmp::from_slice(SEVEN_SEG_SMALL_DATA).unwrap();
    pub static ref SEVEN_SEG_SMALL_FONT: MonoFont<'static> = MonoFont {
        image: ImageRaw::new(&SEVEN_SEG_SMALL_BMP.image_data(), 90),
        glyph_mapping: &SEVEN_SEG_SMALL_GLYPHMAP,
        character_size: Size::new(9, 18),
        character_spacing: 1,
        baseline: 18,
        underline: DecorationDimensions::default_underline(40),
        strikethrough: DecorationDimensions::default_strikethrough(40),
    };
}
