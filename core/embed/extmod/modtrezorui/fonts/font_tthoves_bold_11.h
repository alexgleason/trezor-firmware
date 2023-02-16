#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_Bold_11_HEIGHT 11
#define Font_TTHoves_Bold_11_MAX_HEIGHT 12
#define Font_TTHoves_Bold_11_BASELINE 3
extern const uint8_t* const Font_TTHoves_Bold_11[126 + 1 - 32];
extern const uint8_t Font_TTHoves_Bold_11_glyph_nonprintable[];
