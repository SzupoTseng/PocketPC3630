//! Proportional, anti-aliased text for guest GDI calls.
//!
//! Every GDI text call used to draw and measure with the 6x8 bitmap font
//! in [`crate::font`], whatever font the program selected. Pocket PC draws
//! Tahoma at the size asked for; next to Device Emulator, EverQuest's
//! "Tap here to create a character." came out small, blocky and with the
//! wrong line spacing. DejaVu Sans (see `data/fonts/README.md`)
//! stands in for Tahoma, which may not be redistributed.
//!
//! Drawing and measuring go through the same [`TextFont`], so a program
//! that lays text out with `GetTextExtentExPointW` or `DT_CALCRECT` gets the
//! widths it will see drawn.

use ab_glyph::{point, Font as _, FontRef, PxScale, ScaleFont};

use crate::gdi::Surface;

static REGULAR: &[u8] = include_bytes!("../data/fonts/DejaVuSans-subset.ttf");
static BOLD: &[u8] = include_bytes!("../data/fonts/DejaVuSans-Bold-subset.ttf");
static MONO: &[u8] = include_bytes!("../data/fonts/DejaVuSansMono-subset.ttf");

/// Em size, in pixels, of a font that does not say how big it is: the
/// Pocket PC system font, Tahoma 8 pt at 96 dpi.
pub const DEFAULT_PX: f32 = 11.0;

/// A font as a DC uses it: its pixel size and whether it is bold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextFont {
    /// Em height in pixels.
    pub px: f32,
    pub bold: bool,
    /// Fixed pitch: the program asked for a monospaced face.
    pub mono: bool,
}

fn face_of(bold: bool, mono: bool) -> FontRef<'static> {
    let data = if mono {
        MONO
    } else if bold {
        BOLD
    } else {
        REGULAR
    };
    FontRef::try_from_slice(data).expect("embedded font parses")
}

impl TextFont {
    /// The font a `LOGFONT` asks for. A negative `lfHeight` is the
    /// character (em) height; a positive one is the cell height, ascent
    /// plus descent; zero means the default size. `lfWeight` of 600 or
    /// more is bold.
    pub fn from_logfont(height: i32, weight: i32) -> Self {
        Self::from_logfont_pitch(height, weight, false)
    }

    /// [`Self::from_logfont`] for a font that may be fixed-pitch.
    pub fn from_logfont_pitch(height: i32, weight: i32, mono: bool) -> Self {
        let bold = weight >= 600;
        let f = face_of(bold, mono);
        // Line height (ascent - descent) per em.
        let cell_per_em =
            (f.ascent_unscaled() - f.descent_unscaled()) / f.units_per_em().unwrap_or(2048.0);
        let px = match height {
            h if h < 0 => (-h) as f32,
            h if h > 0 => h as f32 / cell_per_em,
            _ => DEFAULT_PX,
        };
        Self {
            px: px.clamp(4.0, 96.0),
            bold,
            mono,
        }
    }

    fn face(&self) -> FontRef<'static> {
        face_of(self.bold, self.mono)
    }

    /// `ab_glyph` scales by line height (ascent - descent), not by em.
    fn scale(&self) -> PxScale {
        let f = self.face();
        let cell_per_em =
            (f.ascent_unscaled() - f.descent_unscaled()) / f.units_per_em().unwrap_or(2048.0);
        PxScale::from(self.px * cell_per_em)
    }

    fn scaled(&self) -> ab_glyph::PxScaleFont<FontRef<'static>> {
        self.face().into_scaled(self.scale())
    }

    /// Pixels above the baseline.
    pub fn ascent(&self) -> i32 {
        self.scaled().ascent().ceil() as i32
    }

    /// Pixels below the baseline.
    pub fn descent(&self) -> i32 {
        (-self.scaled().descent()).ceil() as i32
    }

    /// Height of a line: ascent plus descent.
    pub fn height(&self) -> i32 {
        self.ascent() + self.descent()
    }

    /// Advance of one UTF-16 unit, in pixels (unrounded).
    fn advance_f(&self, f: &ab_glyph::PxScaleFont<FontRef<'static>>, unit: u16) -> f32 {
        let c = char::from_u32(unit as u32).unwrap_or('?');
        f.h_advance(f.glyph_id(c))
    }

    /// Advance of one character, rounded to whole pixels.
    pub fn advance(&self, c: char) -> i32 {
        let f = self.scaled();
        f.h_advance(f.glyph_id(c)).round() as i32
    }

    /// Where each character ends, measured from the start of `text`, in
    /// whole pixels: the `alpDx` array of `GetTextExtentExPointW`.
    pub fn caret_stops(&self, text: &[u16]) -> Vec<i32> {
        let f = self.scaled();
        let mut x = 0.0f32;
        text.iter()
            .map(|&u| {
                x += self.advance_f(&f, u);
                x.round() as i32
            })
            .collect()
    }

    /// Width of `text` in pixels.
    pub fn width(&self, text: &[u16]) -> i32 {
        self.caret_stops(text).last().copied().unwrap_or(0)
    }

    /// Draw `text` with its cell's top-left corner at `(x, y)`, blending
    /// the anti-aliased edges into what is already there. Returns the
    /// width drawn.
    pub fn draw(&self, surf: &mut Surface<'_>, x: i32, y: i32, text: &[u16], color: u16) -> i32 {
        let f = self.scaled();
        let baseline = y as f32 + f.ascent();
        let mut caret = x as f32;
        for &unit in text {
            let c = char::from_u32(unit as u32).unwrap_or('?');
            let id = f.glyph_id(c);
            let glyph = id.with_scale_and_position(self.scale(), point(caret, baseline));
            caret += f.h_advance(id);
            if let Some(outline) = self.face().outline_glyph(glyph) {
                let bounds = outline.px_bounds();
                // Solid pixels, no anti-aliasing (a pixel at least 30% covered
                // is drawn, so 1-pixel stems such as the i in "Difficulty"
                // survive): Pocket PC 2003 draws text
                // without ClearType (Device Emulator shows crisp Tahoma), and
                // GapiDraw puts text on a colour-keyed surface, where any
                // blended edge pixel misses the key and shows as a pink fringe.
                outline.draw(|gx, gy, coverage| {
                    if coverage >= 0.3 {
                        blend(
                            surf,
                            bounds.min.x as i32 + gx as i32,
                            bounds.min.y as i32 + gy as i32,
                            color,
                            1.0,
                        );
                    }
                });
            }
        }
        (caret - x as f32).round() as i32
    }
}

/// Mix `color` into the RGB565 pixel at `(x, y)` by `alpha` (0..1).
fn blend(surf: &mut Surface<'_>, x: i32, y: i32, color: u16, alpha: f32) {
    let (w, h) = surf.dimensions();
    if x < 0 || y < 0 || x as u32 >= w || y as u32 >= h {
        return;
    }
    let off = (y as u32 * w + x as u32) as usize * 2;
    let pix = surf.pixels_mut();
    let dst = u16::from_le_bytes([pix[off], pix[off + 1]]);
    let a = alpha.clamp(0.0, 1.0);
    let mix = |shift: u32, mask: u16| -> u16 {
        let d = ((dst >> shift) & mask) as f32;
        let s = ((color >> shift) & mask) as f32;
        ((d + (s - d) * a).round() as u16) & mask
    };
    let out = (mix(11, 0x1f) << 11) | (mix(5, 0x3f) << 5) | mix(0, 0x1f);
    let bytes = out.to_le_bytes();
    pix[off] = bytes[0];
    pix[off + 1] = bytes[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().collect()
    }

    #[test]
    fn default_font_is_about_the_size_of_tahoma_8pt() {
        let f = TextFont::from_logfont(0, 400);
        assert_eq!(f.px, DEFAULT_PX);
        // Tahoma 8 pt lines are 13 to 14 pixels apart.
        assert!((12..=15).contains(&f.height()), "height {}", f.height());
    }

    #[test]
    fn text_is_proportional() {
        let f = TextFont::from_logfont(-11, 400);
        assert!(f.advance('W') > f.advance('i'));
        let stops = f.caret_stops(&wide("Hello"));
        assert_eq!(stops.len(), 5);
        assert!(stops.windows(2).all(|w| w[1] > w[0]));
        assert_eq!(f.width(&wide("Hello")), *stops.last().unwrap());
    }

    #[test]
    fn positive_height_is_the_cell_and_bold_is_weight_600_up() {
        let cell = TextFont::from_logfont(16, 700);
        assert!(cell.bold);
        assert!((cell.height() - 16).abs() <= 1, "height {}", cell.height());
        assert!(!TextFont::from_logfont(-11, 400).bold);
    }

    #[test]
    fn fixed_pitch_fonts_are_monospaced() {
        let f = TextFont::from_logfont_pitch(-11, 400, true);
        assert_eq!(f.advance('i'), f.advance('W'));
    }
}
