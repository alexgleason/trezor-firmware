#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_Regular_12_HEIGHT 12
#define Font_TTHoves_Regular_12_MAX_HEIGHT 13
#define Font_TTHoves_Regular_12_BASELINE 3
extern const uint8_t* const Font_TTHoves_Regular_12[126 + 1 - 32];
extern const uint8_t Font_TTHoves_Regular_12_glyph_nonprintable[];
