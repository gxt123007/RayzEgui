#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

use eframe::egui::IconData;
use image;
use std::sync::Arc;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };

    // 自定义图标
    let icon_data = include_bytes!("../assets/rayz.jpg");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Jpeg).unwrap();
    let rgba_data = img.into_rgba8();
    let (width, height) = (rgba_data.width(), rgba_data.height());
    let rgba: Vec<u8> = rgba_data.into_raw();
    options.viewport.icon = Some(Arc::<IconData>::new(IconData {
        rgba,
        width,
        height,
    }));

    eframe::run_native(
        "RayzGui",
        options,
        // Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

struct MyApp {
    // name: String,
    // age: u32,
    page: Page,
    show_sidebar: bool,
}

//自定义字体
fn setup_custom_fonts(ctx: &egui::Context) {
    // 创建一个默认的字体定义对象
    let mut fonts = egui::FontDefinitions::default();

    //安装的字体支持.ttf和.otf文件
    //文件放在main.rs的同级目录下（可以自定义到其它目录）
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/msyh.ttc")),
    );

    // 将字体添加到 Proportional 字体族的第一个位置
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // 将字体添加到 Monospace 字体族的末尾
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // 将加载的字体设置到 egui 的上下文中
    ctx.set_fonts(fonts);
}

//导航枚举
enum Page {
    Test,
    Settings,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 安装自定义字体
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            // name: "Gxt".to_owned(),
            // age: 99,
            page: Page::Test,
            show_sidebar: true,
        }
    }
    // 切换主题
    // fn theme_switcher(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
    //     ui.horizontal(|ui| {
    //         if ui.button("Dark").clicked() {
    //             ctx.set_visuals(egui::Visuals::dark());
    //         }
    //         if ui.button("Light").clicked() {
    //             ctx.set_visuals(egui::Visuals::light());
    //         }
    //     });
    // }
    //左侧导航按钮，egui没有内置树控件，有需要可以自己实现

    // 左导航栏ui
    fn left_ui(&mut self, ui: &mut egui::Ui) {
        //一个垂直布局的ui，内部控件水平居中并对齐（填充全宽）
        ui.vertical_centered_justified(|ui| {
            if ui.button("测试").clicked() {
                self.page = Page::Test;
            }

            if ui.button("设置").clicked() {
                self.page = Page::Settings;
            }
            //根据需要定义其它按钮
        });
    }

    //根据导航显示页面
    fn show_page(&mut self, ui: &mut egui::Ui) {
        match self.page {
            Page::Test => {
                self.test_ui(ui);
            }
            Page::Settings => {
                //...
            }
        }
    }
    fn test_ui(&mut self, ui: &mut egui::Ui) {
        ui.image(egui::include_image!("../assets/ferris.png"));
    }

    //主框架布局
    fn main_ui(&mut self, ui: &mut egui::Ui) {
        // 添加面板的顺序非常重要，影响最终的布局
        egui::TopBottomPanel::top("top_panel").min_height(30.0).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    // .button(if self.show_sidebar { "≡" } else { "☰" })
                    .button(if self.show_sidebar { "<<" } else { "☰" })
                    .clicked()
                {
                    self.show_sidebar = !self.show_sidebar;
                }
                // ui.heading("Rayz");
            });
        });

        if self.show_sidebar {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.add_space(20.0);
                    ui.vertical_centered(|ui| {
                        // ui.heading("Rayz");
                        // 替换 ui.heading("Rayz") 为图片
                        let image = egui::Image::new(egui::include_image!("../assets/rayz.png"))
                            .max_width(100.0);
                        ui.add(image);
                    });
                    ui.add_space(20.0);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.left_ui(ui);
                    });
                });
        }

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show_inside(ui, |ui| {
                // ui.vertical_centered(|ui| {
                //     ui.heading("状态栏");
                // });
                ui.vertical_centered(|ui| {
                    ui.label("状态栏内容");
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("主面板");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("主面板内容");

                self.show_page(ui);
            });
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::WHITE);
        ctx.set_visuals(visuals);
        // ctx.set_visuals(egui::Visuals::dark());
        egui::CentralPanel::default()
            // .frame(egui::Frame::central_panel(&ctx.style()).fill(egui::Color32::from_rgb(0, 0, 0)))
            .show(ctx, |ui| {
                // ui.heading("My egui Application");
                // ui.horizontal(|ui| {
                //     let name_label = ui.label("Your name: ");
                //     ui.text_edit_singleline(&mut self.name)
                //         .labelled_by(name_label.id);
                // });
                // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                // if ui.button("Increment").clicked() {
                //     self.age += 1;
                // }
                // ui.label(format!("Hello '{}', age {}", self.name, self.age));

                // self.theme_switcher(ui, ctx);
                self.main_ui(ui); // 主框架布局
            });
    }
}
