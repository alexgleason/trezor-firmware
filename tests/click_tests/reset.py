from typing import TYPE_CHECKING

from shamir_mnemonic import shamir

from trezorlib import messages

from .. import buttons

if TYPE_CHECKING:
    from trezorlib.debuglink import DebugLink


def confirm_new_wallet(debug: "DebugLink") -> None:
    layout = debug.wait_layout()
    if debug.model == "T":
        assert layout.title() == "CREATE NEW WALLET"
        debug.click(buttons.OK, wait=True)
    elif debug.model == "R":
        assert layout.title() == "WALLET CREATION"
        debug.press_right(wait=True)
        debug.press_right(wait=True)


def confirm_read(debug: "DebugLink", title: str, hold: bool = False) -> None:
    layout = debug.read_layout()
    if title == "Caution":
        if debug.model == "T":
            assert "OK, I UNDERSTAND" in layout.str_content
        elif debug.model == "R":
            assert "use your backup to recover" in layout.text_content()
    elif title == "Success":
        # TODO: improve this
        assert any(
            text in layout.text_content()
            for text in (
                "success",
                "finished",
                "done",
                "has been created",
                "Keep it safe",
            )
        )
    elif title == "Checklist":
        assert "number of shares" in layout.text_content().lower()
    else:
        assert title.upper() in layout.title()

    if debug.model == "T":
        debug.click(buttons.OK, wait=True)
    elif debug.model == "R":
        if layout.page_count() > 1:
            debug.press_right(wait=True)
        if hold:
            # TODO: create debug.hold_right()?
            debug.press_yes()
        else:
            debug.press_right()
        debug.wait_layout()


def set_selection(debug: "DebugLink", button: tuple[int, int], diff: int) -> None:
    if debug.model == "T":
        layout = debug.read_layout()
        assert "NumberInputDialog" in layout.str_content
        for _ in range(diff):
            debug.click(button)
        debug.click(buttons.OK, wait=True)
    elif debug.model == "R":
        layout = debug.read_layout()
        if layout.title() in ("NUMBER OF SHARES", "THRESHOLD"):
            # Special info screens
            layout = debug.press_right(wait=True)
        assert "NumberInput" in layout.str_content
        if button == buttons.RESET_MINUS:
            for _ in range(diff):
                debug.press_left(wait=True)
        else:
            for _ in range(diff):
                debug.press_right(wait=True)
        debug.press_middle(wait=True)


def read_words(debug: "DebugLink", is_advanced: bool = False) -> list[str]:
    words: list[str] = []
    layout = debug.read_layout()

    if debug.model == "T":
        if is_advanced:
            assert layout.title().startswith("GROUP")
        else:
            assert layout.title().startswith("RECOVERY SHARE #")
    elif debug.model == "R":
        if is_advanced:
            assert "SHARE" in layout.text_content()
        else:
            assert layout.text_content().startswith("SHARE #")

    # Swiping through all the page and loading the words
    for _ in range(layout.page_count() - 1):
        words.extend(layout.seed_words())
        layout = debug.input(swipe=messages.DebugSwipeDirection.UP, wait=True)
    if debug.model == "T":
        words.extend(layout.seed_words())

    # It is hold-to-confirm
    # TODO: create debug.hold(ms)?
    debug.press_yes()

    return words


def confirm_words(debug: "DebugLink", words: list[str]) -> None:
    layout = debug.wait_layout()
    if debug.model == "T":
        assert "Select word" in layout.text_content()
        for _ in range(3):
            # "Select word 3 of 20"
            #              ^
            word_pos = int(layout.text_content().split()[2])
            # Unifying both the buttons and words to lowercase
            btn_texts = [
                text.lower() for text in layout.buttons.tt_select_word_button_texts()
            ]
            wanted_word = words[word_pos - 1].lower()
            button_pos = btn_texts.index(wanted_word)
            layout = debug.click(buttons.RESET_WORD_CHECK[button_pos], wait=True)
    elif debug.model == "R":
        assert "Select correct word" in layout.text_content()
        layout = debug.press_right(wait=True)
        for _ in range(3):
            # "SELECT 2ND WORD"
            #         ^
            word_pos = int(layout.title().split()[1][:-2])
            wanted_word = words[word_pos - 1].lower()

            while not layout.buttons.get_middle_select() == wanted_word:
                layout = debug.press_right(wait=True)

            layout = debug.press_middle(wait=True)


def validate_mnemonics(mnemonics: list[str], expected_ems: bytes) -> None:
    # We expect these combinations to recreate the secret properly
    # In case of click tests the mnemonics are always XofX so no need for combinations
    groups = shamir.decode_mnemonics(mnemonics)
    ems = shamir.recover_ems(groups)
    assert expected_ems == ems.ciphertext
