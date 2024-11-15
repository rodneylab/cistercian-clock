use chrono::{Local, Timelike};
use core::time::Duration;
use egui::{
    epaint::Shadow,
    scroll_area::ScrollBarVisibility,
    style::{HandleShape, NumericColorSpace, Selection, TextCursorStyle, Widgets},
    vec2, Color32,
    FontFamily::Proportional,
    FontId, Painter, Pos2, Rounding, ScrollArea, Sense, Stroke,
    TextStyle::{self, Body, Button, Heading, Monospace, Name, Small},
    Ui, Vec2, Visuals,
};

fn dark_mode_override() -> Visuals {
    Visuals {
        dark_mode: true,
        override_text_color: Some(Color32::from_gray(252)),
        widgets: Widgets::default(),
        selection: Selection::default(),
        hyperlink_color: Color32::from_rgb(90, 170, 255),
        faint_bg_color: Color32::from_additive_luminance(5), // visible, but barely so
        extreme_bg_color: Color32::from_gray(10),            // e.g. TextEdit background
        code_bg_color: Color32::from_gray(64),
        warn_fg_color: Color32::from_rgb(255, 143, 0), // orange
        error_fg_color: Color32::from_rgb(255, 0, 0),  // red

        window_rounding: Rounding::same(6.0),
        window_shadow: Shadow {
            offset: vec2(10.0, 20.0),
            blur: 15.0,
            spread: 0.0,
            color: Color32::from_black_alpha(96),
        },
        window_fill: Color32::from_rgb(23, 18, 25),
        window_stroke: Stroke::new(1.0, Color32::from_gray(60)),
        window_highlight_topmost: true,

        menu_rounding: Rounding::same(6.0),

        panel_fill: Color32::from_rgb(23, 18, 25),

        popup_shadow: Shadow {
            offset: vec2(10.0, 20.0),
            blur: 15.0,
            spread: 0.0,
            color: Color32::from_black_alpha(96),
        },
        resize_corner_size: 12.0,
        text_cursor: Default::default(),
        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: true,

        striped: false,

        slider_trailing_fill: false,

        handle_shape: HandleShape::Rect { aspect_ratio: 1.0 },

        interact_cursor: None,

        image_loading_spinners: true,

        numeric_color_space: NumericColorSpace::GammaByte,
    }
}

pub fn light_mode_override() -> Visuals {
    Visuals {
        dark_mode: false,
        override_text_color: Some(Color32::from_rgb(4, 3, 15)),
        widgets: Widgets::light(),
        selection: Selection::default(),
        hyperlink_color: Color32::from_rgb(0, 155, 255),
        faint_bg_color: Color32::from_additive_luminance(5), // visible, but barely so
        extreme_bg_color: Color32::from_gray(255),           // e.g. TextEdit background
        code_bg_color: Color32::from_gray(230),
        warn_fg_color: Color32::from_rgb(255, 100, 0), // slightly orange red. it's difficult to find a warning color that pops on bright background.
        error_fg_color: Color32::from_rgb(255, 0, 0),  // red

        window_shadow: Shadow {
            offset: vec2(10.0, 20.0),
            blur: 15.0,
            spread: 0.0,
            color: Color32::from_black_alpha(25),
        },
        window_fill: Color32::from_gray(255),
        window_stroke: Stroke::new(1.0, Color32::from_gray(190)),

        panel_fill: Color32::from_gray(255),

        popup_shadow: Shadow {
            offset: vec2(6.0, 10.0),
            blur: 8.0,
            spread: 0.0,
            color: Color32::from_black_alpha(25),
        },
        text_cursor: TextCursorStyle {
            stroke: Stroke::new(2.0, Color32::from_rgb(0, 83, 125)),
            ..Default::default()
        },
        ..Visuals::dark()
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CistercianClockApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for CistercianClockApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl CistercianClockApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

struct Colours {
    colour_0: Color32,
    colour_1: Color32,
    colour_2: Color32,
    colour_3: Color32,
    colour_4: Color32,
    colour_6: Color32,
}

fn paint_unit_number(
    painter: &mut Painter,
    centre: Pos2,
    scale: f32,
    colours: &Colours,
    number: u32,
) {
    let width = scale * (34.0 / 2.0 - 1.0);
    let stroke_width = if scale < 2.0 { 2.0 } else { scale * 1.0 };
    let Colours {
        colour_0,
        colour_1,
        colour_2,
        colour_3,
        colour_4,
        colour_6,
    } = colours;
    let stroke = Stroke::new(stroke_width, *colour_0);
    painter.line_segment(
        [centre - vec2(0.0, width), centre + vec2(0.0, width)],
        stroke,
    );

    if number == 1 || number == 5 || number == 7 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_1);
        painter.line_segment(
            [
                centre - vec2(0.0, width),
                centre + vec2(scale * 10.0, -width),
            ],
            stroke,
        );
    }

    if number == 2 || number == 8 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_2);
        painter.line_segment(
            [
                centre - vec2(0.0, width - scale * 10.0),
                centre + vec2(scale * 10.0, -width + (scale * 10.0)),
            ],
            stroke,
        );
    }

    if number == 3 {
        let stroke = Stroke::new(stroke_width, *colour_3);
        painter.line_segment(
            [
                centre - vec2(0.0, width),
                centre + vec2(scale * 10.0, -width + scale * 10.0),
            ],
            stroke,
        );
    }

    if number == 4 || number == 5 {
        let stroke = Stroke::new(stroke_width, *colour_4);
        painter.line_segment(
            [
                centre - vec2(0.0, width - scale * 10.0),
                centre + vec2(scale * 10.0, -width),
            ],
            stroke,
        );
    }

    if number > 5 {
        let stroke = Stroke::new(stroke_width, *colour_6);
        painter.line_segment(
            [
                centre + vec2(scale * 10.0, -width),
                centre + vec2(scale * 10.0, -width + scale * 10.0),
            ],
            stroke,
        );
    }
}

fn paint_tens_number(
    painter: &mut Painter,
    centre: Pos2,
    scale: f32,
    colours: &Colours,
    number: u32,
) {
    let width = scale * (34.0 / 2.0 - 1.0);
    let stroke_width = if scale < 2.0 { 2.0 } else { scale * 1.0 };
    let Colours {
        colour_1,
        colour_2,
        colour_3,
        colour_4,
        colour_6,
        ..
    } = colours;

    if number == 1 || number == 5 || number == 7 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_1);
        painter.line_segment(
            [
                centre - vec2(scale * 10.0, width),
                centre + vec2(0.0, -width),
            ],
            stroke,
        );
    }
    if number == 2 || number == 8 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_2);
        painter.line_segment(
            [
                centre - vec2(scale * 10.0, width - scale * 10.0),
                centre + vec2(0.0, -width + scale * 10.0),
            ],
            stroke,
        );
    }
    if number == 3 {
        let stroke = Stroke::new(stroke_width, *colour_3);
        painter.line_segment(
            [
                centre - vec2(scale * 10.0, width - scale * 10.0),
                centre + vec2(0.0, -width),
            ],
            stroke,
        );
    }
    if number == 4 || number == 5 {
        let stroke = Stroke::new(stroke_width, *colour_4);
        painter.line_segment(
            [
                centre - vec2(scale * 10.0, width),
                centre + vec2(0.0, -width + scale * 10.0),
            ],
            stroke,
        );
    }
    if number > 5 {
        let stroke = Stroke::new(stroke_width, *colour_6);
        painter.line_segment(
            [
                centre + vec2(-scale * 10.0, -width),
                centre + vec2(-scale * 10.0, -width + scale * 10.0),
            ],
            stroke,
        );
    }
}

fn paint_hundreds_number(
    painter: &mut Painter,
    centre: Pos2,
    scale: f32,
    colours: &Colours,
    number: u32,
) {
    let width = scale * (34.0 / 2.0 - 1.0);
    let stroke_width = if scale < 2.0 { 2.0 } else { scale * 1.0 };
    let Colours {
        colour_1,
        colour_2,
        colour_3,
        colour_4,
        colour_6,
        ..
    } = colours;

    if number == 1 || number == 5 || number == 7 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_1);
        painter.line_segment(
            [
                centre + vec2(0.0, width),
                centre + vec2(scale * 10.0, width),
            ],
            stroke,
        );
    }
    if number == 2 || number == 8 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_2);
        painter.line_segment(
            [
                centre + vec2(0.0, width - scale * 10.0),
                centre + vec2(scale * 10.0, width - scale * 10.0),
            ],
            stroke,
        );
    }
    if number == 3 {
        let stroke = Stroke::new(stroke_width, *colour_3);
        painter.line_segment(
            [
                centre + vec2(0.0, width),
                centre + vec2(scale * 10.0, width - scale * 10.0),
            ],
            stroke,
        );
    }
    if number == 4 || number == 5 {
        let stroke = Stroke::new(stroke_width, *colour_4);
        painter.line_segment(
            [
                centre + vec2(0.0, width - scale * 10.0),
                centre + vec2(scale * 10.0, width),
            ],
            stroke,
        );
    }
    if number > 5 {
        let stroke = Stroke::new(stroke_width, *colour_6);
        painter.line_segment(
            [
                centre + vec2(scale * 10.0, width - scale * 10.0),
                centre + vec2(scale * 10.0, width),
            ],
            stroke,
        );
    }
}

fn paint_thousands_number(
    painter: &mut Painter,
    centre: Pos2,
    scale: f32,
    colours: &Colours,
    number: u32,
) {
    let width = scale * (34.0 / 2.0 - 1.0);
    let stroke_width = if scale < 2.0 { 2.0 } else { scale * 1.0 };
    let Colours {
        colour_1,
        colour_2,
        colour_3,
        colour_4,
        colour_6,
        ..
    } = colours;

    if number == 1 || number == 5 || number == 7 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_1);
        painter.line_segment(
            [
                centre + vec2(-scale * 10.0, width),
                centre + vec2(0.0, width),
            ],
            stroke,
        );
    }
    if number == 2 || number == 8 || number == 9 {
        let stroke = Stroke::new(stroke_width, *colour_2);
        painter.line_segment(
            [
                centre + vec2(-scale * 10.0, width - scale * 10.0),
                centre + vec2(0.0, width - scale * 10.0),
            ],
            stroke,
        );
    }
    if number == 3 {
        let stroke = Stroke::new(stroke_width, *colour_3);
        painter.line_segment(
            [
                centre + vec2(-scale * 10.0, width - scale * 10.0),
                centre + vec2(0.0, width),
            ],
            stroke,
        );
    }
    if number == 4 || number == 5 {
        let stroke = Stroke::new(stroke_width, *colour_4);
        painter.line_segment(
            [
                centre + vec2(-scale * 10.0, width),
                centre + vec2(0.0, width - scale * 10.0),
            ],
            stroke,
        );
    }
    if number > 5 {
        let stroke = Stroke::new(stroke_width, *colour_6);
        painter.line_segment(
            [
                centre + vec2(-scale * 10.0, width - scale * 10.0),
                centre + vec2(-scale * 10.0, width),
            ],
            stroke,
        );
    }
}

fn paint_number(
    ui: &mut Ui,
    colours: &Colours,
    number: u32,
    scale: Option<f32>,
    show_arabic_numeral: Option<bool>,
) {
    let scale = if let Some(value) = scale { value } else { 1.0 };
    assert!((0..=9_999).contains(&number));
    if let Some(true) = show_arabic_numeral {
        match number {
            0..=999 => ui.label(number.to_string()),
            _ => ui.label(format!("{},{:003}", number / 1000, number % 1000)),
        };
    }

    let size = Vec2::splat(scale * 34.0);
    let (response, mut painter) = ui.allocate_painter(size, Sense::hover());
    let rect = response.rect;
    let c = rect.center();

    let unit = number % 10;
    paint_unit_number(&mut painter, c, scale, colours, unit);
    if number > 9 {
        let tens = (number % 100) / 10;
        paint_tens_number(&mut painter, c, scale, colours, tens);
    }
    if number > 99 {
        let hundreds = (number % 1_000) / 100;
        paint_hundreds_number(&mut painter, c, scale, colours, hundreds);
    }
    if number > 999 {
        let thousands = (number % 10_000) / 1_000;
        paint_thousands_number(&mut painter, c, scale, colours, thousands);
    }
}

const DARK_CISTERCIAN_NUMERAL_COLOURS: Colours = Colours {
    colour_0: Color32::from_gray(242),
    colour_1: Color32::from_rgb(58, 134, 255),
    colour_2: Color32::from_rgb(251, 86, 7),
    colour_3: Color32::from_rgb(162, 106, 241),
    colour_4: Color32::from_rgb(255, 0, 110),
    colour_6: Color32::from_rgb(255, 190, 11),
};

const LIGHT_CISTERCIAN_NUMERAL_COLOURS: Colours = Colours {
    colour_0: Color32::from_rgb(4, 3, 15),
    colour_1: Color32::from_rgb(93, 93, 91),
    colour_2: Color32::from_rgb(0, 122, 94),
    colour_3: Color32::from_rgb(27, 42, 65),
    colour_4: Color32::from_rgb(150, 2, 0),
    colour_6: Color32::from_rgb(0, 122, 163),
};

fn paint_number_row(ui: &mut Ui, colours: &Colours, start: u32, end: u32) {
    ui.horizontal(|ui| {
        for number in start..end {
            ui.horizontal_top(|ui| paint_number(ui, colours, number, None, Some(true)));
        }
    });
}

impl eframe::App for CistercianClockApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.visuals().dark_mode {
                ctx.set_visuals(dark_mode_override());
            } else {
                ctx.set_visuals(light_mode_override());
            };

            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_switch(ui);
            });
        });

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("clock".into()), FontId::new(64.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            let colours = if ui.visuals().dark_mode {
                DARK_CISTERCIAN_NUMERAL_COLOURS
            } else {
                LIGHT_CISTERCIAN_NUMERAL_COLOURS
            };
            ui.ctx().request_repaint_after(Duration::new(1, 0));
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Cistercian Time");
            ui.add_space(30.0);

            let now = Local::now();
            let hours_minutes: u32 = now.hour() * 100 + now.minute();
            let seconds: u32 = now.second();
            let time = now.format("%H:%M %S").to_string();
            ui.horizontal(|ui| {
                paint_number(ui, &colours, hours_minutes, Some(4.0), None);
                paint_number(ui, &colours, seconds, Some(4.0), None);
            });
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.style_mut().override_text_style = Some(TextStyle::Name("clock".into()));
                ui.label(time)
            });
            ui.add_space(20.0);

            ui.separator();
            ScrollArea::vertical()
                .auto_shrink(false)
                .scroll_bar_visibility(ScrollBarVisibility::default())
                .show(ui, |ui| {
                    ui.heading("Cistercian Numbers");
                    ui.add_space(30.0);
                    paint_number_row(ui, &colours, 0, 10);
                    ui.add_space(30.0);
                    for tens in 1..10 {
                        paint_number_row(ui, &colours, 10 * tens, (tens + 1) * 10);
                        ui.add_space(15.0);
                    }

                    ui.add_space(30.0);
                    ui.horizontal(|ui| {
                        for number in 1..5 {
                            ui.horizontal_top(|ui| {
                                paint_number(ui, &colours, number * 100, None, Some(true));
                            });
                        }
                    });

                    ui.add_space(30.0);
                    ui.horizontal(|ui| {
                        for number in 1..5 {
                            ui.horizontal_top(|ui| {
                                paint_number(ui, &colours, number * 1_000, None, Some(true));
                            });
                        }
                    });

                    ui.add_space(30.0);
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            powered_by_egui_and_eframe(ui);
                            egui::warn_if_debug_build(ui);
                        });
                    });
                });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
