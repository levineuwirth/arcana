"""Arcana Engine — Python bindings.

Build the native module with `maturin develop` from the
`arcana-py/` directory before importing. See `arcana-py/README.md`
for the full setup.

Currently exported (v0):
    arcana.MtgEnv                     — Gymnasium-shaped env wrapper.
                                        reset() works; step() raises
                                        NotImplementedError until
                                        legal-action enumeration lands.
    arcana.BASIC_E2_DIM_TWO_PLAYERS   — observation-vector length for
                                        the default 2-player setup.
    arcana.__version__                — package version string.
"""

from .arcana_py import (
    BASIC_E2_DIM_TWO_PLAYERS,
    MtgEnv,
    __version__,
)

__all__ = [
    "BASIC_E2_DIM_TWO_PLAYERS",
    "MtgEnv",
    "__version__",
]
