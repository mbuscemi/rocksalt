use serde::{ Serialize, Deserialize };

use super::Detail;
use crate::file_system::file::{ File, Named };
use crate::message::YewMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetFile<F: File> {
    pub file: F,
}

impl<F: 'static + File + Named + Clone> Detail for SetFile<F> {
    fn name() -> String { format!("{}{}", "setfile_", F::NAME) }

    fn transform(&self) -> YewMessage {
        YewMessage::SetFile(Box::new(self.file.clone()))
    }
}
