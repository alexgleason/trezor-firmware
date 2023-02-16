#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_Regular_14_HEIGHT 14
#define Font_TTHoves_Regular_14_MAX_HEIGHT 15
#define Font_TTHoves_Regular_14_BASELINE 3
extern const uint8_t* const Font_TTHoves_Regular_14[126 + 1 - 32];
extern const uint8_t Font_TTHoves_Regular_14_glyph_nonprintable[];
