#!/usr/bin/env python3
"""Synthetic regression checks for a2_h2h_summary.py; no real game rows.

    python3 -I arcana-ai/scripts/test_a2_h2h_summary.py

Every invalid input must exit 2 and print no outcome (no point-rate, no
win/draw/loss count, no decision line); --check-only never prints one; a
valid input must report the seed-block interval and the paired differences
as documented, and the registered flags must apply their thresholds to
unrounded values.
"""

import contextlib
import fractions
import importlib.util
import io
import json
import math
import os
import statistics
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("summary", os.path.join(HERE, "a2_h2h_summary.py"))
summary = importlib.util.module_from_spec(spec)
spec.loader.exec_module(summary)

DECKS = summary.EXPECTED_DECKS
FIRST, N = 1000, 4


def rows(budget="12/120", outcome=lambda deck_i, g: "a", first=FIRST, n=N, decks=DECKS):
    return [f"timing,{d},vmc-material vs pimc-{budget},{g},{outcome(i, g)},1.50"
            for i, d in enumerate(decks) for g in range(first, first + n)]


class Summary(unittest.TestCase):
    def run_main(self, lines, budgets=("12/120",), header=True, extra=(), n=N):
        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, "rows.csv")
            with open(path, "w", encoding="utf-8") as f:
                if header:
                    f.write("timing,deck,pairing,game,outcome,seconds\n")
                f.write("".join(l if l.endswith("\n") or l == "" else l + "\n" for l in lines))
            out = io.StringIO()
            argv = ["--first", str(FIRST), "--games-per-deck", str(n)]
            for b in budgets:
                argv += ["--budget", b]
            with contextlib.redirect_stdout(out):
                code = summary.main(argv + list(extra) + [path])
            return code, out.getvalue()

    def assert_invalid(self, lines, budgets=("12/120",), header=True):
        code, text = self.run_main(lines, budgets, header)
        self.assertEqual(code, 2, text)
        self.assertIn("INVALID", text)
        self.assertNotIn("point-rate", text)
        self.assertNotIn("w/d/l", text)
        self.assertNotIn("DECISIONS", text)
        return text

    def test_valid_input_reports_every_game(self):
        code, text = self.run_main(rows())
        self.assertEqual(code, 0, text)
        self.assertIn("ALL (seed blocks)", text)
        self.assertIn(f"n={len(DECKS) * N}", text)
        self.assertIn("point-rate 1.000", text)

    def test_one_deck_only_is_refused(self):
        self.assert_invalid(rows(decks=DECKS[:1]))

    def test_a_missing_game_is_refused(self):
        self.assert_invalid(rows()[:-1])

    def test_a_duplicate_game_is_refused(self):
        r = rows()
        self.assert_invalid(r + [r[0]])

    def test_games_outside_the_range_are_refused(self):
        self.assert_invalid(rows() + rows(first=FIRST - 1, n=1, decks=DECKS[:1]))
        self.assert_invalid(rows() + rows(first=FIRST + N, n=1, decks=DECKS[:1]))

    def test_header_only_is_refused(self):
        self.assert_invalid([])

    def test_a_malformed_header_is_refused(self):
        self.assert_invalid(rows() + ["timing,deck,truncated"])

    def test_check_only_prints_no_outcome(self):
        code, text = self.run_main(rows(), extra=["--check-only"])
        self.assertEqual(code, 0, text)
        self.assertIn("VALID", text)
        for word in ("point-rate", "w/d/l", "DECISIONS"):
            self.assertNotIn(word, text)

    def test_the_wrong_budget_is_refused(self):
        self.assert_invalid(rows(budget="16/160"))

    def test_a_missing_budget_is_refused(self):
        self.assert_invalid(rows(), budgets=("12/120", "16/160"))

    def test_an_unknown_deck_is_refused(self):
        r = rows()
        self.assert_invalid(r + [r[0].replace(DECKS[0], "Mono Green [1_1]")])

    def test_malformed_rows_are_refused(self):
        r = rows()
        for bad in (
            "timing,UW,only,four",
            r[0].replace(",a,", ",win,"),
            r[0].replace(f",{FIRST},", ",x,"),
            r[0].replace(",1.50", ",nan"),
            r[0][: len(r[0]) // 2],
        ):
            text = self.assert_invalid(r[1:] + [bad])
            self.assertNotIn(",a,", text)

    def test_the_interval_comes_from_seed_blocks(self):
        # Block means over the five decks: 1.0, 0.6, 0.2, 0.2 for games 1000..1003.
        wins = {1000: 5, 1001: 3, 1002: 1, 1003: 1}
        code, text = self.run_main(rows(outcome=lambda i, g: "a" if i < wins[g] else "b"))
        self.assertEqual(code, 0, text)
        blocks = [1.0, 0.6, 0.2, 0.2]
        m = statistics.mean(blocks)
        half = 1.96 * statistics.stdev(blocks) / math.sqrt(len(blocks))
        self.assertIn(f"ALL (seed blocks)                point-rate {m:.3f} [{m - half:.3f}, {m + half:.3f}]", text)

    def test_budgets_are_compared_within_seed_blocks(self):
        # Block means at 12/120: 1.0, 0.6, 0.2, 0.2; at 32/320 each 0.2 lower.
        # Paired by block the difference is exactly 0.2 with no spread; taken
        # apart from the blocks, or with the blocks misaligned, it spreads.
        hi = {1000: 5, 1001: 3, 1002: 1, 1003: 1}
        lo = {1000: 4, 1001: 2, 1002: 0, 1003: 0}
        lines = (rows(budget="12/120", outcome=lambda i, g: "a" if i < hi[g] else "b")
                 + rows(budget="32/320", outcome=lambda i, g: "a" if i < lo[g] else "b"))
        code, text = self.run_main(lines, budgets=("12/120", "32/320"))
        self.assertEqual(code, 0, text)
        self.assertIn("point-rate at 12/120 minus at 32/320: +0.200 [+0.200, +0.200]", text)
        figures = json.loads(text.split("DECISIONS ", 1)[1])["figures"]
        d = figures["differences"]["12/120 - 32/320"]
        self.assertEqual(d["mean_exact"], "1/5")
        self.assertAlmostEqual(d["hi"] - d["lo"], 0.0)

    def test_an_exact_drop_of_0_05_meets_h2b(self):
        # 216 games a deck. 32/320 loses deck 0's games in the first 54 seed
        # blocks, so the drop is 54 of 1,080 games, exactly 0.05. Averaging
        # the 54 float block differences (1.0 - 0.8) lands just under 0.05.
        n = 216
        lines = (rows(budget="12/120", n=n) + rows(budget="16/160", n=n)
                 + rows(budget="32/320", n=n, outcome=lambda i, g: "b" if i == 0 and g - FIRST < 54 else "a"))
        code, text = self.run_main(lines, budgets=("12/120", "16/160", "32/320"), n=n)
        self.assertEqual(code, 0, text)
        out = json.loads(text.split("DECISIONS ", 1)[1])
        self.assertEqual(out["figures"]["differences"]["12/120 - 32/320"]["mean_exact"], "1/20")
        self.assertTrue(out["flags"]["H2b"])
        self.assertLess(sum([1.0 - 0.8] * 54) / n, 0.05)  # the float path this guards against


def st(rates, his=None, los=None, diffs=None):
    """A minimal statistics record for the decision rules; rates and
    differences given as decimals become exact fractions."""
    his = his or {}
    los = los or {}
    F = lambda v: fractions.Fraction(str(v))
    return {
        "budgets": {b: {"rate_exact": F(r), "rate": r, "lo": los.get(b, r - 0.03), "hi": his.get(b, r + 0.03)}
                    for b, r in rates.items()},
        "differences": {k: {"mean_exact": F(v), "mean": v} for k, v in (diffs or {}).items()},
    }


class Decisions(unittest.TestCase):
    B = ["12/120", "16/160", "32/320"]

    def flags(self, rates, **kw):
        F = lambda v: fractions.Fraction(str(v))
        diffs = kw.pop("diffs", None) or {
            "12/120 - 16/160": str(F(rates["12/120"]) - F(rates["16/160"])),
            "12/120 - 32/320": str(F(rates["12/120"]) - F(rates["32/320"])),
            "16/160 - 32/320": str(F(rates["16/160"]) - F(rates["32/320"])),
        }
        return summary.decisions(st(rates, diffs=diffs, **kw), list(rates))

    def test_the_64_640_condition_uses_the_unrounded_difference(self):
        rates = {"12/120": 0.56, "16/160": 0.52, "32/320": 0.49}
        diffs = {"12/120 - 16/160": 0.04, "12/120 - 32/320": 0.07, "16/160 - 32/320": 0.0296296296}
        self.assertFalse(self.flags(rates, diffs=diffs)["run_64_640"])  # prints as +0.030
        diffs["16/160 - 32/320"] = 0.03
        self.assertTrue(self.flags(rates, diffs=diffs)["run_64_640"])

    def test_the_64_640_condition_needs_the_interval_to_hold_half(self):
        rates = {"12/120": 0.6, "16/160": 0.55, "32/320": 0.47}
        self.assertFalse(self.flags(rates, his={"32/320": 0.4999})["run_64_640"])
        self.assertTrue(self.flags(rates, his={"32/320": 0.5})["run_64_640"])

    def test_d1_takes_the_cheapest_budget_wholly_below_half(self):
        rates = {"12/120": 0.48, "16/160": 0.44, "32/320": 0.40}
        f = self.flags(rates, his={"12/120": 0.5, "16/160": 0.499, "32/320": 0.43})
        self.assertEqual(f["D1_referee"], "16/160")
        self.assertEqual(f["D2_teacher"], "16/160")

    def test_d2_breaks_a_three_decimal_tie_toward_the_cheaper_budget(self):
        rates = {"12/120": 0.6, "16/160": 0.4904, "32/320": 0.4896}
        f = self.flags(rates, his={"16/160": 0.52, "32/320": 0.52})
        self.assertEqual(f["D2_best_pimc"], "16/160")
        self.assertIsNone(f["D1_referee"])
        self.assertEqual(f["D2_teacher"], "16/160")
        self.assertEqual(f["D2_bar"], "16/160")

    def test_d2_bar_compares_the_unrounded_rate_with_half(self):
        self.assertEqual(self.flags({"12/120": 0.6, "16/160": 0.55, "32/320": 0.5})["D2_bar"], "vmc-material")
        self.assertEqual(self.flags({"12/120": 0.6, "16/160": 0.55, "32/320": 0.49999})["D2_bar"], "32/320")

    def test_h2a_is_strict(self):
        self.assertFalse(self.flags({"12/120": 0.6, "16/160": 0.55, "32/320": 0.55})["H2a"])
        self.assertTrue(self.flags({"12/120": 0.6, "16/160": 0.55, "32/320": 0.5499})["H2a"])


if __name__ == "__main__":
    unittest.main()
