#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_12_HEIGHT 12
#define Font_RobotoMono_Medium_12_MAX_HEIGHT 14
#define Font_RobotoMono_Medium_12_BASELINE 3
extern const uint8_t* const Font_RobotoMono_Medium_12[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_12_glyph_nonprintable[];
