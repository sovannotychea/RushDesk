
use eframe::egui;
use egui::{Color32, RichText, Vec2};

#[derive(Clone)]
struct User {
    name: String,
    email: String,
    department: String,
}

#[derive(PartialEq)]
enum Page {
    Dashboard,
    Users,
    Settings,
}

struct MyApp {
    page: Page,
    users: Vec<User>,
    name: String,
    email: String,
    department: String,
    search: String,
    selected: Option<usize>,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            page: Page::Dashboard,
            users: vec![
                User {
                    name: "John Smith".into(),
                    email: "john@example.com".into(),
                    department: "IT".into(),
                },
                User {
                    name: "Sarah Lee".into(),
                    email: "sarah@example.com".into(),
                    department: "HR".into(),
                },
                User {
                    name: "David Kim".into(),
                    email: "david@example.com".into(),
                    department: "Finance".into(),
                },
            ],
            name: String::new(),
            email: String::new(),
            department: "IT".into(),
            search: String::new(),
            selected: None,
            status: "Ready".into(),
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();

        style.spacing.item_spacing = Vec2::new(10.0, 10.0);
        style.spacing.button_padding = Vec2::new(14.0, 8.0);

        cc.egui_ctx.set_style(style);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self::default()
    }

    fn primary_button(ui: &mut egui::Ui, label: &str) -> bool {
        ui.add(
            egui::Button::new(
                RichText::new(label).color(Color32::WHITE)
            )
                .fill(Color32::from_rgb(59, 130, 246))
                .min_size(Vec2::new(90.0, 34.0))
        )
            .clicked()
    }

    fn card(ui: &mut egui::Ui, title: &str, value: String) {
        egui::Frame::group(ui.style())
            .show(ui, |ui| {
                ui.set_min_width(145.0);
                ui.label(
                    RichText::new(title)
                        .color(Color32::LIGHT_GRAY)
                );
                ui.heading(
                    RichText::new(value)
                        .size(28.0)
                        .color(Color32::WHITE)
                );
            });
    }

    fn sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .exact_width(190.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(12.0);

                ui.heading(
                    RichText::new("◆  MODERN APP")
                        .color(Color32::from_rgb(96, 165, 250))
                );

                ui.add_space(25.0);
                ui.label(
                    RichText::new("WORKSPACE")
                        .size(11.0)
                        .color(Color32::GRAY)
                );

                ui.add_space(8.0);

                if ui.selectable_label(
                    self.page == Page::Dashboard,
                    "▦   Dashboard"
                ).clicked() {
                    self.page = Page::Dashboard;
                }

                if ui.selectable_label(
                    self.page == Page::Users,
                    "♙   Users"
                ).clicked() {
                    self.page = Page::Users;
                }

                if ui.selectable_label(
                    self.page == Page::Settings,
                    "⚙   Settings"
                ).clicked() {
                    self.page = Page::Settings;
                }

                ui.with_layout(
                    egui::Layout::bottom_up(egui::Align::LEFT),
                    |ui| {
                        ui.separator();
                        ui.label(
                            RichText::new("Version 1.0.0")
                                .color(Color32::GRAY)
                                .size(11.0)
                        );
                    }
                );
            });
    }

    fn topbar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("topbar")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Management Console");

                    ui.with_layout(
                        egui::Layout::right_to_left(
                            egui::Align::Center
                        ),
                        |ui| {
                            ui.label("Administrator");
                            ui.label("●");
                        }
                    );
                });
            });
    }

    fn dashboard(&self, ui: &mut egui::Ui) {
        ui.heading("Dashboard");
        ui.label("Welcome to your management workspace.");

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            Self::card(
                ui,
                "Total Users",
                self.users.len().to_string()
            );

            Self::card(
                ui,
                "Departments",
                self.users.iter()
                    .map(|u| &u.department)
                    .collect::<std::collections::HashSet<_>>()
                    .len()
                    .to_string()
            );

            Self::card(
                ui,
                "Application",
                "Running".into()
            );
        });

        ui.add_space(25.0);

        ui.heading("Recent Users");

        egui::Grid::new("recent_users")
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Name");
                ui.strong("Email");
                ui.strong("Department");
                ui.end_row();

                for user in self.users.iter().take(5) {
                    ui.label(&user.name);
                    ui.label(&user.email);
                    ui.label(&user.department);
                    ui.end_row();
                }
            });
    }

    fn user_form(&mut self, ui: &mut egui::Ui) {
        ui.heading("User Information");

        ui.label("Full Name");
        ui.add(
            egui::TextEdit::singleline(&mut self.name)
                .hint_text("Enter full name")
                .desired_width(f32::INFINITY)
        );

        ui.label("Email Address");
        ui.add(
            egui::TextEdit::singleline(&mut self.email)
                .hint_text("Enter email")
                .desired_width(f32::INFINITY)
        );

        ui.label("Department");

        egui::ComboBox::from_id_source("department")
            .selected_text(&self.department)
            .show_ui(ui, |ui| {
                for dept in ["IT", "HR", "Finance", "Marketing"] {
                    ui.selectable_value(
                        &mut self.department,
                        dept.to_string(),
                        dept
                    );
                }
            });

        ui.add_space(10.0);

        if Self::primary_button(ui, "Save User") {
            if self.name.trim().is_empty()
                || self.email.trim().is_empty()
            {
                self.status = "Name and email are required".into();
            } else if !self.email.contains('@') {
                self.status = "Invalid email address".into();
            } else {
                let user = User {
                    name: self.name.trim().to_string(),
                    email: self.email.trim().to_string(),
                    department: self.department.clone(),
                };

                if let Some(index) = self.selected {
                    if index < self.users.len() {
                        self.users[index] = user;
                        self.status = "User updated".into();
                    }
                } else {
                    self.users.push(user);
                    self.status = "User created".into();
                }

                self.clear_form();
            }
        }

        if ui.button("Clear Form").clicked() {
            self.clear_form();
        }
    }

    fn clear_form(&mut self) {
        self.name.clear();
        self.email.clear();
        self.department = "IT".into();
        self.selected = None;
    }

    fn users_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("User Management");
        ui.label("Create, edit and manage application users.");

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("Search users...")
                    .desired_width(250.0)
            );

            if ui.button("New User").clicked() {
                self.clear_form();
            }

            if ui.button("Edit").clicked() {
                if let Some(index) = self.selected {
                    if let Some(user) = self.users.get(index) {
                        self.name = user.name.clone();
                        self.email = user.email.clone();
                        self.department = user.department.clone();
                    }
                }
            }

            if ui.button("Delete").clicked() {
                if let Some(index) = self.selected {
                    if index < self.users.len() {
                        self.users.remove(index);
                        self.clear_form();
                        self.status = "User deleted".into();
                    }
                }
            }
        });

        ui.add_space(10.0);

        ui.columns(2, |columns| {
            columns[0].heading("User List");

            egui::ScrollArea::vertical()
                .show(&mut columns[0], |ui| {
                    egui::Grid::new("user_table")
                        .striped(true)
                        .show(ui, |ui| {
                            ui.strong("Name");
                            ui.strong("Department");
                            ui.end_row();

                            let mut clicked = None;

                            for (index, user) in self.users.iter().enumerate() {
                                let matches =
                                    user.name.to_lowercase().contains(
                                        &self.search.to_lowercase()
                                    )
                                        || user.email.to_lowercase().contains(
                                        &self.search.to_lowercase()
                                    );

                                if !matches {
                                    continue;
                                }

                                if ui.selectable_label(
                                    self.selected == Some(index),
                                    &user.name
                                ).clicked() {
                                    clicked = Some(index);
                                }

                                ui.label(&user.department);
                                ui.end_row();
                            }

                            if let Some(index) = clicked {
                                self.selected = Some(index);
                            }
                        });
                });

            columns[1].add_space(5.0);
            self.user_form(&mut columns[1]);
        });

        ui.add_space(10.0);
        ui.label(
            RichText::new(format!("Status: {}", self.status))
                .color(Color32::LIGHT_BLUE)
        );
    }

    fn settings(&self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("Application configuration");
        ui.separator();
        ui.label("Theme: Dark");
        ui.label("Framework: Rust egui");
        ui.label("Platform: Windows Desktop");
    }
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.sidebar(ctx);
        self.topbar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            match self.page {
                Page::Dashboard => self.dashboard(ui),
                Page::Users => self.users_page(ui),
                Page::Settings => self.settings(ui),
            }
        });

        egui::TopBottomPanel::bottom("statusbar")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("● Connected");
                    ui.separator();
                    ui.label(&self.status);
                });
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1150.0, 750.0])
            .with_min_inner_size([800.0, 550.0])
            .with_title("Modern Rust Desktop Application"),
        ..Default::default()
    };

    eframe::run_native(
        "Modern Rust Desktop Application",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
