#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_Regular_10_HEIGHT 10
#define Font_TTHoves_Regular_10_MAX_HEIGHT 10
#define Font_TTHoves_Regular_10_BASELINE 2
extern const uint8_t* const Font_TTHoves_Regular_10[126 + 1 - 32];
extern const uint8_t Font_TTHoves_Regular_10_glyph_nonprintable[];
