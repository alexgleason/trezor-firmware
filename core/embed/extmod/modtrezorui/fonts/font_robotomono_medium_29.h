#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_29_HEIGHT 29
#define Font_RobotoMono_Medium_29_MAX_HEIGHT 32
#define Font_RobotoMono_Medium_29_BASELINE 7
extern const uint8_t* const Font_RobotoMono_Medium_29[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_29_glyph_nonprintable[];
