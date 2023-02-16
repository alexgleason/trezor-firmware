#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_10_HEIGHT 10
#define Font_RobotoMono_Medium_10_MAX_HEIGHT 13
#define Font_RobotoMono_Medium_10_BASELINE 3
extern const uint8_t* const Font_RobotoMono_Medium_10[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_10_glyph_nonprintable[];
