from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from trezor.wire import GenericContext
    from trezor.messages import WipeDevice, Success


async def wipe_device(ctx: GenericContext, msg: WipeDevice) -> Success:
    import storage
    from trezor.enums import ButtonRequestType
    from trezor.messages import Success

    from apps.base import reload_settings_from_storage

    # Skip UI option is only available in debug builds (to speed up tests)
    if msg.skip_ui:
        if not __debug__:
            from trezor.wire import ProcessError

            raise ProcessError("skip_ui option is only for debug builds")
    else:
        from trezor.ui.layouts import confirm_action

        await confirm_action(
            ctx,
            "confirm_wipe",
            "Wipe device",
            "All data will be erased.",
            "Do you really want to wipe the device?\n",
            reverse=True,
            verb="Hold to confirm",
            hold=True,
            hold_danger=True,
            br_code=ButtonRequestType.WipeDevice,
        )

    storage.wipe()
    reload_settings_from_storage()

    return Success(message="Device wiped")
