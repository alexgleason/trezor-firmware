#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_DemiBold_19_HEIGHT 19
#define Font_TTHoves_DemiBold_19_MAX_HEIGHT 20
#define Font_TTHoves_DemiBold_19_BASELINE 4
extern const uint8_t* const Font_TTHoves_DemiBold_19[126 + 1 - 32];
extern const uint8_t Font_TTHoves_DemiBold_19_glyph_nonprintable[];
