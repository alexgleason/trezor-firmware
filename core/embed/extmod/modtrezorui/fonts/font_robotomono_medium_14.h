#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_14_HEIGHT 14
#define Font_RobotoMono_Medium_14_MAX_HEIGHT 17
#define Font_RobotoMono_Medium_14_BASELINE 4
extern const uint8_t* const Font_RobotoMono_Medium_14[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_14_glyph_nonprintable[];
