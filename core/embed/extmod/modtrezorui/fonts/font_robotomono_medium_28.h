#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_28_HEIGHT 28
#define Font_RobotoMono_Medium_28_MAX_HEIGHT 30
#define Font_RobotoMono_Medium_28_BASELINE 7
extern const uint8_t* const Font_RobotoMono_Medium_28[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_28_glyph_nonprintable[];
