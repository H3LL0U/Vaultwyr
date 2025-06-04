use eframe::egui;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::cell::RefCell;
use std::rc::Rc;

pub fn show_error() -> eframe::Result {
    let (sender, reciever) = enum_to_buf::<UserResponseShowError>();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(move |_cc| Ok(Box::new(GenericPopUp::<UserResponseShowError>::new(sender)))),
    )?;

    println!("Final selected: {:?}", reciever.borrow());

    Ok(())
}


pub fn enum_to_buf<T>() -> (Rc<RefCell<Option<T>>>, Rc<RefCell<Option<T>>>) {
    let buffer = Rc::new(RefCell::new(None));
    let clone = Rc::clone(&buffer);
    (buffer, clone)
}


#[derive(Debug, EnumIter, Clone, Copy)]
enum UserResponseShowError {
    Cancel,
    Confirm
}

struct GenericPopUp<T>
where
    T: IntoEnumIterator + Copy,
{
    heading: String,
    buffer: Rc<RefCell<Option<T>>>,
}

impl<T> GenericPopUp<T>
where
    T: IntoEnumIterator + Copy,
{
    pub fn new(buffer: Rc<RefCell<Option<T>>>) -> Self {
        Self {
            heading: "Please confirm your choice".to_owned(),
            buffer,
        }
    }
}

impl<T> eframe::App for GenericPopUp<T>
where
    T: IntoEnumIterator + Copy + std::fmt::Debug + 'static,
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(&self.heading);

                for variant in T::iter() {
                    if ui.button(format!("{:?}", variant)).clicked() {
                        println!("You selected: {:?}", variant);
                        *self.buffer.borrow_mut() = Some(variant);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            println!("requested close");


        }
    }
}
