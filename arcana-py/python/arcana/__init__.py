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

import inspect
import warnings
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
)
from .arcana_py import run_episode as _rust_run_episode


def run_episode(
    seed: int = 0,
    policy_a=None,
    policy_b=None,
    max_turns: int = 200,
    max_steps: int = 5000,
):
    """Drive one self-play episode end-to-end.

    Both policy slots accept either an `arcana.Policy` (built-in
    constructed via `arcana.policies.*`) or a plain Python callable
    `(obs: np.ndarray, n_legal: int) -> int`. Passing `None`
    defaults to `policies.random()` seeded from the episode seed.

    `seed` deterministically determines the entire episode trace —
    re-running with the same seed produces byte-identical
    trajectories.

    The GIL is held for the duration of this call. Other Python
    threads cannot run during episode execution; this matters only
    for parallel-episode workloads, which should use process-level
    parallelism (multiprocessing.Pool) on top of single-episode
    primitives until in-Rust vectorization lands.
    """
    _validate_callable_policy(policy_a)
    _validate_callable_policy(policy_b)
    return _rust_run_episode(
        seed=seed,
        policy_a=policy_a,
        policy_b=policy_b,
        max_turns=max_turns,
        max_steps=max_steps,
    )


def _validate_callable_policy(p) -> None:
    """Warn if `p` is a Python callable that doesn't accept **kwargs.

    Forward-compat guardrail: future versions may pass extra keyword
    arguments to policies (e.g. `kinds=` for action introspection).
    Catching the breakage at policy construction with a clear
    FutureWarning is cheaper than the silent breakage at v1 release.
    """
    if p is None or not callable(p):
        # None or a PyPolicy handle (PyPolicy isn't callable from Python).
        return
    try:
        sig = inspect.signature(p)
    except (ValueError, TypeError):
        # C builtins and some objects don't have introspectable signatures.
        # Skip rather than guess.
        return
    has_kwargs = any(
        param.kind == inspect.Parameter.VAR_KEYWORD
        for param in sig.parameters.values()
    )
    if not has_kwargs:
        warnings.warn(
            "Python policy callable should accept **kwargs for forward "
            "compatibility. Future versions may pass additional keyword "
            "arguments (e.g. kinds=). Recommended signature: "
            "(obs, n_legal, **kwargs) -> int",
            FutureWarning,
            stacklevel=3,
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
