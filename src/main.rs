

use eframe::egui;


#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new().start(
            canvas,
            web_options,
            Box::new(|cc| Ok(Box::new(Program::new(cc)))),
        ).await;

        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p>ERROR: The app has crashed.</p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let _ = eframe::run_native(
        "School Project",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                inner_size: Some(egui::vec2(1280.0, 720.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Program::new(cc)))),
    );
}


struct Program {
    page: Page,
    dark_mode: bool,
}

impl eframe::App for Program {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header")
            .exact_height(47.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.visuals_mut().button_frame = false;
                    ui.style_mut().spacing.item_spacing = egui::vec2(29.0, 0.0);
                    if ui.button(egui::RichText::new("Plistr").heading()).clicked() {
                        self.page = Page::Home;
                    }
                    if ui.button("About Us").clicked() {
                        self.page = Page::AboutUs;
                    }
                    // if ui.button("Timeline").clicked() {
                    //     self.page = Page::Timeline;
                    // }
                    if ui.button("Deliverables").clicked() {
                        self.page = Page::Deliverables;
                    }
                    if ui.button("Contact").clicked() {
                        self.page = Page::Contact;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(if self.dark_mode { "☀" } else { "🌙" }).clicked() {
                            self.dark_mode = !self.dark_mode;
                            let mut visuals = ctx.style().visuals.clone();
                            if self.dark_mode {
                                apply_dark_mode(&mut visuals);
                            } else {
                                apply_light_mode(&mut visuals);
                            }
                            ctx.set_visuals(visuals);
                        }
                    });
                });
            });
        egui::TopBottomPanel::bottom("footer")
            .exact_height(47.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        match self.page {
                            Page::Home => {
                                if ui.button("Next: About Us").clicked() {
                                    self.page = Page::AboutUs;
                                }
                            }
                            Page::AboutUs => {
                                // if ui.button("Next: Timeline").clicked() {
                                //     self.page = Page::Timeline;
                                // }
                                if ui.button("Next: Deliverables").clicked() {
                                    self.page = Page::Deliverables;
                                }
                            }
                            Page::Timeline => {
                                if ui.button("Next: Deliverables").clicked() {
                                    self.page = Page::Deliverables;
                                }
                            }
                            Page::Deliverables => {
                                if ui.button("Next: Contact").clicked() {
                                    self.page = Page::Contact;
                                }
                            }
                            Page::Contact => {}
                        }
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                match self.page {
                    Page::Home => {
                        ui.label(egui::RichText::new(APP_NAME).size(59.0));
                        ui.separator();
                        egui::Frame {
                            inner_margin: egui::Margin::symmetric(59.0, 11.0),
                            ..Default::default()
                        }.show(ui, |ui| {
                            ui.add_space(29.0);
                            if ui.available_width() >= 700.0 {
                                let panel_size = egui::vec2(
                                    ui.available_width() * 0.5,
                                    ui.available_height(),
                                );
                                let layout = egui::Layout::top_down(egui::Align::Center);
                                ui.horizontal(|ui| {
                                    ui.allocate_ui_with_layout(panel_size, layout, |ui| {
                                        self.show_home_info(ui);
                                    });
                                    ui.allocate_ui_with_layout(panel_size, layout, |ui| {
                                        self.show_home_links(ui);
                                    });
                                });
                            } else {
                                self.show_home_info(ui);
                                self.show_home_links(ui);
                            }
                        });
                    }
                    Page::AboutUs => {
                        ui.label(egui::RichText::new("About Us").size(59.0));
                        ui.separator();
                        egui::Frame {
                            inner_margin: egui::Margin::symmetric(59.0, 11.0),
                            ..Default::default()
                        }.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.button_padding = egui::Vec2::ZERO;
                                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                                let month_width = ui.available_size_before_wrap().x / 3.0;
                                let layout = egui::Layout::top_down(egui::Align::Center);
                                ui.allocate_ui_with_layout(egui::vec2(month_width, 111.0), layout, |ui| {
                                    ui.heading("Matthew Norman");
                                    ui.weak("Developer");
                                    ui.separator();
                                    ui.label("As our developer, Matthew is responsible for the actual implementation of Plistr. He writes the code that runs not just the app, but also this very website.");
                                    ui.separator();
                                    ui.heading("Responsibilities:");
                                    ui.strong("Programming");
                                    ui.strong("Development");
                                });
                                ui.allocate_ui_with_layout(egui::vec2(month_width, 111.0), layout, |ui| {
                                    ui.heading("Sara Quintana");
                                    ui.weak("Designer");
                                    ui.separator();
                                    ui.label("As our designer, Sara is responsible for designing Plistr. Everything from the application interface to the user experience falls under her purview.");
                                    ui.separator();
                                    ui.heading("Responsibilities:");
                                    ui.strong("UI/UX Design");
                                    ui.strong("Wireframing");
                                });
                                ui.allocate_ui_with_layout(egui::vec2(month_width, 111.0), layout, |ui| {
                                    ui.heading("Snailey Dol");
                                    ui.weak("Researcher");
                                    ui.separator();
                                    ui.label("As our researcher, Snailey is responsible for information collection. Anything from running user tests, to analyzing the efficacy of the studying methodologies we support.");
                                    ui.separator();
                                    ui.heading("Responsibilities:");
                                    ui.strong("User Testing");
                                    ui.strong("Efficacy Analysis");
                                });
                            });
                        });
                    }
                    Page::Timeline => {
                        ui.label(egui::RichText::new("Timeline").size(59.0));
                        ui.separator();
                        egui::Frame {
                            inner_margin: egui::Margin::symmetric(59.0, 11.0),
                            ..Default::default()
                        }.show(ui, |ui| {
                            ui.add_space(29.0);
                            ui.weak("Hover over important dates to see more.");
                            ui.add_space(19.0);
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.button_padding = egui::Vec2::ZERO;
                                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                                let month_width = ui.available_size_before_wrap().x / 4.0;
                                let layout = egui::Layout::top_down(egui::Align::Center);
                                ui.allocate_ui_with_layout(egui::vec2(month_width, 111.0), layout, |ui| {
                                    ui.heading("September");
                                });
                                ui.allocate_ui_with_layout(egui::vec2(month_width, 111.0), layout, |ui| {
                                    ui.heading("October");
                                });
                                ui.allocate_ui_with_layout(egui::vec2(month_width, 111.0), layout, |ui| {
                                    ui.heading("November");
                                });
                                ui.allocate_ui_with_layout(egui::vec2(month_width, 111.0), layout, |ui| {
                                    ui.heading("December");
                                });
                            });
                            ui.add_space(19.0);
                            let width = ui.available_width();
                            ui.horizontal(|ui| {
                                ui.add_space(width * 0.25 * 0.3);
                                ui.heading("9/10/24").on_hover_ui(|ui| {
                                    ui.label("Initial ideation phase");
                                });
                                ui.add_space(width * 0.5 * 0.3);
                                ui.heading("11/8/24").on_hover_ui(|ui| {
                                    ui.label("Rough prototype commpleted");
                                });
                            });
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.add_space(width * 0.25 * 0.75);
                                ui.heading("9/19/24").on_hover_ui(|ui| {
                                    ui.label("Initial ideation");
                                });
                                ui.add_space(width * 0.5 * 0.3);
                                ui.heading("11/15/24").on_hover_ui(|ui| {
                                    ui.label("First prototype commpleted");
                                });
                            });
                        });
                    }
                    Page::Deliverables => {
                        ui.label(egui::RichText::new("Deliverables").size(59.0));
                            ui.separator();
                        egui::Frame {
                            inner_margin: egui::Margin::symmetric(59.0, 11.0),
                            ..Default::default()
                        }.show(ui, |ui| {
                            ui.add_space(29.0);
                            ui.heading("Work Breakdown Structure");
                            ui.separator();
                            if ui.link("View PDF ⎆").clicked() {
                                ctx.open_url(egui::OpenUrl::new_tab(WBS_URL));
                            }
                            ui.horizontal(|ui| {
                                ui.strong("COMPLETION DATE: ");
                                ui.label("10/25/2024");
                            });
                            ui.label(WBS_DESC);
                            ui.add_space(29.0);
                            ui.heading("Wireframes (Rough Prototype)");
                            ui.separator();
                            if ui.link("View PDF ⎆").clicked() {
                                ctx.open_url(egui::OpenUrl::new_tab(WIREFRAMES_URL));
                            }
                            ui.horizontal(|ui| {
                                ui.strong("COMPLETION DATE: ");
                                ui.label("11/8/2024");
                            });
                            ui.label(ROUGH_PROTO_DESC);
                            ui.add_space(29.0);
                            ui.heading("First Prototype");
                            ui.separator();
                            if ui.link("View PDF ⎆").clicked() {
                                ctx.open_url(egui::OpenUrl::new_tab(PROTOTYPE_URL));
                            }
                            ui.horizontal(|ui| {
                                ui.strong("COMPLETION DATE: ");
                                ui.label("11/15/2024");
                            });
                            ui.label(FIRST_PROTO_DESC);
                            ui.add_space(29.0);
                            ui.heading("...And more!");
                            ui.separator();
                            ui.horizontal_wrapped(|ui| {
                                ui.label("Because this is an ongoing project, more work will need to be done in the future. We plan on providing updates with deliverables here and on the discussion boards linked on our ");
                                if ui.link("Contact page").clicked() {
                                    self.page = Page::Contact;
                                }
                                ui.label(".");
                            });
                        });
                    }
                    Page::Contact => {
                        ui.label(egui::RichText::new("Contact").size(59.0));
                        ui.separator();
                        egui::Frame {
                            inner_margin: egui::Margin::symmetric(59.0, 11.0),
                            ..Default::default()
                        }.show(ui, |ui| {
                            ui.add_space(29.0);
                            ui.heading("Discussions");
                            if ui.link("Open discussion boards in a new tab").clicked() {
                                ctx.open_url(egui::OpenUrl::new_tab(DISCUSSIONS_URL));
                            }
                            ui.label("This site is heavily integrate with GitHub for hosting, content delivery, and communications. We have set up a discussion forum at the link above where you can view updates, ask questions, and chat with us.");
                        });
                    }
                }
            });
        });
    }
}

impl Program {
    fn new(cc: &eframe::CreationContext) -> Self {
        let mut visuals = egui::Visuals::light();
        apply_light_mode(&mut visuals);
        cc.egui_ctx.set_visuals(visuals);
        cc.egui_ctx.style_mut(|s| {
            use egui::FontFamily::Proportional;
            use egui::FontId;
            use egui::TextStyle::*;
            
            s.text_styles = [
                (Heading, FontId::new(37.0, Proportional)),
                (Body, FontId::new(29.0, Proportional)),
                (Monospace, FontId::new(29.0, Proportional)),
                (Button, FontId::new(23.0, Proportional)),
                (Small, FontId::new(19.0, Proportional)),
              ].into();
        });

        Self {
            page: Page::Home,
            dark_mode: false,
        }
    }

    fn show_home_info(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new(TAGLINE).size(43.0));
        ui.separator();
        ui.label(egui::RichText::new(DESCRIPTION).size(23.0));
    }

    fn show_home_links(&mut self, ui: &mut egui::Ui) {
        // if ui.link(egui::RichText::new("Timeline").size(37.0)).clicked() {
        //     self.page = Page::Timeline;
        // }
        // ui.add_space(29.0);
        if ui.link(egui::RichText::new("About Us").size(37.0)).clicked() {
            self.page = Page::AboutUs;
        }
        ui.add_space(29.0);
        if ui.link(egui::RichText::new("Deliverables").size(37.0)).clicked() {
            self.page = Page::Deliverables;
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    AboutUs,
    Timeline,
    Deliverables,
    Contact,
}

fn apply_dark_mode(visuals: &mut egui::Visuals) {
    *visuals = egui::Visuals::dark();
}

fn apply_light_mode(visuals: &mut egui::Visuals) {
    *visuals = egui::Visuals::light();
}


const APP_NAME: &str = "Plistr";
const TAGLINE: &str = "An audio-based social networking platform";
const DESCRIPTION: &str = "With a focus on productivity, Plistr aims to be an all-in-one studying/meditation tool for students, professionals, and anyone who wants to get some work done.";
const WBS_URL: &str = "https://github.com/mrnrm/plistr/blob/master/assets/wbs.pdf";
const WIREFRAMES_URL: &str = "https://github.com/mrnrm/plistr/blob/master/assets/wireframes.pdf";
const PROTOTYPE_URL: &str = "https://github.com/mrnrm/plistr/blob/master/assets/first_prototype.pdf";
const DISCUSSIONS_URL: &str = "https://github.com/mrnrm/plistr/discussions";
const WBS_DESC: &str = "An overview of all work that must be done to see our project through to completion. This documents details the categorization of tasks and the steps required to see them through.";
const ROUGH_PROTO_DESC: &str = "A set of wireframes that visually explain the initial idea for the project.";
const FIRST_PROTO_DESC: &str = "An interactive set of high fidelity wireframes detailing all the routes a user could take as s/he navigates Plistr.";
