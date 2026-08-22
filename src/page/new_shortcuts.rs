use crate::fl;
use lingmo::iced::{Alignment, Length};
use lingmo::{cosmic_theme, widget};
use std::any::Any;

static SCREENSHOT: &[u8] = include_bytes!("../../res/new-shortcuts.svg");

pub struct Page {
    handle: widget::svg::Handle,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            handle: widget::svg::Handle::from_memory(SCREENSHOT),
        }
    }
}

impl super::Page for Page {
    fn title(&self) -> String {
        fl!("new-shortcuts-page")
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn skippable(&self) -> bool {
        true
    }

    fn view(&self) -> lingmo::Element<'_, super::Message> {
        let cosmic_theme::Spacing { space_s, .. } = lingmo::theme::spacing();

        let description = widget::text::body(fl!("new-shortcuts-page", "description"))
            .align_x(lingmo::iced::Alignment::Center)
            .width(Length::Fill);

        widget::column::with_capacity(2)
            .push(description)
            .push(widget::svg(self.handle.clone()).width(Length::Fill))
            .align_x(Alignment::Center)
            .spacing(space_s)
            .into()
    }
}
