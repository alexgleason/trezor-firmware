#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_DemiBold_26_HEIGHT 26
#define Font_TTHoves_DemiBold_26_MAX_HEIGHT 26
#define Font_TTHoves_DemiBold_26_BASELINE 5
extern const uint8_t* const Font_TTHoves_DemiBold_26[126 + 1 - 32];
extern const uint8_t Font_TTHoves_DemiBold_26_glyph_nonprintable[];
