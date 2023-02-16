#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_DemiBold_28_HEIGHT 28
#define Font_TTHoves_DemiBold_28_MAX_HEIGHT 29
#define Font_TTHoves_DemiBold_28_BASELINE 6
extern const uint8_t* const Font_TTHoves_DemiBold_28[126 + 1 - 32];
extern const uint8_t Font_TTHoves_DemiBold_28_glyph_nonprintable[];
