#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_18_HEIGHT 18
#define Font_RobotoMono_Medium_18_MAX_HEIGHT 20
#define Font_RobotoMono_Medium_18_BASELINE 5
extern const uint8_t* const Font_RobotoMono_Medium_18[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_18_glyph_nonprintable[];
