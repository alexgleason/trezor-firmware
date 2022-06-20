"""
Common things for all the row handlers.
"""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from .api import RowHandlerAPI

if TYPE_CHECKING:  # pragma: no cover
    from .api import DataRow, SourceDefinitionCacheAPI


HERE = Path(__file__).resolve().parent
CORE_DIR = HERE.parent.parent.parent.parent


INVALID_FILE_PREFIX = "--invalid_file--"

# Defining all the prefixes here at one place,
# so we see they are not clashing with each other
MPY_PREFIXES = ("fun_data_", "const_table_data_", "const_obj_", "raw_code_")
RUST_PREFIXES = (
    "trezor_lib",
    "compiler_builtins",
    "core::",
    "_$LT$",
    "heapless::",
    "cstr_core::",
)


class CommonRow(RowHandlerAPI):
    """Common functionality for all the handlers.

    Provides a sensible implementation of the `RowHandlerAPI` that can be reused -
    _get_module_and_function() and _get_definition() are the only methods
    that need to be defined on the specific row handlers.
    """

    language = "Common language"

    def __init__(
        self, source_def_cache: SourceDefinitionCacheAPI | None = None
    ) -> None:
        self.source_def_cache = source_def_cache

    def add_basic_info(self, row: DataRow) -> DataRow:
        module_name, func_name = self._get_module_and_function(row.symbol_name)

        if row.symbol_name.startswith("[section"):
            row.language = ""
        else:
            row.language = self.language
        row.module_name = module_name
        row.func_name = func_name
        return row

    def add_definition(self, row: DataRow) -> DataRow:
        # In case row is missing a basic info, add it first
        if not row.language:
            row = self.add_basic_info(row)

        # Taking the source definition from the build definition, if
        # it is already there and is a valid source - not coming from build
        # (applicable for most of the C files)
        if (
            row.build_definition
            and "build/firmware/frozen_mpy.c" not in row.build_definition
        ):
            row.source_definition = row.build_definition
        else:
            row.source_definition = self._get_definition_cached(row)

        return row

    def _get_definition_cached(self, row: DataRow) -> str:
        if self.source_def_cache is not None:
            # If not in cache or invalidated, computing it and adding it to the cache
            cached = self.source_def_cache.get(row.symbol_name)
            if cached is None or self.source_def_cache.is_invalidated(row.symbol_name):
                result = self._get_definition(row)
                self.source_def_cache.add(row.symbol_name, result)
                return result
            else:
                return cached
        else:
            return self._get_definition(row)

    def _get_module_and_function(self, symbol_name: str) -> tuple[str, str]:
        ...

    def _get_definition(self, row: DataRow) -> str:
        ...