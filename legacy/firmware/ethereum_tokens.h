// This file is automatically generated from ethereum_tokens.h.mako
// DO NOT EDIT

#ifndef __ETHEREUM_TOKENS_H__
#define __ETHEREUM_TOKENS_H__

#include <stdint.h>
#include "ethereum_definitions.h"

extern const EthereumTokenInfo UNKNOWN_TOKEN;

const EthereumTokenInfo *ethereum_token_by_address(uint64_t chain_id,
                                                   const uint8_t *address);
bool is_unknown_token(const EthereumTokenInfo *token);

#endif
