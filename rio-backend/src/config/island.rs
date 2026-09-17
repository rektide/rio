//! The `[island]` table: every geometry/color knob of the tab strip
//! ("island") renderer. Field names mirror the constants they replaced
//! in `frontends/rioterm/src/renderer/island.rs`; the defaults below
//! are exactly the values those constants held, so an absent `[island]`
//! table renders identically to the hardcoded strip.
//!
//! All sizes are logical pixels. Values are clamped in `sanitize`
//! (called once after config load and platform merge) so a bad value
//! degrades to a bound instead of drawing a broken strip.

use crate::config::colors::{deserialize_to_arr_vec, ColorArray};
use serde::{Deserialize, Serialize};

// --- defaults -------------------------------------------------------------

#[inline]
pub fn default_height() -> f32 {
    38.0
}

#[inline]
pub fn default_title_font_size() -> f32 {
    12.0
}

#[inline]
pub fn default_tab_padding_x() -> f32 {
    27.0
}

#[inline]
pub fn default_tab_gap() -> f32 {
    6.0
}

#[inline]
pub fn default_tab_inset_y() -> f32 {
    7.0
}

#[inline]
pub fn default_tab_radius() -> f32 {
    6.0
}

#[inline]
pub fn default_margin_right() -> f32 {
    8.0
}

#[inline]
pub fn default_margin_left_macos() -> f32 {
    76.0
}

#[inline]
pub fn default_progress_bar_height() -> f32 {
    3.0
}

#[inline]
pub fn default_progress_bar_timeout_secs() -> u64 {
    15
}

#[inline]
pub fn default_picker_swatch_size() -> f32 {
    18.0
}

#[inline]
pub fn default_picker_swatch_gap() -> f32 {
    4.0
}

#[inline]
pub fn default_picker_padding() -> f32 {
    6.0
}

#[inline]
pub fn default_picker_input_height() -> f32 {
    26.0
}

#[inline]
pub fn default_picker_input_font_size() -> f32 {
    12.0
}

#[inline]
pub fn default_picker_input_margin_top() -> f32 {
    8.0
}

#[inline]
pub fn default_picker_top_padding() -> f32 {
    4.0
}

/// The six swatches the hardcoded picker offered (red, orange, yellow,
/// green, blue, purple), in the same order and the same sRGB 0-1 values.
#[inline]
pub fn default_picker_colors() -> Vec<ColorArray> {
    vec![
        // red
        [0.86, 0.26, 0.27, 1.0],
        // orange
        [0.90, 0.57, 0.22, 1.0],
        // yellow
        [0.85, 0.78, 0.25, 1.0],
        // green
        [0.34, 0.70, 0.38, 1.0],
        // blue
        [0.30, 0.55, 0.85, 1.0],
        // purple
        [0.68, 0.40, 0.80, 1.0],
    ]
}

#[inline]
pub fn default_close_margin_right() -> f32 {
    14.0
}

#[inline]
pub fn default_close_glyph_half() -> f32 {
    3.5
}

#[inline]
pub fn default_close_min_island_width() -> f32 {
    64.0
}

#[inline]
pub fn default_close_hover_half() -> f32 {
    10.0
}

#[inline]
pub fn default_close_hover_corner_radius() -> f32 {
    5.0
}

#[inline]
pub fn default_close_hit_half_width() -> f32 {
    10.0
}

#[inline]
pub fn default_close_alpha_idle() -> f32 {
    0.55
}

#[inline]
pub fn default_close_alpha_hover() -> f32 {
    0.95
}

#[inline]
pub fn default_close_stroke_width() -> f32 {
    1.2
}

#[inline]
pub fn default_inactive_custom_mute() -> f32 {
    0.55
}

#[inline]
pub fn default_drag_threshold() -> f32 {
    4.0
}

#[inline]
pub fn default_drag_animation_length() -> f32 {
    0.15
}

#[inline]
pub fn default_drag_max_dt() -> f32 {
    0.05
}

#[inline]
pub fn default_bell_dot_size() -> f32 {
    6.0
}

#[inline]
pub fn default_bell_dot_order() -> u8 {
    1
}

#[inline]
pub fn default_bell_gap() -> f32 {
    4.0
}

// --- struct ---------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IslandConfig {
    /// Height of the whole strip in logical pixels. The terminal grid
    /// starts below it and mouse routing treats y < height as strip
    /// chrome, so this one value moves the entire layout.
    #[serde(default = "default_height")]
    pub height: f32,
    /// Font size of tab titles.
    #[serde(default = "default_title_font_size", rename = "title-font-size")]
    pub title_font_size: f32,
    /// Horizontal padding between a tab's edge and its (centred) title.
    #[serde(default = "default_tab_padding_x", rename = "tab-padding-x")]
    pub tab_padding_x: f32,
    /// Space between tab slots; the pill is inset by half the gap on
    /// each side.
    #[serde(default = "default_tab_gap", rename = "tab-gap")]
    pub tab_gap: f32,
    /// Vertical inset of a tab pill from the top/bottom of the strip.
    /// Kept below `height / 2` so the pill stays visible.
    #[serde(default = "default_tab_inset_y", rename = "tab-inset-y")]
    pub tab_inset_y: f32,
    /// Corner radius of a tab pill. Clamped to half the pill at draw
    /// time, so large values round into a capsule.
    #[serde(default = "default_tab_radius", rename = "tab-radius")]
    pub tab_radius: f32,
    /// Right margin kept after the last tab slot.
    #[serde(default = "default_margin_right", rename = "margin-right")]
    pub margin_right: f32,
    /// Left margin on macOS clearing the traffic-light buttons.
    /// Ignored on other platforms.
    #[serde(default = "default_margin_left_macos", rename = "margin-left-macos")]
    pub margin_left_macos: f32,
    /// Thickness of the OSC 9;4 progress bar.
    #[serde(
        default = "default_progress_bar_height",
        rename = "progress-bar-height"
    )]
    pub progress_bar_height: f32,
    /// Seconds without an OSC 9;4 report before the progress bar is
    /// dismissed.
    #[serde(
        default = "default_progress_bar_timeout_secs",
        rename = "progress-bar-timeout-secs"
    )]
    pub progress_bar_timeout_secs: u64,
    /// Size of one color-picker swatch.
    #[serde(default = "default_picker_swatch_size", rename = "picker-swatch-size")]
    pub picker_swatch_size: f32,
    /// Gap between color-picker swatches.
    #[serde(default = "default_picker_swatch_gap", rename = "picker-swatch-gap")]
    pub picker_swatch_gap: f32,
    /// Inner padding of the color-picker dropdown.
    #[serde(default = "default_picker_padding", rename = "picker-padding")]
    pub picker_padding: f32,
    /// Height of the rename input inside the color picker.
    #[serde(
        default = "default_picker_input_height",
        rename = "picker-input-height"
    )]
    pub picker_input_height: f32,
    /// Font size of the rename input.
    #[serde(
        default = "default_picker_input_font_size",
        rename = "picker-input-font-size"
    )]
    pub picker_input_font_size: f32,
    /// Space between the swatch row and the rename input.
    #[serde(
        default = "default_picker_input_margin_top",
        rename = "picker-input-margin-top"
    )]
    pub picker_input_margin_top: f32,
    /// Extra padding above the first swatch row.
    #[serde(default = "default_picker_top_padding", rename = "picker-top-padding")]
    pub picker_top_padding: f32,
    /// Swatches offered by the tab color picker (right-click a tab).
    /// Hex strings; an empty list falls back to the default palette.
    /// Not serialized (ColorArray has no hex representation).
    #[serde(
        default = "default_picker_colors",
        deserialize_with = "deserialize_to_arr_vec",
        rename = "picker-colors",
        skip_serializing
    )]
    pub picker_colors: Vec<ColorArray>,
    /// Distance from a pill's right edge to the close glyph center.
    #[serde(default = "default_close_margin_right", rename = "close-margin-right")]
    pub close_margin_right: f32,
    /// Half-size of the × glyph.
    #[serde(default = "default_close_glyph_half", rename = "close-glyph-half")]
    pub close_glyph_half: f32,
    /// Pills narrower than this hide the close button (the title needs
    /// the room).
    #[serde(
        default = "default_close_min_island_width",
        rename = "close-min-island-width"
    )]
    pub close_min_island_width: f32,
    /// Half-size of the close-button hover highlight box.
    #[serde(default = "default_close_hover_half", rename = "close-hover-half")]
    pub close_hover_half: f32,
    /// Corner radius of the close-button hover highlight.
    #[serde(
        default = "default_close_hover_corner_radius",
        rename = "close-hover-corner-radius"
    )]
    pub close_hover_corner_radius: f32,
    /// Half-width of the close-button hit area.
    #[serde(
        default = "default_close_hit_half_width",
        rename = "close-hit-half-width"
    )]
    pub close_hit_half_width: f32,
    /// Idle alpha of the close glyph.
    #[serde(default = "default_close_alpha_idle", rename = "close-alpha-idle")]
    pub close_alpha_idle: f32,
    /// Hover alpha of the close glyph.
    #[serde(default = "default_close_alpha_hover", rename = "close-alpha-hover")]
    pub close_alpha_hover: f32,
    /// Stroke width of the close glyph lines.
    #[serde(default = "default_close_stroke_width", rename = "close-stroke-width")]
    pub close_stroke_width: f32,
    /// Alpha multiplier applied to a custom-colored tab when it is not
    /// the active one, muting inactive pills toward the strip.
    #[serde(
        default = "default_inactive_custom_mute",
        rename = "inactive-custom-mute"
    )]
    pub inactive_custom_mute: f32,
    /// Pointer travel (logical px) before a tab press becomes a drag.
    #[serde(default = "default_drag_threshold", rename = "drag-threshold")]
    pub drag_threshold: f32,
    /// Slide-animation length (seconds) when tabs settle after reorder.
    #[serde(
        default = "default_drag_animation_length",
        rename = "drag-animation-length"
    )]
    pub drag_animation_length: f32,
    /// Upper bound on a single animation step (seconds); larger steps
    /// are clamped so a stall can't teleport tabs.
    #[serde(default = "default_drag_max_dt", rename = "drag-max-dt")]
    pub drag_max_dt: f32,
    /// Edge length of the bell dot drawn beside the title of a
    /// background tab that rang (a dot, not a glyph: it renders
    /// identically without an emoji font).
    #[serde(default = "default_bell_dot_size", rename = "bell-dot-size")]
    pub bell_dot_size: f32,
    /// Rect draw order of the bell dot; must sit above the tab
    /// backgrounds, which are submitted at order 0 after the title
    /// pass.
    #[serde(default = "default_bell_dot_order", rename = "bell-dot-order")]
    pub bell_dot_order: u8,
    /// Gap between the bell dot and the tab title.
    #[serde(default = "default_bell_gap", rename = "bell-gap")]
    pub bell_gap: f32,
}

impl Default for IslandConfig {
    fn default() -> Self {
        IslandConfig {
            height: default_height(),
            title_font_size: default_title_font_size(),
            tab_padding_x: default_tab_padding_x(),
            tab_gap: default_tab_gap(),
            tab_inset_y: default_tab_inset_y(),
            tab_radius: default_tab_radius(),
            margin_right: default_margin_right(),
            margin_left_macos: default_margin_left_macos(),
            progress_bar_height: default_progress_bar_height(),
            progress_bar_timeout_secs: default_progress_bar_timeout_secs(),
            picker_swatch_size: default_picker_swatch_size(),
            picker_swatch_gap: default_picker_swatch_gap(),
            picker_padding: default_picker_padding(),
            picker_input_height: default_picker_input_height(),
            picker_input_font_size: default_picker_input_font_size(),
            picker_input_margin_top: default_picker_input_margin_top(),
            picker_top_padding: default_picker_top_padding(),
            picker_colors: default_picker_colors(),
            close_margin_right: default_close_margin_right(),
            close_glyph_half: default_close_glyph_half(),
            close_min_island_width: default_close_min_island_width(),
            close_hover_half: default_close_hover_half(),
            close_hover_corner_radius: default_close_hover_corner_radius(),
            close_hit_half_width: default_close_hit_half_width(),
            close_alpha_idle: default_close_alpha_idle(),
            close_alpha_hover: default_close_alpha_hover(),
            close_stroke_width: default_close_stroke_width(),
            inactive_custom_mute: default_inactive_custom_mute(),
            drag_threshold: default_drag_threshold(),
            drag_animation_length: default_drag_animation_length(),
            drag_max_dt: default_drag_max_dt(),
            bell_dot_size: default_bell_dot_size(),
            bell_dot_order: default_bell_dot_order(),
            bell_gap: default_bell_gap(),
        }
    }
}

/// Clamp helper: non-finite values fall back to `default`, everything
/// else to `[min, max]` — the `clamp_max_tab_width` contract.
#[inline]
fn clamp_f32(v: f32, min: f32, max: f32, default: f32) -> f32 {
    if !v.is_finite() {
        return default;
    }
    v.clamp(min, max)
}

impl IslandConfig {
    /// Clamp every field and enforce the cross-field invariants the
    /// hardcoded strip guaranteed:
    ///
    /// - `tab_inset_y * 2 < height` (the pill stays visible)
    /// - `close_hover_half * 2 <= height - tab_inset_y * 2` (the hover
    ///   box fits inside the pill)
    /// - `close_margin_right + close_hit_half_width < close_min_island_width`
    ///   (a pill that shows the button is wide enough to hit it)
    ///
    /// Must run after every field assignment (config load, platform
    /// merge, live reload).
    pub fn sanitize(&mut self) {
        self.height = clamp_f32(self.height, 20.0, 128.0, default_height());
        self.title_font_size =
            clamp_f32(self.title_font_size, 6.0, 32.0, default_title_font_size());
        self.tab_padding_x =
            clamp_f32(self.tab_padding_x, 0.0, 128.0, default_tab_padding_x());
        self.tab_gap = clamp_f32(self.tab_gap, 0.0, 64.0, default_tab_gap());
        self.tab_radius = clamp_f32(self.tab_radius, 0.0, 64.0, default_tab_radius());
        self.margin_right =
            clamp_f32(self.margin_right, 0.0, 128.0, default_margin_right());
        self.margin_left_macos = clamp_f32(
            self.margin_left_macos,
            0.0,
            1024.0,
            default_margin_left_macos(),
        );
        self.progress_bar_height = clamp_f32(
            self.progress_bar_height,
            0.5,
            32.0,
            default_progress_bar_height(),
        );
        self.progress_bar_timeout_secs = self.progress_bar_timeout_secs.clamp(1, 3600);
        self.picker_swatch_size = clamp_f32(
            self.picker_swatch_size,
            4.0,
            64.0,
            default_picker_swatch_size(),
        );
        self.picker_swatch_gap = clamp_f32(
            self.picker_swatch_gap,
            0.0,
            32.0,
            default_picker_swatch_gap(),
        );
        self.picker_padding =
            clamp_f32(self.picker_padding, 0.0, 64.0, default_picker_padding());
        self.picker_input_height = clamp_f32(
            self.picker_input_height,
            8.0,
            64.0,
            default_picker_input_height(),
        );
        self.picker_input_font_size = clamp_f32(
            self.picker_input_font_size,
            6.0,
            32.0,
            default_picker_input_font_size(),
        );
        self.picker_input_margin_top = clamp_f32(
            self.picker_input_margin_top,
            0.0,
            64.0,
            default_picker_input_margin_top(),
        );
        self.picker_top_padding = clamp_f32(
            self.picker_top_padding,
            0.0,
            64.0,
            default_picker_top_padding(),
        );
        self.close_margin_right = clamp_f32(
            self.close_margin_right,
            0.0,
            128.0,
            default_close_margin_right(),
        );
        self.close_glyph_half =
            clamp_f32(self.close_glyph_half, 0.5, 32.0, default_close_glyph_half());
        self.close_min_island_width = clamp_f32(
            self.close_min_island_width,
            0.0,
            1024.0,
            default_close_min_island_width(),
        );
        self.close_hover_half =
            clamp_f32(self.close_hover_half, 1.0, 64.0, default_close_hover_half());
        self.close_hover_corner_radius = clamp_f32(
            self.close_hover_corner_radius,
            0.0,
            32.0,
            default_close_hover_corner_radius(),
        );
        self.close_hit_half_width = clamp_f32(
            self.close_hit_half_width,
            0.5,
            64.0,
            default_close_hit_half_width(),
        );
        self.close_alpha_idle =
            clamp_f32(self.close_alpha_idle, 0.0, 1.0, default_close_alpha_idle());
        self.close_alpha_hover = clamp_f32(
            self.close_alpha_hover,
            0.0,
            1.0,
            default_close_alpha_hover(),
        );
        self.close_stroke_width = clamp_f32(
            self.close_stroke_width,
            0.1,
            16.0,
            default_close_stroke_width(),
        );
        self.inactive_custom_mute = clamp_f32(
            self.inactive_custom_mute,
            0.0,
            1.0,
            default_inactive_custom_mute(),
        );
        self.drag_threshold =
            clamp_f32(self.drag_threshold, 0.0, 128.0, default_drag_threshold());
        self.drag_animation_length = clamp_f32(
            self.drag_animation_length,
            0.01,
            5.0,
            default_drag_animation_length(),
        );
        self.drag_max_dt = clamp_f32(self.drag_max_dt, 0.001, 1.0, default_drag_max_dt());
        self.bell_dot_size =
            clamp_f32(self.bell_dot_size, 1.0, 32.0, default_bell_dot_size());
        self.bell_gap = clamp_f32(self.bell_gap, 0.0, 64.0, default_bell_gap());

        // Invariant: the pill must stay inside the strip, with a hair
        // of strictness to keep a degenerate `inset == height / 2`
        // pill of height zero from appearing.
        self.tab_inset_y = clamp_f32(
            self.tab_inset_y,
            0.0,
            (self.height / 2.0 - 0.5).max(0.0),
            default_tab_inset_y(),
        );

        // Invariant: the hover highlight fits inside the pill.
        let pill_half = (self.height - self.tab_inset_y * 2.0) / 2.0;
        if self.close_hover_half > pill_half {
            self.close_hover_half = pill_half.max(0.0);
        }

        // Invariant: a pill wide enough to show the close button is
        // also wide enough to hit it. Raise the floor rather than
        // shrink the hit box, matching the original constant relation.
        let min_island = self.close_margin_right + self.close_hit_half_width + 1.0;
        if self.close_min_island_width < min_island {
            self.close_min_island_width = min_island;
        }

        // An empty palette leaves the picker with only the reset
        // swatch — restore the default six rather than a useless row.
        if self.picker_colors.is_empty() {
            self.picker_colors = default_picker_colors();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_hardcoded_strip() {
        let cfg = IslandConfig::default();
        assert_eq!(cfg.height, 38.0);
        assert_eq!(cfg.title_font_size, 12.0);
        assert_eq!(cfg.tab_padding_x, 27.0);
        assert_eq!(cfg.tab_gap, 6.0);
        assert_eq!(cfg.tab_inset_y, 7.0);
        assert_eq!(cfg.tab_radius, 6.0);
        assert_eq!(cfg.margin_right, 8.0);
        assert_eq!(cfg.margin_left_macos, 76.0);
        assert_eq!(cfg.progress_bar_height, 3.0);
        assert_eq!(cfg.progress_bar_timeout_secs, 15);
        assert_eq!(cfg.picker_swatch_size, 18.0);
        assert_eq!(cfg.picker_swatch_gap, 4.0);
        assert_eq!(cfg.picker_padding, 6.0);
        assert_eq!(cfg.picker_input_height, 26.0);
        assert_eq!(cfg.picker_input_font_size, 12.0);
        assert_eq!(cfg.picker_input_margin_top, 8.0);
        assert_eq!(cfg.picker_top_padding, 4.0);
        assert_eq!(cfg.picker_colors.len(), 6);
        assert_eq!(cfg.close_margin_right, 14.0);
        assert_eq!(cfg.close_glyph_half, 3.5);
        assert_eq!(cfg.close_min_island_width, 64.0);
        assert_eq!(cfg.close_hover_half, 10.0);
        assert_eq!(cfg.close_hover_corner_radius, 5.0);
        assert_eq!(cfg.close_hit_half_width, 10.0);
        assert_eq!(cfg.close_alpha_idle, 0.55);
        assert_eq!(cfg.close_alpha_hover, 0.95);
        assert_eq!(cfg.close_stroke_width, 1.2);
        assert_eq!(cfg.inactive_custom_mute, 0.55);
        assert_eq!(cfg.drag_threshold, 4.0);
        assert_eq!(cfg.drag_animation_length, 0.15);
        assert_eq!(cfg.drag_max_dt, 0.05);
        assert_eq!(cfg.bell_dot_size, 6.0);
        assert_eq!(cfg.bell_dot_order, 1);
        assert_eq!(cfg.bell_gap, 4.0);
    }

    #[test]
    fn sanitize_keeps_geometry_invariants() {
        // The relations the old `const` assertions guaranteed hold for
        // arbitrary hostile input.
        let mut cfg = IslandConfig {
            height: 20.0,
            tab_inset_y: 18.0,
            close_hover_half: 64.0,
            close_min_island_width: 0.0,
            close_margin_right: 120.0,
            close_hit_half_width: 64.0,
            tab_padding_x: f32::NAN,
            close_alpha_idle: 2.0,
            picker_colors: vec![],
            drag_max_dt: 0.0,
            bell_dot_size: 0.0,
            bell_gap: f32::NAN,
            ..IslandConfig::default()
        };
        cfg.sanitize();

        assert!(cfg.tab_inset_y * 2.0 < cfg.height);
        assert!(cfg.close_hover_half * 2.0 <= cfg.height - cfg.tab_inset_y * 2.0);
        assert!(
            cfg.close_margin_right + cfg.close_hit_half_width
                < cfg.close_min_island_width
        );
        // Non-finite falls back to the default, clamps bound the rest.
        assert_eq!(cfg.tab_padding_x, default_tab_padding_x());
        assert_eq!(cfg.close_alpha_idle, 1.0);
        assert_eq!(cfg.drag_max_dt, 0.001);
        assert_eq!(cfg.bell_dot_size, 1.0);
        assert_eq!(cfg.bell_gap, default_bell_gap());
        // Empty palette restores the default swatches.
        assert_eq!(cfg.picker_colors.len(), 6);
    }

    #[test]
    fn toml_decode_kebab_case_and_colors() {
        // r## delimiter: the payload contains "#rrggbb" strings whose
        // `"#` would otherwise close a plain r#"…"# literal.
        let content = r##"
            [island]
            height = 30.0
            tab-padding-x = 10.0
            picker-colors = ["#ff0000", "#00ff00", "#0000ff"]
        "##;

        #[derive(Deserialize)]
        struct Root {
            island: IslandConfig,
        }

        let decoded: Root = toml::from_str(content).unwrap();
        assert_eq!(decoded.island.height, 30.0);
        assert_eq!(decoded.island.tab_padding_x, 10.0);
        assert_eq!(decoded.island.picker_colors.len(), 3);
        assert_eq!(decoded.island.picker_colors[0], [1.0, 0.0, 0.0, 1.0]);
        // Untouched fields keep their defaults.
        assert_eq!(decoded.island.tab_gap, default_tab_gap());
    }

    #[test]
    fn absent_table_uses_defaults() {
        #[derive(Deserialize)]
        struct Root {
            #[serde(default)]
            island: IslandConfig,
        }

        let decoded: Root = toml::from_str("").unwrap();
        assert_eq!(decoded.island, IslandConfig::default());
    }
}
