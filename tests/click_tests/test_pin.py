# This file is part of the Trezor project.
#
# Copyright (C) 2012-2023 SatoshiLabs and contributors
#
# This library is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License version 3
# as published by the Free Software Foundation.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the License along with this library.
# If not, see <https://www.gnu.org/licenses/lgpl-3.0.html>.

from contextlib import contextmanager
from typing import TYPE_CHECKING, Generator

import pytest

from trezorlib import device

if TYPE_CHECKING:
    from ..device_handler import BackgroundDeviceHandler
    from trezorlib.debuglink import DebugLink, LayoutContent


pytestmark = [pytest.mark.skip_t1, pytest.mark.skip_t2]

PIN4 = "1234"
PIN24 = "875163065288639289952973"
PIN50 = "31415926535897932384626433832795028841971693993751"
PIN60 = PIN50 + "9" * 10

# TODO: bad PIN
# TODO: possibly trying to set it, not just enter it
# TODO: wipe-code as well?


@contextmanager
def prepare_pin_dialogue(
    device_handler: "BackgroundDeviceHandler",
) -> Generator["DebugLink", None, None]:
    debug = device_handler.debuglink()
    # So that random digits are always the same
    debug.reseed(0)
    # Any action triggering the PIN dialogue
    device_handler.run(device.apply_settings, auto_lock_delay_ms=300_000)  # type: ignore

    layout = debug.wait_layout()
    if debug.model == "T":
        assert "PinKeyboard" in layout.str_content
    elif debug.model == "R":
        assert "PinEntry" in layout.str_content

    yield debug

    # Finish the handler
    debug.press_right()
    device_handler.result()


def _input_pin(debug: "DebugLink", pin: str) -> None:
    """Input the PIN"""
    for digit in pin:
        _navigate_to_action_and_press(debug, f"Select({digit})")


def _see_pin(debug: "DebugLink") -> None:
    """Navigate to "SEE" and press it"""
    _navigate_to_action_and_press(debug, "Show")


def _delete_pin(debug: "DebugLink", digits_to_delete: int) -> None:
    """Navigate to "DELETE" and press it how many times requested"""
    for _ in range(digits_to_delete):
        _navigate_to_action_and_press(debug, "Delete")


def _confirm_pin(debug: "DebugLink") -> None:
    """Navigate to "ENTER" and press it"""
    _navigate_to_action_and_press(debug, "Confirm")


def _navigate_to_action_and_press(debug: "DebugLink", wanted_action: str) -> None:
    """Navigate to the button and press it"""
    # Orient
    actions = [
        "Delete",
        "Show",
        "Confirm",
        "Select(0)",
        "Select(1)",
        "Select(2)",
        "Select(3)",
        "Select(4)",
        "Select(5)",
        "Select(6)",
        "Select(7)",
        "Select(8)",
        "Select(9)",
    ]
    if wanted_action not in actions:
        raise ValueError(f"Action {wanted_action} is not supported in PIN entry")

    # Navigate
    layout = debug.read_layout()
    current_action = layout.buttons.get_middle_action()
    while current_action != wanted_action:
        layout = _move_one_closer(
            debug, wanted_action, current_action, actions, is_carousel=True
        )
        current_action = layout.buttons.get_middle_action()

    # Press
    debug.press_middle(wait=True)


def _move_one_closer(
    debug: "DebugLink",
    wanted_action: str,
    current_action: str,
    all_actions: list[str],
    is_carousel: bool,
) -> "LayoutContent":
    """Pressing either left or right regarding to the current situation"""
    index_diff = all_actions.index(wanted_action) - all_actions.index(current_action)
    if not is_carousel:
        # Simply move according to the index in a closed list
        if index_diff > 0:
            return debug.press_right(wait=True)
        else:
            return debug.press_left(wait=True)
    else:
        # Carousel can move in a circle - over the edges
        # Always move the shortest way
        action_half = len(all_actions) // 2
        if index_diff > action_half or -action_half < index_diff < 0:
            return debug.press_left(wait=True)
        else:
            return debug.press_right(wait=True)


@pytest.mark.setup_client(pin=PIN4)
def test_pin_short(device_handler: "BackgroundDeviceHandler"):
    with prepare_pin_dialogue(device_handler) as debug:
        _input_pin(debug, PIN4)
        _see_pin(debug)
        _confirm_pin(debug)


@pytest.mark.setup_client(pin=PIN24)
def test_pin_long(device_handler: "BackgroundDeviceHandler"):
    with prepare_pin_dialogue(device_handler) as debug:
        _input_pin(debug, PIN24)
        _see_pin(debug)
        _confirm_pin(debug)


@pytest.mark.setup_client(pin=PIN24)
def test_pin_long_delete(device_handler: "BackgroundDeviceHandler"):
    with prepare_pin_dialogue(device_handler) as debug:
        _input_pin(debug, PIN24)
        _see_pin(debug)

        _delete_pin(debug, 10)
        _see_pin(debug)

        _input_pin(debug, PIN24[-10:])
        _see_pin(debug)
        _confirm_pin(debug)


@pytest.mark.setup_client(pin=PIN60[:50])
def test_pin_longer_than_max(device_handler: "BackgroundDeviceHandler"):
    with prepare_pin_dialogue(device_handler) as debug:
        _input_pin(debug, PIN60)

        # What is over 50 digits was not entered
        # TODO: do some UI change when limit is reached?
        layout = debug.read_layout()
        assert PIN60[:50] in layout.str_content
        assert not PIN60[-10:] in layout.str_content

        _see_pin(debug)
        _confirm_pin(debug)
