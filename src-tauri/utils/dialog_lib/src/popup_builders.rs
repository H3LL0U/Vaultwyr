

use eframe::egui;
use egui::{Button, Color32, Vec2, Widget, IconData};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::cell::RefCell;
use std::fmt::Debug;
use std::path::Path;
use std::rc::Rc;
use egui_alignments::Alignable;





fn load_icon<P: AsRef<Path>>(path: P) -> Result<IconData, String> {
    let image = image::open(path).map_err(|e| format!("Failed to open icon: {}", e))?;
    let image = image.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Ok(IconData { rgba, width, height })
}
fn is_mostly_white(color: &Color32) -> bool {
    let r = color.r() as f32;
    let g = color.g() as f32;
    let b = color.b() as f32;

    let brightness = 0.299 * r + 0.587 * g + 0.114 * b;
    
    // Threshold: 128 is a reasonable middle value for 0-255 range
    brightness > 128.0
}

pub fn enum_to_sender_and_receiver<T>() -> (Rc<RefCell<Option<T>>>, Rc<RefCell<Option<T>>>) {
    let sender = Rc::new(RefCell::new(None));
    let clone = Rc::clone(&sender);
    (sender, clone)
}


pub struct GenericPopUp<T>
where
    T: IntoEnumIterator + Copy,
{
    options: eframe::NativeOptions,
    title: String, 
    heading: String,
    button_collors: Vec<egui::Color32>,
    sender: Option<Rc<RefCell<Option<T>>>>,
    max_buttons_per_row: u8,

    // if extra confirmation is needed after selection just in case
    extra_confimation: bool,
    show_confirmation: bool,
    allowed_to_close: bool
}

impl<T> GenericPopUp<T>
where
    T: 'static + IntoEnumIterator + Copy + Debug,
{
    pub fn default() -> Self {
        let icon = load_icon("icons/icon.ico").unwrap_or_default(); //uses vaultwyr icon as default
        let default_title = "Title".to_string();
        let default_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_resizable(false)
        
        .with_inner_size([500.0, 200.0])
        .with_maximize_button(false)
        .with_fullsize_content_view(false)
        .with_minimize_button(false)
        .with_taskbar(false)
        .with_title(&default_title)
        .with_close_button(false)
        .with_icon(icon),
        
        ..Default::default()
    };

        Self {
            options:default_options,
            title: default_title,
            heading: "Please confirm your choice".to_owned(),
            button_collors: Vec::new(),
            sender:None,
            max_buttons_per_row: 3,
            extra_confimation: false,
            show_confirmation: false,
            allowed_to_close: false
        
        }
    }

    pub fn heading(mut self,heading: impl ToString) -> Self{
        self.heading = heading.to_string();
        self
    }

    pub fn button_collors(mut self,button_collors: Vec<egui::Color32>) -> Self{
        self.button_collors = button_collors;
        self
    }
    pub fn title(mut self,title: impl ToString) -> Self{
        self.title = title.to_string();
        self.options.viewport.title = Some(title.to_string());
        self
    }

    pub fn options(mut self, options: eframe::NativeOptions) -> Self{
        self.options = options;
        self
    }

    pub fn extra_confimation(mut self, enable: bool) -> Self{
        self.extra_confimation = enable;
        self.allowed_to_close = !enable;

        self
    }

    pub fn show(mut self) -> Option<T>{
    let (sender, reciever) = enum_to_sender_and_receiver::<T>();
    self.sender = Some(sender);

    let options = std::mem::take(&mut self.options);
    match eframe::run_native(
            "Confirm exit",
            options,
            Box::new(move |_cc| Ok(Box::new(self))),
        ) {
        Ok(_) => {},
        Err(_) => {return None;},
    };

    let user_response: Option<T> = {
    let borrowed = reciever.borrow();
    match *borrowed {
        Some(val) => Some(val),  
        None => None,
    }
    };

    

    user_response
    }

}
impl<T> eframe::App for GenericPopUp<T>
where
    T: IntoEnumIterator + Copy + std::fmt::Debug + 'static,
{
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let window_height = ui.available_height();
        ui.vertical_centered(|ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2]) 
                .max_height(window_height)
                .show(ui, |ui| {
                    let line_height: f32 = 17.0; 
                    let button_height = 30.0;
                    let margin_text_and_button = 1.0;

                    ui.label(
                        egui::RichText::new(&self.heading)
                            .size(line_height)
                            .color(egui::Color32::BLACK),
                    );

                    ui.add_space(margin_text_and_button);

                    let variants: Vec<T> = T::iter().collect();
                    ui.columns(variants.len().min(self.max_buttons_per_row as usize), |columns| {
                        for (i, variant) in variants.into_iter().enumerate() {
                            columns[i % self.max_buttons_per_row as usize].allocate_ui_with_layout(
                                egui::Vec2::ZERO,
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    let btn_color = self.button_collors.get(i).cloned().unwrap_or(egui::Color32::from_rgb(200, 30, 30));

                                    let btn_text_color = if is_mostly_white(&btn_color) {
                                        egui::Color32::BLACK
                                    } else {
                                        egui::Color32::WHITE
                                    };

                                    let btn = egui::Button::new(
                                        egui::RichText::new(format!("{:?}", variant))
                                            .size(17.0)
                                            .color(btn_text_color),
                                    )
                                    .fill(btn_color)
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                                    .min_size(egui::Vec2::new(50.0, button_height));

                                    if ui.add(btn).clicked() {
                                        if let Some(sender) = &self.sender {
                                            *sender.borrow_mut() = Some(variant);
                                        }

                                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
                                },
                            );
                        }
                    });
                });
        });
    });

    // Show confirmation dialog with clean enum name
    if self.extra_confimation && self.show_confirmation {
        let confirmation_text = if let Some(rc) = &self.sender {
            match &*rc.borrow() {
                Some(variant) => format!("Are you sure you want to {:?}?", variant),
                None => "Are you sure you want to nothing?".to_string(),
            }
        } else {
            "Are you sure you want to nothing?".to_string()
        };

        egui::Window::new(confirmation_text)
            .collapsible(false)
            .resizable(true)
            .auto_sized()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("No").clicked() {
                        self.show_confirmation = false;
                        self.allowed_to_close = false;
                    }

                    if ui.button("Yes").clicked() {
                        self.show_confirmation = false;
                        self.allowed_to_close = true;
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
    }

    // Handle window close
    if ctx.input(|i| i.viewport().close_requested()) {
        if self.allowed_to_close {
            // Closes without extra logic
        } else if self.extra_confimation {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.show_confirmation = true;
        }
    }
}


}