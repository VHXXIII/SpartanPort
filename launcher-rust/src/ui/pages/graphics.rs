//! The Graphics page: window, effects, textures, overlay and screenshots.
//!
//! Ruling R35: like Python 2.6.0, the Experimental and Upscaling sections (`experimental_upscaler`,
//! `upscaler`, `preset`, `output_res`, `live_resolution`, `object_motion`) stay parked until the
//! image quality of temporal upscaling is validated, so this page does not show them. The one
//! exception is the Anti-aliasing pair of the Effects section: the game's FXAA / SMAA, and a
//! temporal anti-aliasing (TAA or FSR 3.1 Native AA) that always runs at native resolution.

use iced::widget::{button, column, text};
use iced::Element;

use crate::core::i18n::tr;
use crate::core::options::{ANISOTROPY, ANTI_ALIASING, DISPLAY_MODES, TEMPORAL_AA};
use crate::ui::app::{Message, State};
use crate::ui::theme;
use crate::ui::widgets::{checkbox, row, section, select, select_pairs};

/// The hint of the Anti-aliasing row.
pub const AA_HINT: &str = "Smooths jagged edges. FXAA is cheap and a little soft; SMAA keeps the picture sharper and costs a bit more. Both also touch the HUD slightly.";

/// The hint of the Temporal anti-aliasing row.
pub const TEMPORAL_AA_HINT: &str = "Off (default). Smooths edges and shimmering using previous frames, at native resolution (no downscaling). TAA is the port's own and is tuned for this game: its default values are the optimized ones, and every setting can be adjusted to your taste in the in-game menu (Insert / R3+L2) > Image. FSR 3.1 uses AMD's algorithm in its Native AA mode, on any graphics card; its default values are optimized for this game too, and its look can be adjusted in the same menu. Use either with Anti-aliasing set to Off. Takes effect after restarting the game.";

pub fn view(state: &State) -> Element<'_, Message> {
    let lang = state.lang.as_str();
    let hint = |english: &str| tr(lang, english);
    let ini = |key: &str| state.store.text(key);

    // The list the probe made, or just the primary default while it has not answered.
    let pairs: Vec<(String, String)> = if state.monitor_pairs.is_empty() {
        crate::core::gpu::monitor_options(&[])
    } else {
        state.monitor_pairs.clone()
    }
    .into_iter()
    .map(|(value, label)| {
        let label = if value.is_empty() {
            tr(lang, &label)
        } else {
            label
        };
        (value, label)
    })
    .collect();
    let monitor = state.store.text("monitor");

    // The Python marks a row that needs a new start of the game with a small `restart` note.
    let anisotropy_label = format!(
        "{}  ·  {}",
        tr(lang, "Anisotropic filtering"),
        tr(lang, "restart")
    );

    let temporal_label = format!(
        "{}  ·  {}",
        tr(lang, "Temporal anti-aliasing"),
        tr(lang, "restart")
    );

    let shots = button(text(tr(lang, "Open the screenshots folder")))
        .style(theme::button_flat)
        .on_press(Message::OpenFolder(
            state
                .paths
                .user_dir(&state.store.text("user_dir"))
                .join("screenshots"),
        ));

    column![
        section(state.rtl, &tr(lang, "Window")),
        row(
            state.rtl,
            &tr(lang, "Display mode"),
            select("display_mode", DISPLAY_MODES, &ini("display_mode"), lang),
            Some(&hint(
                "Borderless covers the screen without changing the display mode."
            )),
        ),
        row(
            state.rtl,
            &tr(lang, "Monitor"),
            select_pairs("monitor", &pairs, &monitor),
            Some(&hint(
                "The screen the game opens its window on. Primary is the monitor the system considers the main one."
            )),
        ),
        checkbox(
            state.rtl,
            "vsync",
            &tr(lang, "VSync"),
            state.store.flag("vsync"),
            Some(&hint("Off allows tearing for the lowest latency.")),
        ),
        checkbox(
            state.rtl,
            "hdr",
            &tr(lang, "Allow HDR output"),
            state.store.flag("hdr"),
            Some(&hint("When HDR is on in Windows and the display supports it.")),
        ),
        section(state.rtl, &tr(lang, "Effects")),
        checkbox(
            state.rtl,
            "motion_blur",
            &tr(lang, "Motion blur"),
            state.store.flag("motion_blur"),
            Some(&hint(
                "The game's own motion blur on fast moving objects and the camera. Off removes it."
            )),
        ),
        checkbox(
            state.rtl,
            "depth_of_field",
            &tr(lang, "Depth of field"),
            state.store.flag("depth_of_field"),
            Some(&hint(
                "The game's own blur of distant objects, strongest in cutscenes. Off keeps the whole picture sharp."
            )),
        ),
        row(
            state.rtl,
            &tr(lang, "Anti-aliasing"),
            select("anti_aliasing", ANTI_ALIASING, &ini("anti_aliasing"), lang),
            Some(&hint(AA_HINT)),
        ),
        row(
            state.rtl,
            &temporal_label,
            select("temporal_aa", TEMPORAL_AA, &ini("temporal_aa"), lang),
            Some(&hint(TEMPORAL_AA_HINT)),
        ),
        section(state.rtl, &tr(lang, "Textures")),
        row(
            state.rtl,
            &anisotropy_label,
            select("anisotropy", ANISOTROPY, &ini("anisotropy"), lang),
            Some(&hint(
                "Sharper floors and walls seen at an angle, at little cost on most graphics cards."
            )),
        ),
        section(state.rtl, &tr(lang, "Overlay")),
        checkbox(
            state.rtl,
            "show_fps",
            &tr(lang, "Show performance overlay"),
            state.store.flag("show_fps"),
            Some(&hint(
                "Items, size and corner: in-game menu (Insert / R3+L2) > Overlay."
            )),
        ),
        section(state.rtl, &tr(lang, "Screenshots")),
        row(
            state.rtl,
            &tr(lang, "F12 in the game"),
            shots.into(),
            Some(&hint(
                "Saves the game's image, without the overlay, as a PNG in the screenshots folder of the saves."
            )),
        ),
    ]
    .spacing(2)
    .into()
}
