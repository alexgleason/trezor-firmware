#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_11_HEIGHT 11
#define Font_RobotoMono_Medium_11_MAX_HEIGHT 13
#define Font_RobotoMono_Medium_11_BASELINE 3
extern const uint8_t* const Font_RobotoMono_Medium_11[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_11_glyph_nonprintable[];
