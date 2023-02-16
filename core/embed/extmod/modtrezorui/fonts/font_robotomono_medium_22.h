#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_22_HEIGHT 22
#define Font_RobotoMono_Medium_22_MAX_HEIGHT 24
#define Font_RobotoMono_Medium_22_BASELINE 5
extern const uint8_t* const Font_RobotoMono_Medium_22[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_22_glyph_nonprintable[];
