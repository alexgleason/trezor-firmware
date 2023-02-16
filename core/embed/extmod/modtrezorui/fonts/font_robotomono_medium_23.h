#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_23_HEIGHT 23
#define Font_RobotoMono_Medium_23_MAX_HEIGHT 26
#define Font_RobotoMono_Medium_23_BASELINE 6
extern const uint8_t* const Font_RobotoMono_Medium_23[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_23_glyph_nonprintable[];
