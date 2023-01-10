use crate::{
    micropython::buffer::StrBuffer,
    time::Instant,
    ui::{
        component::{Component, Event, EventCtx, Marquee, Never},
        display,
        geometry::{Offset, Rect},
        model_tr::theme,
    },
};

pub struct Title {
    area: Rect,
    title: StrBuffer,
    marquee: Marquee,
    centered: bool,
}

impl Title {
    pub fn new(title: StrBuffer) -> Self {
        Self {
            title,
            marquee: Marquee::new(title, theme::FONT_HEADER, theme::FG, theme::BG),
            area: Rect::zero(),
            centered: false,
        }
    }

    pub fn with_title_centered(mut self) -> Self {
        self.centered = true;
        self
    }

    /// Display title/header at the top left of the given area.
    /// Returning the painted height of the whole header.
    pub fn paint_header_left(title: StrBuffer, area: Rect) -> i16 {
        let text_heigth = theme::FONT_HEADER.text_height();
        let title_baseline = area.top_left() + Offset::y(text_heigth - 1);
        display::text_left(
            title_baseline,
            title.as_ref(),
            theme::FONT_HEADER,
            theme::FG,
            theme::BG,
        );
        text_heigth
    }

    /// Display title/header centered at the top of the given area.
    /// Returning the painted height of the whole header.
    pub fn paint_header_centered(title: StrBuffer, area: Rect) -> i16 {
        let text_heigth = theme::FONT_HEADER.text_height();
        let title_baseline = area.top_center() + Offset::y(text_heigth - 1);
        display::text_center(
            title_baseline,
            title.as_ref(),
            theme::FONT_HEADER,
            theme::FG,
            theme::BG,
        );
        text_heigth
    }
}

impl Component for Title {
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.area = bounds;
        self.marquee.place(bounds);
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if !self.marquee.is_animating() {
            self.marquee.start(ctx, Instant::now());
        }
        self.marquee.event(ctx, event)
    }

    fn paint(&mut self) {
        let width = theme::FONT_HEADER.text_width(self.title.as_ref());
        if width > self.area.width() {
            self.marquee.paint();
        } else if self.centered {
            Self::paint_header_centered(self.title, self.area);
        } else {
            Self::paint_header_left(self.title, self.area);
        }
    }
}

// DEBUG-ONLY SECTION BELOW

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for Title {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("Title");
        t.title(self.title.as_ref());
        t.close();
    }
}
