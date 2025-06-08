use eframe::egui;
use egui::{Button, Color32, Widget};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use egui_alignments::Alignable;

pub fn show_error(title: impl ToString, content: impl ToString) -> Option<UserResponseShowError> {
    let pop_up = GenericPopUp::<UserResponseShowError>::default()
    .heading(content.to_string())
    .title(title);

    pop_up.show()
}


pub fn enum_to_sender_and_receiver<T>() -> (Rc<RefCell<Option<T>>>, Rc<RefCell<Option<T>>>) {
    let sender = Rc::new(RefCell::new(None));
    let clone = Rc::clone(&sender);
    (sender, clone)
}








#[derive(Debug, EnumIter, Clone, Copy)]
pub enum UserResponseShowError {
    Close,
    Open,
    ETC
}

struct GenericPopUp<T>
where
    T: IntoEnumIterator + Copy,
{
    options: eframe::NativeOptions,
    title: String, 
    heading: String,
    button_collors: Vec<egui::Color32>,
    sender: Option<Rc<RefCell<Option<T>>>>,
}

impl<T> GenericPopUp<T>
where
    T: 'static + IntoEnumIterator + Copy + Debug,
{
    pub fn default() -> Self {
        let default_title = "Title".to_string();
        let default_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_resizable(false)
        
        .with_inner_size([500.0, 200.0])
        .with_maximize_button(false)
        .with_fullsize_content_view(false)
        .with_taskbar(false)
        .with_title(&default_title),
        
        ..Default::default()
    };

        Self {
            options:default_options,
            title: default_title,
            heading: "Please confirm your choice".to_owned(),
            button_collors: Vec::new(),
            sender:None,
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
        self
    }

    pub fn options(mut self, options: eframe::NativeOptions) -> Self{
        self.options = options;
        self
    }
    pub fn show(mut self) -> Option<T>{
    let (sender, reciever) = enum_to_sender_and_receiver::<T>();
    self.sender = Some(sender);

    let options = std::mem::take(&mut self.options);
    match eframe::run_native(
            "Confirm exit",
            options,
            Box::new(move |_cc| Ok(Box::new(self))), //the parameter type `T` may not live long enough
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
                    ui.columns(variants.len(), |columns| {
                        for (i, variant) in variants.into_iter().enumerate() {
                            columns[i].allocate_ui_with_layout(
                                egui::Vec2::ZERO,
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    let btn_color = self.button_collors.get(i).cloned().unwrap_or(egui::Color32::from_rgb(200, 30, 30));
                                    let btn = egui::Button::new(
                                        egui::RichText::new(format!("{:?}", variant))
                                            .size(17.0)
                                            .color(egui::Color32::WHITE),
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

    if ctx.input(|i| i.viewport().close_requested()) {
        //no extra logic when window is closing for now
    }
}


}