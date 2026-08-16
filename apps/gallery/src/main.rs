//! The bezel gallery — every component rendered in a real window. This is the
//! dev surface: new components land here the day they land in `crates/ui`.

use bezel_theme::{Theme, appearance};
use bezel_ui::{icons, loaders, popover, widgets};
use gpui::{
    App, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};

fn main() {
    gpui_platform::application()
        .with_assets(icons::Assets)
        .run(|cx: &mut App| {
            bezel_ui::register_fonts(cx).ok();
            appearance::init(appearance::AppearanceMode::System, cx);
            let bounds = Bounds::centered(None, size(px(960.0), px(760.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    appearance::observe_window(window, cx).detach();
                    cx.new(|_| Gallery)
                },
            )
            .unwrap();
            cx.activate(true);
        });
}

struct Gallery;

fn section(theme: &Theme, title: &str) -> gpui::Div {
    div().flex().flex_col().gap(px(12.0)).child(
        div()
            .text_size(px(11.0))
            .font_weight(gpui::FontWeight::MEDIUM)
            .text_color(theme.text_faint)
            .child(SharedString::from(popover::tracked_upper(title))),
    )
}

fn row() -> gpui::Div {
    div().flex().flex_row().items_center().gap(px(12.0))
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let view = cx.entity_id();

        let buttons = section(&theme, "Buttons").child(
            row()
                .child(popover::button(&theme, "Ghost", "g-ghost"))
                .child(popover::button_prominent(&theme, "Prominent"))
                .child(popover::button_destructive(&theme, "Destructive")),
        );

        let toggles = section(&theme, "Toggle & badges").child(
            row()
                .child(widgets::toggle(&theme, true))
                .child(widgets::toggle(&theme, false))
                .child(widgets::badge(&theme, "badge"))
                .child(widgets::badge_active(&theme, "active")),
        );

        let menu = section(&theme, "Menu").child(
            popover::popover_card(&theme).w(px(240.0)).children([
                popover::menu_heading(&theme, "Section").into_any_element(),
                popover::menu_row(&theme, false, "m-one")
                    .child("First item")
                    .into_any_element(),
                popover::menu_row(&theme, true, "m-two")
                    .child("Active item")
                    .into_any_element(),
                popover::divider().into_any_element(),
                popover::menu_row(&theme, false, "m-three")
                    .child("Third item")
                    .into_any_element(),
            ]),
        );

        let group = section(&theme, "Group box").child(
            widgets::group_box(&theme)
                .child(
                    widgets::card_row(&theme, true)
                        .child(widgets::row_tile(&theme, icons::MONITOR))
                        .child(widgets::row_title(&theme, "First row")),
                )
                .child(
                    widgets::card_row(&theme, false)
                        .child(widgets::row_tile(&theme, icons::FOLDER))
                        .child(widgets::row_title(&theme, "Second row")),
                ),
        );

        let spinners = section(&theme, "Loaders").child(
            row()
                .child(loaders::pulse_loader("g-pulse", &theme, 8.0, view, cx))
                .child(loaders::gradient_spinner("g-spin", &theme, 5.0, view, cx))
                .child(loaders::mini_gradient_spinner("g-mini", 2.5, view, cx))
                .child(loaders::loading_word(&theme)),
        );

        let strips = section(&theme, "Strips & redacted")
            .child(widgets::error_strip(&theme, "Something went wrong."))
            .child(widgets::warning_strip(&theme, "Heads up, check this."))
            .child(popover::redacted_rows("g-redacted", &theme, 3, view, cx));

        div()
            .id("gallery-scroll")
            .size_full()
            .overflow_y_scroll()
            .bg(theme.bg)
            .font_family(theme.font_sans.clone())
            .text_color(theme.text)
            .text_size(px(14.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(28.0))
                    .p(px(32.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("bezel gallery"),
                    )
                    .child(buttons)
                    .child(toggles)
                    .child(menu)
                    .child(group)
                    .child(spinners)
                    .child(strips),
            )
    }
}
