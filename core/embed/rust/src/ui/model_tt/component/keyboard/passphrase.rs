use crate::ui::{
    component::{base::ComponentExt, Child, Component, Event, EventCtx, Never},
    display,
    display::toif::Icon,
    geometry::{Grid, Insets, Offset, Rect},
    model_tt::component::{
        button::{Button, ButtonContent, ButtonMsg},
        keyboard::common::{
            paint_pending_marker, MultiTapKeyboard, TextBox, HEADER_HEIGHT, HEADER_PADDING_BOTTOM,
            HEADER_PADDING_SIDE,
        },
        swipe::{Swipe, SwipeDirection},
        theme, ScrollBar,
    },
};

pub enum PassphraseKeyboardMsg {
    Confirmed,
    Cancelled,
}

pub struct PassphraseKeyboard {
    page_swipe: Swipe,
    input: Child<Input>,
    back: Child<Button<&'static str>>,
    confirm: Child<Button<&'static str>>,
    keys: [Child<Button<&'static str>>; KEY_COUNT],
    scrollbar: ScrollBar,
    fade: bool,
}

const STARTING_PAGE: usize = 1;
const PAGE_COUNT: usize = 4;
const KEY_COUNT: usize = 10;
#[rustfmt::skip]
const KEYBOARD: [[&str; KEY_COUNT]; PAGE_COUNT] = [
    ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
    [" ", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz", "*#"],
    [" ", "ABC", "DEF", "GHI", "JKL", "MNO", "PQRS", "TUV", "WXYZ", "*#"],
    ["_<>", ".:@", "/|\\", "!()", "+%&", "-[]", "?{}", ",'`", ";\"~", "$^="],
    ];

const MAX_LENGTH: usize = 50;

impl PassphraseKeyboard {
    pub fn new() -> Self {
        Self {
            page_swipe: Swipe::horizontal(),
            input: Input::new().into_child(),
            confirm: Button::with_icon(Icon::new(theme::ICON_CONFIRM))
                .styled(theme::button_confirm())
                .into_child(),
            back: Button::with_icon_blend(
                Icon::new(theme::IMAGE_BG_BACK_BTN),
                Icon::new(theme::ICON_BACK),
                Offset::new(30, 12),
            )
            .styled(theme::button_reset())
            .initially_enabled(false)
            .with_long_press(theme::ERASE_HOLD_DURATION)
            .into_child(),
            keys: KEYBOARD[STARTING_PAGE]
                .map(|text| Child::new(Button::new(Self::key_content(text)))),
            scrollbar: ScrollBar::horizontal(),
            fade: false,
        }
    }

    fn key_text(content: &ButtonContent<&'static str>) -> &'static str {
        match content {
            ButtonContent::Text(text) => text,
            ButtonContent::Icon(_) => " ",
            ButtonContent::IconAndText(_) => " ",
            ButtonContent::Empty => "",
            ButtonContent::IconBlend(_, _, _) => "",
        }
    }

    fn key_content(text: &'static str) -> ButtonContent<&'static str> {
        match text {
            " " => ButtonContent::Icon(Icon::new(theme::ICON_SPACE)),
            t => ButtonContent::Text(t),
        }
    }

    fn on_page_swipe(&mut self, ctx: &mut EventCtx, swipe: SwipeDirection) {
        // Change the page number.
        let key_page = self.scrollbar.active_page;
        let key_page = match swipe {
            SwipeDirection::Left => (key_page as isize + 1) as usize % PAGE_COUNT,
            SwipeDirection::Right => (key_page as isize - 1) as usize % PAGE_COUNT,
            _ => key_page,
        };
        self.scrollbar.go_to(key_page);
        // Clear the pending state.
        self.input
            .mutate(ctx, |ctx, i| i.multi_tap.clear_pending_state(ctx));
        // Update buttons.
        self.replace_button_content(ctx, key_page);
        // Reset backlight to normal level on next paint.
        self.fade = true;
        // So that swipe does not visually enable the input buttons when max length
        // reached
        self.update_input_btns_state(ctx);
    }

    fn replace_button_content(&mut self, ctx: &mut EventCtx, page: usize) {
        for (i, btn) in self.keys.iter_mut().enumerate() {
            let text = KEYBOARD[page][i];
            let content = Self::key_content(text);
            btn.mutate(ctx, |ctx, b| b.set_content(ctx, content));
            btn.request_complete_repaint(ctx);
        }
    }

    /// Possibly changing the buttons' state after change of the input.
    fn after_edit(&mut self, ctx: &mut EventCtx) {
        self.update_back_btn_state(ctx);
        self.update_input_btns_state(ctx);
    }

    /// When the input is empty, disable the back button.
    fn update_back_btn_state(&mut self, ctx: &mut EventCtx) {
        if self.input.inner().textbox.is_empty() {
            self.back.mutate(ctx, |ctx, b| b.disable(ctx));
        } else {
            self.back.mutate(ctx, |ctx, b| b.enable(ctx));
        }
    }

    /// When the input has reached max length, disable all the input buttons.
    fn update_input_btns_state(&mut self, ctx: &mut EventCtx) {
        for btn in self.keys.iter_mut() {
            btn.mutate(ctx, |ctx, b| {
                if self.input.inner().textbox.is_full() {
                    b.disable(ctx);
                } else {
                    b.enable(ctx);
                }
            });
        }
    }

    pub fn passphrase(&self) -> &str {
        self.input.inner().textbox.content()
    }
}

impl Component for PassphraseKeyboard {
    type Msg = PassphraseKeyboardMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        let bounds = bounds.inset(theme::borders());

        let (input_area, key_grid_area) = bounds.split_top(HEADER_HEIGHT + HEADER_PADDING_BOTTOM);

        let (input_area, scroll_area) =
            input_area.split_bottom(ScrollBar::DOT_SIZE + theme::KEYBOARD_SPACING);
        let (scroll_area, _) = scroll_area.split_top(ScrollBar::DOT_SIZE);
        let input_area = input_area.inset(Insets::sides(HEADER_PADDING_SIDE));

        let key_grid = Grid::new(key_grid_area, 4, 3).with_spacing(theme::KEYBOARD_SPACING);
        let confirm_btn_area = key_grid.cell(11);
        let back_btn_area = key_grid.cell(9);

        self.page_swipe.place(bounds);
        self.input.place(input_area);
        self.confirm.place(confirm_btn_area);
        self.back.place(back_btn_area);
        self.scrollbar.place(scroll_area);
        self.scrollbar
            .set_count_and_active_page(PAGE_COUNT, STARTING_PAGE);

        // Place all the character buttons.
        for (key, btn) in &mut self.keys.iter_mut().enumerate() {
            // Assign the keys in each page to buttons on a 5x3 grid, starting
            // from the second row.
            let area = key_grid.cell(if key < 9 {
                // The grid has 3 columns, and we skip the first row.
                key
            } else {
                // For the last key (the "0" position) we skip one cell.
                key + 1
            });
            btn.place(area);
        }

        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if self.input.inner().multi_tap.is_timeout_event(event) {
            self.input
                .mutate(ctx, |ctx, i| i.multi_tap.clear_pending_state(ctx));
            return None;
        }
        if let Some(swipe) = self.page_swipe.event(ctx, event) {
            // We have detected a horizontal swipe. Change the keyboard page.
            self.on_page_swipe(ctx, swipe);
            return None;
        }
        if let Some(ButtonMsg::Clicked) = self.confirm.event(ctx, event) {
            // Confirm button was clicked, we're done.
            return Some(PassphraseKeyboardMsg::Confirmed);
        }

        match self.back.event(ctx, event) {
            Some(ButtonMsg::Clicked) => {
                // Backspace button was clicked. If we have any content in the textbox, let's
                // delete the last character. Otherwise cancel.
                return if self.input.inner().textbox.is_empty() {
                    Some(PassphraseKeyboardMsg::Cancelled)
                } else {
                    self.input.mutate(ctx, |ctx, i| {
                        i.multi_tap.clear_pending_state(ctx);
                        i.textbox.delete_last(ctx);
                    });
                    self.after_edit(ctx);
                    None
                };
            }
            Some(ButtonMsg::LongPressed) => {
                self.input.mutate(ctx, |ctx, i| {
                    i.multi_tap.clear_pending_state(ctx);
                    i.textbox.clear(ctx);
                });
                self.after_edit(ctx);
                return None;
            }
            _ => {}
        }

        // Process key button events in case we did not reach maximum passphrase length.
        // (All input buttons should be disallowed in that case, this is just a safety
        // measure.)
        if !self.input.inner().textbox.is_full() {
            for (key, btn) in self.keys.iter_mut().enumerate() {
                if let Some(ButtonMsg::Clicked) = btn.event(ctx, event) {
                    // Key button was clicked. If this button is pending, let's cycle the pending
                    // character in textbox. If not, let's just append the first character.
                    let text = Self::key_text(btn.inner().content());
                    self.input.mutate(ctx, |ctx, i| {
                        let edit = i.multi_tap.click_key(ctx, key, text);
                        i.textbox.apply(ctx, edit);
                    });
                    self.after_edit(ctx);
                    return None;
                }
            }
        }
        None
    }

    fn paint(&mut self) {
        self.input.paint();
        self.scrollbar.paint();
        self.confirm.paint();
        self.back.paint();
        for btn in &mut self.keys {
            btn.paint();
        }
        if self.fade {
            self.fade = false;
            // Note that this is blocking and takes some time.
            display::fade_backlight(theme::BACKLIGHT_NORMAL);
        }
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.input.bounds(sink);
        self.scrollbar.bounds(sink);
        self.confirm.bounds(sink);
        self.back.bounds(sink);
        for btn in &self.keys {
            btn.bounds(sink)
        }
    }
}

struct Input {
    area: Rect,
    textbox: TextBox<MAX_LENGTH>,
    multi_tap: MultiTapKeyboard,
}

impl Input {
    fn new() -> Self {
        Self {
            area: Rect::zero(),
            textbox: TextBox::empty(),
            multi_tap: MultiTapKeyboard::new(),
        }
    }
}

impl Component for Input {
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.area = bounds;
        self.area
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: Event) -> Option<Self::Msg> {
        None
    }

    fn paint(&mut self) {
        let style = theme::label_keyboard();

        let mut text_baseline = self.area.top_left() + Offset::y(style.text_font.text_height())
            - Offset::y(style.text_font.text_baseline());

        let text = self.textbox.content();

        // Preparing the new text to be displayed.
        // Possible optimization is to redraw the background only when pending character
        // is replaced, or only draw rectangle over the pending character and
        // marker.
        display::rect_fill(self.area, theme::BG);

        // Find out how much text can fit into the textbox.
        // Accounting for the pending marker, which draws itself one pixel longer than
        // the last character
        let available_area_width = self.area.width() - 1;
        let text_to_display = if style.text_font.text_width(text) <= available_area_width {
            text // whole text can fit
        } else {
            // Text is longer, showing its right end with ellipsis at the beginning.
            let ellipsis = "...";
            let ellipsis_width = style.text_font.text_width(ellipsis);

            // Drawing the ellipsis and moving the baseline for the rest of the text.
            display::text(
                text_baseline,
                ellipsis,
                style.text_font,
                style.text_color,
                style.background_color,
            );
            text_baseline = text_baseline + Offset::x(ellipsis_width);

            // Finding out how many additional text characters will fit in,
            // starting from the right end.
            let remaining_available_width = available_area_width - ellipsis_width;
            let chars_from_right = style
                .text_font
                .longest_suffix(remaining_available_width, text);

            &text[text.len() - chars_from_right..]
        };

        display::text(
            text_baseline,
            text_to_display,
            style.text_font,
            style.text_color,
            style.background_color,
        );

        // Paint the pending marker.
        if self.multi_tap.pending_key().is_some() {
            paint_pending_marker(
                text_baseline,
                text_to_display,
                style.text_font,
                style.text_color,
            );
        }
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        sink(self.area)
    }
}

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for PassphraseKeyboard {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("PassphraseKeyboard");
        t.close();
    }
}
