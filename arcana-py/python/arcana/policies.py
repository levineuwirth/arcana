"""Built-in self-play policies.

These are Rust-side implementations exposed as Python objects.
**Prefer these over Python callables for high-throughput training**:
they run inside the Rust harness without crossing the FFI boundary
at every decision, and a typical 200-turn game has 600-1000
decisions.

Python callables (``def my_policy(obs, n_legal): ...``) are still
supported by `arcana.run_episode` for development, debugging, and
evaluation against custom heuristics — see the module-level
documentation in `arcana-py/src/episode.rs` for the GIL/throughput
tradeoff.
"""

from .arcana_py import (
    _make_first_action_policy,
    _make_progress_biased_policy,
    _make_random_policy,
)

__all__ = ["random", "progress_biased", "first_action"]


def random(seed: int = 0):
    """Uniform-random over `legal_actions`.

    NOTE: pure uniform may not terminate in pathological cases —
    uninstructed agents can mulligan to oblivion or spin without
    making progress. Use `progress_biased(seed)` for integration
    tests that need reliable termination.
    """
    return _make_random_policy(seed)


def progress_biased(seed: int = 0):
    """Random with the engine integration-test bias: take MulliganKeep
    when offered, prefer non-pass non-concede actions, fall back to
    pass.

    Mirrors the policy used by
    ``arcana-core/tests/basic_game.rs::pick_action`` — known to
    drive Lightning Bolt + vanilla games to completion.
    """
    return _make_progress_biased_policy(seed)


def first_action():
    """Always pick action index 0. Deterministic baseline for tests
    where reproducible-trajectory shape matters more than realistic
    play."""
    return _make_first_action_policy()
