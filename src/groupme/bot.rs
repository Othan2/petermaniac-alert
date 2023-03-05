// mod crate::groupme::client;
pub use crate::groupme::client::GroupmeClient;
use crate::groupme::error::GroupmeError;

use std::rc::Rc;

/// Groupme Bot interface for posting messages.
#[derive(Debug)]
pub struct Bot {
    pub(crate) bot_id: String,
    pub(crate) client: Rc<GroupmeClient>,
}

impl Bot {
    /// Posts a message.
    ///
    /// # Errors
    ///
    /// Returns an `Err(GroupmeError::AuthError)`
    /// if the bot_id is not found.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use groupme_bot::{Groupme, Bot};
    ///
    /// let groupme = Groupme::new(None);
    /// let bot: Bot = groupme.bot("Some bot_id");
    ///
    /// bot.post("Hello from Rust!").unwrap();
    /// ```
    pub fn post(&self, text: &str) -> Result<(), GroupmeError> {
        let gm_client = &self.client;
        gm_client.post(&self.bot_id, text, None)?;
        Ok(())
    }
}
