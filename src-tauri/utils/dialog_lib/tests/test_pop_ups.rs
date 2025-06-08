#[cfg(test)]
mod tests{
    use dialog_lib::prebuilt_windows::close_popup;
    use eframe;
    

    #[test]
    fn test_show_error() -> Result<(), String> {
        let result = close_popup("test_title", "test content");
        if result.is_some() {
            Ok(())
        } else {
            Err("Expected Some(UserResponseShowError), got None".into())
        }
    }


}