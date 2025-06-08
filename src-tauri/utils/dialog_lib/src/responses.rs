



    use strum::IntoEnumIterator;
    use strum_macros::EnumIter; // Required for #[derive(EnumIter)]

    macro_rules! user_response {
        (
            $vis:vis enum $name:ident {
                $($variant:ident),* $(,)?
            }
        ) => {
            #[derive(Debug, Clone, Copy, EnumIter)]
            $vis enum $name {
                $($variant),*
            }
        };
    }

    
    user_response! {
        pub enum UserResponseClose {
            Close,
        }
    }

    user_response! {
        pub enum UserResponseYesNo {
            Yes,
            No
        }
    }

    user_response! {
        pub enum UserResponseYesNoCancel {
            Yes,
            No,
            Cancel
        }
    }

    user_response! {
        pub enum UserResponseSkipRetry {
            Skip,
            Retry
        }
    }

    user_response! {
        pub enum UserResponseTerminateRetry {
            Terminate,
            Retry
        }
    }

    user_response! {
        pub enum  UserResponseReplaceTerminateRetry{
            Replace,
            Terminate,
            Retry
        }
    }


