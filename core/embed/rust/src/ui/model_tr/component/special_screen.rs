use crate::ui::{
    component::{Component, Event, EventCtx, Never},
    display::{text_center, text_left, Font, Icon},
    geometry::{self, Offset, Rect},
    model_tr::theme,
};

pub struct SpecialScreen {
    area: Rect,
}

impl SpecialScreen {
    pub fn new() -> Self {
        Self { area: Rect::zero() }
    }
}

impl Component for SpecialScreen {
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.area = bounds;
        self.area
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        None
    }

    fn paint(&mut self) {
        let icon = Icon::new(theme::ICON_LOGO);
        icon.draw(
            self.area.top_center() + Offset::new(0, 5),
            geometry::TOP_CENTER,
            theme::FG,
            theme::BG,
        );

        text_center(
            self.area.bottom_center() + Offset::new(0, -2),
            "Trezor Secret",
            Font::DEMIBOLD,
            theme::FG,
            theme::BG,
        );
    }
}

// DEBUG-ONLY SECTION BELOW

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for SpecialScreen {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("SpecialScreen");
        t.close();
    }
}
