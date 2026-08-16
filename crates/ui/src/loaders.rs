//! Loaders: the pulse loader and the gradient matrix spinners. All motion
//! routes through `bezel_motion` pure helpers, so the math is unit-tested and
//! these elements are testable-by-compile.
//!
//! Rendering pattern: each cell is its own `with_animation` repeating element
//! sharing one period; per-cell offsets come from [`motion::staggered_phase`],
//! so all cells stay phase-locked (they start on the same frame) without a
//! shared clock. Cells animate inside fixed-size slots — opacity and inner size
//! are paint-local and never move surrounding layout. Reduced motion snaps every
//! cell to its rest state automatically (gpui `reduce_motion`).

use gpui::{App, EntityId, IntoElement, ParentElement, SharedString, Styled, div, px};

use bezel_motion as motion;
use bezel_motion::{GRADIENT_SPIN, PULSE, PULSE_STAGGER};
use bezel_theme::Theme;

pub use bezel_motion::phase::{GSPIN_DIM, GSPIN_ROW_TINTS, MATRIX_SIDE, PULSE_CELLS};

/// The pulse wave loader: a row of cells pulsing opacity 0.08→1 / scale 0.9→1
/// over 2.4s with a 0.15s stagger per cell.
///
/// `id` scopes the per-cell animation state — give each loader instance a
/// distinct id.
pub fn pulse_loader(
    _id: &'static str,
    theme: &Theme,
    cell_px: f32,
    view: EntityId,
    cx: &mut App,
) -> impl IntoElement {
    let color = theme.text;
    let slot = cell_px;
    let delta = motion::pulse_delta(&PULSE, view, cx);
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(slot / 2.0))
        .children((0..PULSE_CELLS).map(move |i| {
            // Fixed slot; the animated cell breathes inside it.
            div()
                .size(px(slot))
                .flex()
                .items_center()
                .justify_center()
                .child({
                    let phase = motion::staggered_phase(delta, i, PULSE_STAGGER);
                    div()
                        .rounded(px(slot / 4.0))
                        .bg(color)
                        .opacity(motion::pulse_opacity(phase))
                        .size(px(slot * motion::pulse_scale(phase)))
                })
        }))
}

/// The gradient matrix spinner (working indicator): a 3×3 grid of round cells
/// tinted per row from the sunrise gradient. Each cell pulses opacity once per
/// 750ms period; the per-cell phase follows the "arrow-up" pattern (the pulse
/// enters at the bottom edge and converges toward the top-center cell), so the
/// wave reads as travelling upward.
pub fn gradient_spinner(
    _id: &'static str,
    _theme: &Theme,
    cell_px: f32,
    view: EntityId,
    cx: &mut App,
) -> impl IntoElement {
    let center = (MATRIX_SIDE as f32 - 1.0) / 2.0;
    let max = MATRIX_SIDE as f32 - 1.0 + center;
    let delta = motion::pulse_delta(&GRADIENT_SPIN, view, cx);
    div()
        .flex()
        .flex_col()
        .gap(px(cell_px / 2.0))
        .children((0..MATRIX_SIDE).map(move |row| {
            let tint: gpui::Hsla = gpui::rgb(GSPIN_ROW_TINTS[row]).into();
            div()
                .flex()
                .flex_row()
                .gap(px(cell_px / 2.0))
                .children((0..MATRIX_SIDE).map(move |col| {
                    // Distance of this cell from the wave origin, normalized
                    // into a phase offset (gradient-spin's `--gspin-phase`).
                    let d = MATRIX_SIDE as f32 - 1.0 - row as f32 + (col as f32 - center).abs();
                    let phase = if max == 0.0 { 0.0 } else { d / (max + 1.0) };
                    div()
                        .size(px(cell_px))
                        .rounded(px(cell_px / 2.0))
                        .bg(tint)
                        .opacity(motion::gspin_opacity(delta + phase, GSPIN_DIM))
                }))
        }))
}

/// A 2×3 miniature of [`gradient_spinner`] sized for a status-dot slot: same
/// row tints and pulse timing, but the brightness SNAKES around the grid's
/// perimeter (every cell of a 2×3 grid is on the ring) instead of sweeping as
/// a vertical wave — a tiny radial chase. ~6×10px footprint at the default
/// 2.5px cells.
pub fn mini_gradient_spinner(
    key: impl Into<SharedString>,
    cell_px: f32,
    view: EntityId,
    cx: &mut App,
) -> impl IntoElement {
    const COLS: usize = 2;
    const ROWS: usize = 3;
    /// Clockwise ring position of each `(row, col)` cell, top-left first:
    /// (0,0) → (0,1) → (1,1) → (2,1) → (2,0) → (1,0).
    const RING: [[usize; COLS]; ROWS] = [[0, 1], [5, 2], [4, 3]];
    const RING_LEN: f32 = (COLS * ROWS) as f32;
    let _key = key.into();
    let delta = motion::pulse_delta(&GRADIENT_SPIN, view, cx);
    div()
        .flex()
        .flex_col()
        .gap(px(cell_px / 2.0))
        .children((0..ROWS).map(move |row| {
            let tint: gpui::Hsla = gpui::rgb(GSPIN_ROW_TINTS[row]).into();
            div()
                .flex()
                .flex_row()
                .gap(px(cell_px / 2.0))
                .children((0..COLS).map(move |col| {
                    let phase = RING[row][col] as f32 / RING_LEN;
                    div()
                        .size(px(cell_px))
                        .rounded(px(cell_px / 2.0))
                        .bg(tint)
                        .opacity(motion::gspin_opacity(delta + phase, GSPIN_DIM))
                }))
        }))
}

/// "L O A D I N G" — `text-[11px] uppercase tracking-[0.32em]`; tracking
/// approximated with thin spaces (gpui has no letter-spacing at the pinned
/// rev).
pub fn loading_word(theme: &Theme) -> impl IntoElement {
    div()
        .text_size(px(11.0))
        .text_color(theme.text_muted.opacity(0.7))
        .child(SharedString::from(
            "L\u{2009}O\u{2009}A\u{2009}D\u{2009}I\u{2009}N\u{2009}G",
        ))
}

// Compile-time proof the specs referenced here stay wired to the catalog.
const _: () = {
    assert!(PULSE.duration_ms == 2400);
    assert!(GRADIENT_SPIN.duration_ms == 750);
};
