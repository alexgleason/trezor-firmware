#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_17_HEIGHT 17
#define Font_RobotoMono_Medium_17_MAX_HEIGHT 18
#define Font_RobotoMono_Medium_17_BASELINE 4
extern const uint8_t* const Font_RobotoMono_Medium_17[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_17_glyph_nonprintable[];
