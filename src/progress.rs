use std::time::Duration;
use indicatif::{ProgressBar as PBar, ProgressStyle};

use crate::result::Result;

pub trait ProgressTrait {
    type ProgressResultType;

    fn update(&self, msg: &str) -> Result<Self::ProgressResultType>;
    fn done(&self, msg: &str) -> Result<Self::ProgressResultType>;
    fn done_without_indicator(&self, msg: &str) -> Result<Self::ProgressResultType>;
}

pub struct ProgressBar {
    bar: PBar,
}

impl ProgressBar {
    pub fn new() -> Self {
        let bar = PBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(80));
        bar.set_style(
            ProgressStyle::default_spinner()
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&["⢹", "⢺", "⢼", "⣸", "⣇", "⡧", "⡗", "✔"])
                .template("{spinner} {msg}").unwrap_or(ProgressStyle::default_spinner()),
        );

        Self { bar }
    }
}

impl ProgressTrait for ProgressBar {
    type ProgressResultType = ();

    fn update(&self, msg: &str) -> Result<Self::ProgressResultType> {
        self.bar.set_message(msg.to_owned());

        Ok(())
    }

    fn done(&self, msg: &str) -> Result<Self::ProgressResultType> {
        self.bar.finish_with_message(msg.to_owned());

        Ok(())
    }

    fn done_without_indicator(&self, msg: &str) -> Result<Self::ProgressResultType> {
        self.bar.finish_and_clear();
        if !msg.is_empty() {
            println!("{}", msg);
        }

        Ok(())
    }
}
