from micropython import const
from typing import TYPE_CHECKING

import storage.device as storage_device
from trezor.wire import DataError

if TYPE_CHECKING:
    from trezor.wire import Context


_MAX_PASSPHRASE_LEN = const(50)


def is_enabled() -> bool:
    return storage_device.is_passphrase_enabled()


async def get(ctx: Context) -> str:
    from trezor import workflow

    if not is_enabled():
        return ""
    else:
        workflow.close_others()  # request exclusive UI access
        if storage_device.get_passphrase_always_on_device():
            from trezor.ui.layouts import request_passphrase_on_device

            passphrase = await request_passphrase_on_device(ctx, _MAX_PASSPHRASE_LEN)
        else:
            passphrase = await _request_on_host(ctx)
        if len(passphrase.encode()) > _MAX_PASSPHRASE_LEN:
            raise DataError(f"Maximum passphrase length is {_MAX_PASSPHRASE_LEN} bytes")

        return passphrase


async def _request_on_host(ctx: Context) -> str:
    from trezor.messages import PassphraseAck, PassphraseRequest
    from trezor.ui.layouts import request_passphrase_on_host

    request_passphrase_on_host()

    request = PassphraseRequest()
    ack = await ctx.call(request, PassphraseAck)
    passphrase = ack.passphrase  # local_cache_attribute

    if ack.on_device:
        from trezor.ui.layouts import request_passphrase_on_device

        if passphrase is not None:
            raise DataError("Passphrase provided when it should not be")
        return await request_passphrase_on_device(ctx, _MAX_PASSPHRASE_LEN)

    if passphrase is None:
        raise DataError(
            "Passphrase not provided and on_device is False. Use empty string to set an empty passphrase."
        )

    # non-empty passphrase
    if passphrase:
        from trezor.ui.layouts import confirm_action, confirm_blob
        from trezor import utils

        description = "Access hidden wallet?\n\nNext screen will show the passphrase!"

        # Getting rid of newlines for TR, to fit one smaller screen:
        if utils.MODEL in ("R",):
            description = description.replace("\n\n", " ")

        await confirm_action(
            ctx,
            "passphrase_host1",
            "Hidden wallet",
            description=description,
        )

        await confirm_blob(
            ctx,
            "passphrase_host2",
            "Hidden wallet",
            passphrase,
            "Use this passphrase?\n",
        )

    return passphrase
