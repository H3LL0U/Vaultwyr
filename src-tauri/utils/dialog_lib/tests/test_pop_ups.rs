#[cfg(test)]
mod tests{
    use dialog_lib::show_error;
    use eframe;


    #[test]
    fn test_show_error() ->  eframe::Result {
        let result = show_error();
        result
    }


}