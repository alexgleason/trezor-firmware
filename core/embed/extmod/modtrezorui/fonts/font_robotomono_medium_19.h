#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_19_HEIGHT 19
#define Font_RobotoMono_Medium_19_MAX_HEIGHT 21
#define Font_RobotoMono_Medium_19_BASELINE 5
extern const uint8_t* const Font_RobotoMono_Medium_19[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_19_glyph_nonprintable[];
