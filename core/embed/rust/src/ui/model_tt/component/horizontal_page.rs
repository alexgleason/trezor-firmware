use crate::{
    error::Error,
    ui::{
        component::{
            base::ComponentExt, AuxPageMsg, Component, Event, EventCtx, Never, Pad, PageMsg,
            Paginate, Qr,
        },
        display::{self, Color},
        geometry::Rect,
    },
};

use super::{theme, ScrollBar, Swipe, SwipeDirection};

const SCROLLBAR_HEIGHT: i16 = 32;

pub struct HorizontalPage<T> {
    content: T,
    pad: Pad,
    swipe: Swipe,
    scrollbar: ScrollBar,
    swipe_right_to_go_back: bool,
    fade: Option<i32>,
}

impl<T> HorizontalPage<T>
where
    T: Paginate,
    T: Component,
{
    pub fn new(content: T, background: Color) -> Self {
        Self {
            content,
            swipe: Swipe::new(),
            pad: Pad::with_background(background),
            scrollbar: ScrollBar::horizontal(),
            swipe_right_to_go_back: false,
            fade: None,
        }
    }

    pub fn with_swipe_right_to_go_back(mut self) -> Self {
        self.swipe_right_to_go_back = true;
        self
    }

    pub fn inner(&self) -> &T {
        &self.content
    }

    fn setup_swipe(&mut self) {
        self.swipe.allow_left = self.scrollbar.has_next_page();
        self.swipe.allow_right = self.scrollbar.has_previous_page() || self.swipe_right_to_go_back;
    }

    fn on_page_change(&mut self, ctx: &mut EventCtx) {
        // Adjust the swipe parameters according to the scrollbar.
        self.setup_swipe();

        // Change the page in the content, make sure it gets completely repainted and
        // clear the background under it.
        self.content.change_page(self.scrollbar.active_page);
        self.content.request_complete_repaint(ctx);
        self.pad.clear();

        // Swipe has dimmed the screen, so fade back to normal backlight after the next
        // paint.
        self.fade = Some(theme::BACKLIGHT_NORMAL);
    }
}

impl<T> Component for HorizontalPage<T>
where
    T: Paginate,
    T: Component,
{
    type Msg = PageMsg<T::Msg, Never>;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.swipe.place(bounds);

        let (content, scrollbar) = bounds.split_bottom(SCROLLBAR_HEIGHT);
        self.pad.place(content);
        self.content.place(content);
        self.scrollbar.place(scrollbar);

        self.scrollbar
            .set_count_and_active_page(self.content.page_count(), 0);
        self.setup_swipe();

        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        ctx.set_page_count(self.scrollbar.page_count);
        if let Some(swipe) = self.swipe.event(ctx, event) {
            match swipe {
                SwipeDirection::Left => {
                    // Scroll down, if possible.
                    self.scrollbar.go_to_next_page();
                    self.on_page_change(ctx);
                    return None;
                }
                SwipeDirection::Right => {
                    if self.swipe_right_to_go_back && self.scrollbar.active_page == 0 {
                        return Some(PageMsg::Aux(AuxPageMsg::GoBack));
                    }
                    // Scroll up, if possible.
                    self.scrollbar.go_to_previous_page();
                    self.on_page_change(ctx);
                    return None;
                }
                _ => {
                    // Ignore other directions.
                }
            }
        }
        self.content.event(ctx, event).map(PageMsg::Content)
    }

    fn paint(&mut self) {
        self.pad.paint();
        self.content.paint();
        self.scrollbar.paint();
        if let Some(val) = self.fade.take() {
            // Note that this is blocking and takes some time.
            display::fade_backlight(val);
        }
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        sink(self.pad.area);
        self.scrollbar.bounds(sink);
        self.content.bounds(sink);
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for HorizontalPage<T>
where
    T: crate::trace::Trace,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("HorizontalPage");
        t.field("active_page", &self.scrollbar.active_page);
        t.field("page_count", &self.scrollbar.page_count);
        t.field("content", &self.content);
        t.close();
    }
}

use super::Frame;
use crate::ui::component::text::paragraphs::{
    Paragraph, ParagraphSource, ParagraphStrType, ParagraphVecShort, Paragraphs, VecExt,
};
use heapless::Vec;

pub struct AddressDetails<T> {
    qr_code: Frame<Qr, T>,
    details: Frame<Paragraphs<ParagraphVecShort<T>>, T>,
    xpub_view: Frame<Paragraphs<Paragraph<T>>, T>,
    xpubs: Vec<(T, T), 16>,
    current_page: usize,
}

impl<T> AddressDetails<T>
where
    T: ParagraphStrType + From<&'static str>,
{
    pub fn new(
        qr_address: T,
        case_sensitive: bool,
        account: Option<T>,
        path: Option<T>,
    ) -> Result<Self, Error> {
        let mut para = ParagraphVecShort::new();
        if let Some(a) = account {
            para.add(Paragraph::new(&theme::TEXT_NORMAL, "Account:".into()));
            para.add(Paragraph::new(&theme::TEXT_MONO, a));
        }
        if let Some(p) = path {
            para.add(Paragraph::new(
                &theme::TEXT_NORMAL,
                "\nDerivation path:".into(),
            ));
            para.add(Paragraph::new(&theme::TEXT_MONO, p));
        }
        let result = Self {
            qr_code: Frame::left_aligned(
                theme::label_title(),
                "RECEIVE ADDRESS".into(),
                Qr::new(qr_address, case_sensitive)?.with_border(7),
            )
            .with_border(theme::borders_horizontal_scroll()),
            details: Frame::left_aligned(
                theme::label_title(),
                "RECEIVE ADDRESS".into(),
                para.into_paragraphs(),
            )
            .with_border(theme::borders_horizontal_scroll()),
            xpub_view: Frame::left_aligned(
                theme::label_title(),
                " \n ".into(),
                Paragraph::new(&theme::TEXT_XPUB, "".into()).into_paragraphs(),
            )
            .with_border(theme::borders_horizontal_scroll()),
            xpubs: Vec::new(),
            current_page: 0,
        };
        Ok(result)
    }

    pub fn add_xpub(&mut self, title: T, xpub: T) -> Result<(), Error> {
        self.xpubs
            .push((title, xpub))
            .map_err(|_| Error::OutOfRange)
    }
}

impl<T> Paginate for AddressDetails<T>
where
    T: ParagraphStrType + Clone,
{
    fn page_count(&mut self) -> usize {
        2 + self.xpubs.len()
    }

    fn change_page(&mut self, to_page: usize) {
        self.current_page = to_page;
        if to_page > 1 {
            let i = to_page - 2;
            // Context is needed for updating child so that it can request repaint. In this
            // case the parent component that handles paging always requests complete
            // repaint after page change so we can use a dummy context here.
            let mut dummy_ctx = EventCtx::new();
            self.xpub_view
                .update_title(&mut dummy_ctx, self.xpubs[i].0.clone());
            self.xpub_view.update_content(&mut dummy_ctx, |p| {
                p.inner_mut().update(self.xpubs[i].1.clone());
                p.change_page(0)
            });
        }
    }
}

impl<T> Component for AddressDetails<T>
where
    T: ParagraphStrType,
{
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.qr_code.place(bounds);
        self.details.place(bounds);
        self.xpub_view.place(bounds);
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        match self.current_page {
            0 => self.qr_code.event(ctx, event),
            1 => self.details.event(ctx, event),
            _ => self.xpub_view.event(ctx, event),
        }
    }

    fn paint(&mut self) {
        match self.current_page {
            0 => self.qr_code.paint(),
            1 => self.details.paint(),
            _ => self.xpub_view.paint(),
        }
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        match self.current_page {
            0 => self.qr_code.bounds(sink),
            1 => self.details.bounds(sink),
            _ => self.xpub_view.bounds(sink),
        }
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for AddressDetails<T>
where
    T: ParagraphStrType + crate::trace::Trace,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("AddressDetails");
        match self.current_page {
            0 => self.qr_code.trace(t),
            1 => self.details.trace(t),
            _ => self.xpub_view.trace(t),
        }
        t.close();
    }
}
