#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_25_HEIGHT 25
#define Font_RobotoMono_Medium_25_MAX_HEIGHT 28
#define Font_RobotoMono_Medium_25_BASELINE 6
extern const uint8_t* const Font_RobotoMono_Medium_25[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_25_glyph_nonprintable[];
