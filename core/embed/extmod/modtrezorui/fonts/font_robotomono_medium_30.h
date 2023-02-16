#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_30_HEIGHT 30
#define Font_RobotoMono_Medium_30_MAX_HEIGHT 32
#define Font_RobotoMono_Medium_30_BASELINE 7
extern const uint8_t* const Font_RobotoMono_Medium_30[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_30_glyph_nonprintable[];
