use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::{
    constants::SCREEN_SIZE,
    graphics::renderer::{GfxSubsystem, RendererManager},
};

pub struct Game {}
pub struct GameBuilder {}
impl Game {}
impl GameBuilder {
    pub fn build() {}
    fn init_sdl(&self) -> Result<(), String> {
        Ok(())
    }
    fn init_logger() {
        /*Init Logger*/
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d}: {l} - {m}\n")))
            .build("log/output.log");

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile.unwrap())))
            .build(
                Root::builder()
                    .appender("logfile")
                    .build(LevelFilter::Error),
            );

        let _ = log4rs::init_config(config.unwrap());
    }
}
