#include <stdint.h>

#if TREZOR_FONT_BPP != 4
#error Wrong TREZOR_FONT_BPP (expected 4)
#endif
#define Font_RobotoMono_Medium_27_HEIGHT 27
#define Font_RobotoMono_Medium_27_MAX_HEIGHT 30
#define Font_RobotoMono_Medium_27_BASELINE 7
extern const uint8_t* const Font_RobotoMono_Medium_27[126 + 1 - 32];
extern const uint8_t Font_RobotoMono_Medium_27_glyph_nonprintable[];
