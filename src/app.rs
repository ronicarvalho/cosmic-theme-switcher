use cosmic::app::{Core, Task};
use cosmic::Element;

use crate::icons;
use crate::theme_io::{self, ThemeMode};

pub const APP_ID: &str = "dev.encoders.CosmicThemeSwitch";

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Toggle,
}

pub struct App {
    core: Core,
    theme: ThemeMode,
}

impl cosmic::Application for App {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let theme = theme_io::read();
        (Self { core, theme }, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Toggle => {
                let new = self.theme.opposite();
                match theme_io::write(new) {
                    Ok(()) => self.theme = new,
                    Err(err) => tracing::error!(?err, "failed to toggle COSMIC theme"),
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let bytes: &'static [u8] = match self.theme {
            ThemeMode::Light => icons::SUN_SVG,
            ThemeMode::Dark => icons::MOON_SVG,
        };
        let handle = cosmic::widget::icon::from_svg_bytes(bytes).symbolic(true);
        self.core
            .applet
            .icon_button_from_handle(handle)
            .on_press(Message::Toggle)
            .into()
    }
}
