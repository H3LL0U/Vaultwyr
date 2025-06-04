///contains enums that are used to specify the behaviour that the program should have in some sceneraios

pub enum OnErrorBehaviour{
    AskUser,
    Retry,
    Skip,
    Error,
    Terminate,

}