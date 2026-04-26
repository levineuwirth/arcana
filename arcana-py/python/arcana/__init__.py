"""Arcana Engine — Python bindings.

Build the native module with `maturin develop` from the
`arcana-py/` directory before importing. See `arcana-py/README.md`
for the full setup.

Currently exported (v0):
    arcana.MtgEnv                     — Gymnasium-shaped env wrapper
                                        (reset() works; step() raises
                                        NotImplementedError until the
                                        single-perspective auto-advance
                                        binding lands).
    arcana.run_episode                — stateless self-play episode
                                        driver. Drives the real engine
                                        end-to-end and returns
                                        per-perspective trajectories
                                        as numpy struct-of-arrays.
    arcana.policies                   — built-in policy factories
                                        (random, progress_biased,
                                        first_action).
    arcana.SEED_DECK_SETUP            — description of the Lightning
                                        Bolt seed-deck setup that
                                        run_episode constructs.
    arcana.BASIC_E2_DIM_TWO_PLAYERS   — observation-vector length for
                                        the default 2-player setup.
    arcana.__version__                — package version string.
"""

from dataclasses import dataclass, field

from . import policies
from .arcana_py import (
    BASIC_E2_DIM_TWO_PLAYERS,
    MtgEnv,
    PyEpisodeOutcome as EpisodeOutcome,
    PyEpisodeResult as EpisodeResult,
    PyPolicy as Policy,
    PyTrajectory as Trajectory,
    __version__,
    run_episode,
)


@dataclass(frozen=True)
class _SeedDeckSetup:
    """Description of the seed-deck setup used by `run_episode`.

    Pinned constant in v0; Phase 2 may reshape this when the
    `Deck` / `CardRegistry` types are exposed to Python and
    `run_episode` accepts arbitrary decks.
    """

    player_0_deck: list = field(
        default_factory=lambda: [
            ("Mountain", 10),
            ("Lightning Bolt", 4),
            ("Grizzly Bears", 4),
        ]
    )
    player_1_deck: list = field(
        default_factory=lambda: [
            ("Forest", 10),
            ("Grizzly Bears", 8),
        ]
    )
    card_pool: frozenset = field(
        default_factory=lambda: frozenset(
            {"Mountain", "Forest", "Lightning Bolt", "Grizzly Bears"}
        )
    )
    description: str = (
        "Lightning Bolt milestone (matches arcana-core/tests/basic_game.rs)"
    )


SEED_DECK_SETUP = _SeedDeckSetup()


__all__ = [
    "BASIC_E2_DIM_TWO_PLAYERS",
    "EpisodeOutcome",
    "EpisodeResult",
    "MtgEnv",
    "Policy",
    "SEED_DECK_SETUP",
    "Trajectory",
    "__version__",
    "policies",
    "run_episode",
]
