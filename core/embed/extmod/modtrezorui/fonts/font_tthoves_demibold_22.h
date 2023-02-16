#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_DemiBold_22_HEIGHT 22
#define Font_TTHoves_DemiBold_22_MAX_HEIGHT 23
#define Font_TTHoves_DemiBold_22_BASELINE 5
extern const uint8_t* const Font_TTHoves_DemiBold_22[126 + 1 - 32];
extern const uint8_t Font_TTHoves_DemiBold_22_glyph_nonprintable[];
