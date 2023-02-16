#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_Bold_27_HEIGHT 27
#define Font_TTHoves_Bold_27_MAX_HEIGHT 28
#define Font_TTHoves_Bold_27_BASELINE 6
extern const uint8_t* const Font_TTHoves_Bold_27[126 + 1 - 32];
extern const uint8_t Font_TTHoves_Bold_27_glyph_nonprintable[];
