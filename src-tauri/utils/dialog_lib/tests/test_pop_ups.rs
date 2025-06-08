#[cfg(test)]
mod tests{
    use dialog_lib::{show_error, UserResponseShowError};
    use eframe;
    

    #[test]
    fn test_show_error() -> Result<(), String> {
        let result = show_error("test_title", "test content");
        if result.is_some() {
            Ok(())
        } else {
            Err("Expected Some(UserResponseShowError), got None".into())
        }
    }


}