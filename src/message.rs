pub enum Message {
    String(String),
    Bool(bool),
    TwoStrings(String, String),
}

pub trait MessageSender {
    fn get_message(&mut self) -> Option<Message> {
        None
    }
}

pub trait MessageReceiver {
    fn handle_message(
        &mut self,
        _message: Option<Message>,
        _file_manager: &mut crate::file_manager::FileManager,
    ) {
    }
}
