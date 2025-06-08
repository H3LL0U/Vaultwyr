
use crate::responses::*;
use crate::popup_builders::*;
use egui::Color32;
 use strum::IntoEnumIterator;

pub fn build_default_window<T>(title: impl ToString, content: impl ToString) -> GenericPopUp<T>
where T: IntoEnumIterator + Copy + std::fmt::Debug + 'static,
{
    let pop_up = GenericPopUp::<T>::default()
    .heading(content.to_string())
    .title(title);

    pop_up
}


pub fn close_popup(title: impl ToString, content: impl ToString) -> Option<UserResponseClose> {
    build_default_window::<UserResponseClose>(title, content).show()
}


pub fn ask_yes_no(title: impl ToString, content: impl ToString) -> Option<UserResponseYesNo> {
    build_default_window::<UserResponseYesNo>(title, content)
    .button_collors(vec![Color32::from_rgb(20, 125, 20), Color32::from_rgb(255, 20, 20)])
    .show()    
}

pub fn ask_skip_retry(title: impl ToString, content: impl ToString) -> Option<UserResponseSkipRetry> {
    build_default_window::<UserResponseSkipRetry>(title, content)
    .button_collors(vec![Color32::from_rgb(125, 125, 125), Color32::from_rgb(125, 125, 255)])
    .show()    
}

pub fn ask_terminate_retry(title: impl ToString, content: impl ToString) -> Option<UserResponseTerminateRetry> {
    build_default_window::<UserResponseTerminateRetry>(title, content)
    .button_collors(vec![Color32::from_rgb(255, 125, 125), Color32::from_rgb(125, 125, 255)])
    .show()    
}



