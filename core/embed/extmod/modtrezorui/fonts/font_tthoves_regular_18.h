#include <stdint.h>

#if TREZOR_FONT_BPP != 4
  #error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_TTHoves_Regular_18_HEIGHT 18
#define Font_TTHoves_Regular_18_MAX_HEIGHT 19
#define Font_TTHoves_Regular_18_BASELINE 4
extern const uint8_t* const Font_TTHoves_Regular_18[126 + 1 - 32];
extern const uint8_t Font_TTHoves_Regular_18_glyph_nonprintable[];
