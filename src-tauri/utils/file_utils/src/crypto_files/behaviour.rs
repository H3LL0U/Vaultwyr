///contains enums that are used to specify the behaviour that the program should have in some sceneraios
use dialog_lib::prebuilt_windows::close_popup;
pub enum OnErrorBehaviour{
    AskUser,
    TerminateOnError

}

#[derive(Debug)]
pub enum VaultwyrError{
    BadPath,
    BadPassword,
    FileWriteError,
    FileReadError,
    FileHashError,
    FileOpenError,
    PathSizeError,
    NotImplemented,
    EncryptionError,
    DecryptionError,
    FileDeletionError
}

impl VaultwyrError{

pub fn handle_file_write_error(on_error_behaviour: &OnErrorBehaviour, title: &str, message: &str) -> VaultwyrError {
    VaultwyrError::handle_generic_error(on_error_behaviour, title, message, VaultwyrError::FileWriteError)
    }


pub fn handle_generic_error(
    on_error_behaviour: &OnErrorBehaviour,
    title: &str,
    message: &str,
    error: VaultwyrError,
) -> VaultwyrError {
    match on_error_behaviour {
        OnErrorBehaviour::AskUser => {
            close_popup(title, message);
            error
        }
        OnErrorBehaviour::TerminateOnError => error,
    }
}
}




