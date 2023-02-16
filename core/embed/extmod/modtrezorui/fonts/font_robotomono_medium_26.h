#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_26_HEIGHT 26
#define Font_RobotoMono_Medium_26_MAX_HEIGHT 28
#define Font_RobotoMono_Medium_26_BASELINE 6
extern const uint8_t* const Font_RobotoMono_Medium_26[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_26_glyph_nonprintable[];
