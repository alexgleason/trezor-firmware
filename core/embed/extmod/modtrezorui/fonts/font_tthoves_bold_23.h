#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_Bold_23_HEIGHT 23
#define Font_TTHoves_Bold_23_MAX_HEIGHT 24
#define Font_TTHoves_Bold_23_BASELINE 5
extern const uint8_t* const Font_TTHoves_Bold_23[126 + 1 - 32];
extern const uint8_t Font_TTHoves_Bold_23_glyph_nonprintable[];
