use std::collections::HashMap;

use eframe::{
    egui::{
        Align2, Button, CentralPanel, Color32, CtxRef, FontDefinitions, Grid, Label, ScrollArea,
        TopBottomPanel, Ui, Window,
    },
    epi,
};
use url::Url;

use crate::config::{create_config, read_config, write_config, Config, Storage, StorageItem};

pub struct App {
    config_path: String,
    config: Config,
    path: String,
    state: State,
}

struct State {
    add: Add,
    checkbox: HashMap<String, bool>,
}

struct Add {
    name: String,
    url: String,
    open: bool,
    closable: bool,
    validated: bool,
    advice: String,
}

impl App {
    // fn jump(&mut self, path: &str) {
    //     self.path = path.to_string();
    // }

    fn set_fonts(&self, ctx: &CtxRef) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Ubuntu-Light".to_owned(),
            std::borrow::Cow::Borrowed(include_bytes!("../asset/STFANGSO.TTF")),
        );
        // fonts.family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, 20.0));
        // fonts.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 20.0));
        ctx.set_fonts(fonts);
    }

    fn render_top_bar(&mut self, ctx: &CtxRef) {
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("+").on_hover_text("新增").clicked() {
                    self.state.add.open = true;
                    self.state.add.closable = true;
                }
                if ui.button("-").on_hover_text("删除").clicked() {
                    let Storage { private, public } = &mut self.config.storage;
                    self.state
                        .checkbox
                        .iter()
                        .filter(|item| *item.1)
                        .map(|item| item.0)
                        .for_each(|name| {
                            if let Some(index) = private
                                .iter()
                                .position(|value| *value.name == name.to_string())
                            {
                                private.remove(index);
                            }
                            if let Some(index) = public
                                .iter()
                                .position(|value| *value.name == name.to_string())
                            {
                                public.remove(index);
                            }
                        })
                }
                if ui.add(Button::new("下载")).clicked() {
                    let Storage { private, public } = self.config.storage.clone();
                    let mut list: Vec<StorageItem> = vec![];
                    list.extend(private);
                    list.extend(public);
                    self.state
                        .checkbox
                        .iter()
                        .filter(|item| *item.1)
                        .map(|item| item.0)
                        .for_each(|name| {
                            let storage_item = list
                                .iter()
                                .find(|item| item.name == name.to_string())
                                .unwrap();
                            let url = &storage_item.url.clone();
                            match open::that(url) {
                                Ok(()) => println!("Opened '{}' successfully.", url),
                                Err(err) => {
                                    panic!("An error occurred when opening '{}': {}", url, err)
                                }
                            }
                        });
                    self.state
                        .checkbox
                        .iter_mut()
                        .for_each(|item| *item.1 = false);
                }
            });
        });
    }

    fn render_add_window(&mut self, ctx: &CtxRef) {
        let Add {
            name,
            url,
            open,
            closable,
            validated,
            advice,
        } = &mut self.state.add;
        let private = &mut self.config.storage.private;
        let checkbox = &mut self.state.checkbox;

        let validate = |list: &Vec<StorageItem>, target: &StorageItem| -> (bool, &str) {
            let StorageItem { name, url } = target;
            if name.len() == 0 || url.len() == 0 {
                return (false, "必须输入名称和链接");
            }
            if list.iter().find(|x| x.name == target.name).is_some() {
                return (false, "名称重复");
            }
            if Url::parse(target.url.as_str()).is_err() {
                return (false, "链接格式错误");
            }
            if list.iter().find(|x| x.url == target.url).is_some() {
                return (false, "链接重复");
            }
            (true, "advice")
        };

        if !open.clone() {
            return;
        }

        Window::new("新增")
            .collapsible(false)
            .resizable(false)
            .open(closable)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                Grid::new("add")
                    .num_columns(2)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("名称");
                        ui.text_edit_singleline(name);
                        ui.end_row();
                        ui.label("链接");
                        ui.text_edit_singleline(url);
                        ui.end_row();
                        if ui.button("提交").clicked() {
                            let target = StorageItem {
                                name: name.clone(),
                                url: url.clone(),
                            };
                            let (v, a) = validate(private, &target);
                            *validated = v;
                            *advice = a.to_string();
                            if v {
                                private.push(target);
                                checkbox.insert(name.to_string(), false);
                                *name = String::new();
                                *url = String::new();
                                *open = false;
                            }
                        }
                        ui.scope(|ui| {
                            ui.set_visible(!validated.clone());
                            ui.add(Label::new(&advice).text_color(Color32::RED));
                        });
                    });
            });
    }

    fn render_storage(&mut self, ctx: &CtxRef) {
        let Self {
            config_path: _,
            config,
            path,
            state,
        } = self;

        if path != "storage" {
            return;
        }

        let checkbox = &mut state.checkbox;

        CentralPanel::default().show(&ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                Grid::new("storage").show(ui, |ui| {
                    let mut render = |list: &Vec<StorageItem>, name: &str, ui: &mut Ui| {
                        let mut render_row = |row: &StorageItem, ui: &mut Ui| {
                            let StorageItem { name, url } = row;
                            let checked = checkbox.get_mut(name).unwrap();
                            ui.checkbox(checked, name);
                            ui.label(url);
                            // if ui.add(Button::new("下载")).clicked() {
                            //     match open::that(url) {
                            //         Ok(()) => println!("Opened '{}' successfully.", url),
                            //         Err(err) => {
                            //             panic!("An error occurred when opening '{}': {}", url, err)
                            //         }
                            //     }
                            // }
                            ui.end_row();
                        };
                        ui.label(name);
                        ui.end_row();
                        list.iter().for_each(|row| {
                            render_row(row, ui);
                        });
                    };
                    render(&config.storage.public, "公共", ui);
                    render(&config.storage.private, "私有", ui);
                });
            });
        });
    }

    // fn render_bottom_bar(&mut self, ctx: &CtxRef) {
    //     egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
    //         if ui.button("storage").clicked() {
    //             self.jump("storage");
    //         }
    //     });
    // }
}

impl Default for App {
    fn default() -> Self {
        let json_path = create_config().unwrap_or_else(|_| {
            panic!("create config fail");
        });
        let json_path = json_path.to_str().unwrap();
        let config = read_config(json_path).unwrap_or_else(|_| {
            panic!("read config fail");
        });
        let mut checkbox: HashMap<String, bool> = HashMap::new();
        let mut init_checkbox = |list: &Vec<StorageItem>| {
            list.iter().for_each(|s| {
                checkbox.insert(s.name.to_owned(), false);
            });
        };
        init_checkbox(&config.storage.public);
        init_checkbox(&config.storage.private);
        Self {
            config_path: json_path.to_string(),
            config,
            path: String::from("storage"),
            state: State {
                add: Add {
                    name: String::new(),
                    url: String::new(),
                    open: false,
                    closable: false,
                    validated: false,
                    advice: String::new(),
                },
                checkbox,
            },
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "mark"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame<'_>) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        self.set_fonts(ctx);
        self.render_top_bar(ctx);
        self.render_storage(ctx);
        self.render_add_window(ctx);
        // self.render_bottom_bar(ctx);
    }

    fn on_exit(&mut self) {
        let Self {
            config_path,
            config,
            path: _,
            state: _,
        } = self;
        write_config(config_path, config).unwrap_or_else(|_| {
            eprintln!("save config fail");
        });
    }
}
