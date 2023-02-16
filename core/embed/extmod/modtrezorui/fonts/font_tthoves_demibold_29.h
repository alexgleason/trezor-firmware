#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_DemiBold_29_HEIGHT 29
#define Font_TTHoves_DemiBold_29_MAX_HEIGHT 30
#define Font_TTHoves_DemiBold_29_BASELINE 6
extern const uint8_t* const Font_TTHoves_DemiBold_29[126 + 1 - 32];
extern const uint8_t Font_TTHoves_DemiBold_29_glyph_nonprintable[];
