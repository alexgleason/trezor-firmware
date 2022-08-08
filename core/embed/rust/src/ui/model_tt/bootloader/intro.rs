use crate::ui::{component::{text::paragraphs::Paragraphs, Child, Component, Event, EventCtx, Pad}, display, display::Color, geometry::{LinearPlacement, Point, Rect}, model_tt::{
    bootloader::{
        theme::{button_bld_menu, TTBootloaderText, MENU, IMAGE_BG, ICON_BG},
        title::Title,
        ReturnToC,
    },
    theme::FONT_MEDIUM,
}};
use crate::ui::constant::screen;
use crate::trezorhal::time::{clear_acc, get_ticks, init_ticks};
use crate::ui::geometry::Offset;
use crate::ui::model_tt::theme::FONT_NORMAL;


use crate::ui::model_tt::{bootloader::title::TitleMsg, component::{Button, HoldToConfirm, HoldToConfirmMsg}, constant::{HEIGHT, WIDTH}, theme};

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IntroMsg {
    Menu = 1,
    Host = 2,
}
impl ReturnToC for IntroMsg {
    fn return_to_c(&self) -> u32 {
        *self as u32
    }
}

pub struct Intro {
    bg: Pad,
    title: Child<Title>,
    menu: Child<Button<&'static str>>,
    host: Child<HoldToConfirm<Title>>,
    text: Child<Paragraphs<&'static str>>,
}

impl Intro {
    pub fn new(bld_version: &'static str, vendor: &'static str, version: &'static str) -> Self {
        let p1 = Paragraphs::new()
            .add::<TTBootloaderText>(FONT_MEDIUM, version)
            .add::<TTBootloaderText>(FONT_MEDIUM, vendor)
            .with_placement(LinearPlacement::vertical().align_at_start());

        let mut instance = Self {
            bg: Pad::with_background(Color::rgb(0, 0, 0)),
            title: Child::new(Title::new(bld_version)),
            menu: Child::new(Button::with_icon(MENU).styled(button_bld_menu())),
            host: Child::new(HoldToConfirm::new(Title::new("aaa"))),
            text: Child::new(p1),
        };

        instance.bg.clear();
        instance
    }
}

impl Component for Intro {
    type Msg = HoldToConfirmMsg<TitleMsg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg
            .place(Rect::new(Point::new(0, 0), Point::new(WIDTH, HEIGHT)));
        self.title
            .place(Rect::new(Point::new(15, 24), Point::new(180, 40)));
        self.menu.place(Rect::new(
            Point::new(187, 15),
            Point::new(187 + 38, 15 + 38),
        ));
        self.host
            .place(Rect::new(Point::new(0, 0), Point::new(240, 240)));
        self.text
            .place(Rect::new(Point::new(15, 75), Point::new(225, 200)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        return self.host.event(ctx, event);
    }

    fn paint(&mut self) {
        self.host.paint();
        // init_ticks();
        // //display::icon(screen().center(), ICON_BG, theme::GREEN, theme::BLUE);
        // display::image(screen().center(), IMAGE_BG);
        //
        // get_ticks();
        // clear_acc();
        //
        // display::text(screen().top_left() + Offset::new(48, 10+18), "MY TREZOR", FONT_NORMAL, Color::rgb(0xff, 0xff, 0xff), Color::rgb(0,0,0),)
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.menu.bounds(sink);
    }
}
