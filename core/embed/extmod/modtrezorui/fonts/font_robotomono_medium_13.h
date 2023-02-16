#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_13_HEIGHT 13
#define Font_RobotoMono_Medium_13_MAX_HEIGHT 14
#define Font_RobotoMono_Medium_13_BASELINE 3
extern const uint8_t* const Font_RobotoMono_Medium_13[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_13_glyph_nonprintable[];
