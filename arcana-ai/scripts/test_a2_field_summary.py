#!/usr/bin/env python3
"""Synthetic regression checks for a2_field_summary.py; no real game rows.

    python3 -I arcana-ai/scripts/test_a2_field_summary.py

A six-deck manifest keeps every fixture small: 15 pairs, ten games a deck.
Every invalid input must exit 2 and print no outcome (no rate, no rho, no
s_X, no DECISIONS line); --check-only and --sizing never print one; a valid
input must apply the registered rules to unrounded or exact values.
"""

import contextlib
import fractions
import importlib.util
import io
import json
import math
import os
import random
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("field", os.path.join(HERE, "a2_field_summary.py"))
field = importlib.util.module_from_spec(spec)
spec.loader.exec_module(field)

N = 6
NAMES = [f"Deck {k} [{1000 + k}_{2000 + k}]" for k in range(N)]
PAIRS = field.pairs_of(N)
WIN, LOSS, DRAW = ("1", "0"), ("0", "1"), ("0.5", "0.5")


def strength_duels(strength):
    """Every duel won twice by the deck with the higher `strength`."""
    return {(i, j): (WIN + WIN) if strength[i] > strength[j] else (LOSS + LOSS) for i, j in PAIRS}


def lines(arm, results, shard=(0, 1), seed=None, decks=None, extra=(), names=NAMES):
    kind = field.ARMS[arm][0]
    seed = field.ARMS[arm][3] if seed is None else seed
    out = [field.config_prefix(arm, seed) + f"{shard[0]}/{shard[1]}"]
    out += [f"deck,{k},{name}" for k, name in (decks if decks is not None else enumerate(names))]
    for p, (i, j) in enumerate(field.pairs_of(len(names))):
        if (i, j) in results and p % shard[1] == shard[0]:
            out.append(f"{kind},{i},{j},0,{','.join(results[(i, j)])},1.25")
    return out + list(extra)


def full_field(x_results=None, p_strength=None, r_strength=None):
    f = strength_duels(list(range(N)))
    blocks = x_results or {p: ("1", "1", "0", "0") for p in PAIRS}
    return {
        "F": [lines("F", f)],
        "Fprime": [lines("Fprime", f)],
        "R": [lines("R", strength_duels(r_strength or list(reversed(range(N)))))],
        "P": [lines("P", strength_duels(p_strength or list(range(N))))],
        "X": [lines("X", blocks)],
    }


class Base(unittest.TestCase):
    def run_main(self, arms, extra=(), names=NAMES):
        with tempfile.TemporaryDirectory() as tmp:
            man = os.path.join(tmp, "manifest.csv")
            with open(man, "w", encoding="utf-8") as f:
                f.write("index,name,source,sha256\n")
                for k, name in enumerate(names):
                    f.write(f"{k},{name},{1000 + k}_{2000 + k},{'0' * 64}\n")
            argv = ["--manifest", man]
            for arm, files in arms.items():
                paths = []
                for n, content in enumerate(files):
                    path = os.path.join(tmp, f"{arm}-{n}.csv")
                    with open(path, "w", encoding="utf-8") as f:
                        f.write("\n".join(content) + "\n")
                    paths.append(path)
                argv += [f"--{arm}"] + paths
            out = io.StringIO()
            with contextlib.redirect_stdout(out):
                code = field.main(argv + list(extra))
            return code, out.getvalue()

    def assert_no_outcome(self, text):
        for word in ("rho", "point-rate", "s_X", "DECISIONS", "ranking"):
            self.assertNotIn(word, text)

    def assert_invalid(self, arms, extra=()):
        code, text = self.run_main(arms, extra)
        self.assertEqual(code, 2, text)
        self.assertIn("INVALID", text)
        self.assert_no_outcome(text)
        return text

    def decisions(self, arms):
        code, text = self.run_main(arms)
        self.assertEqual(code, 0, text)
        return json.loads(text.split("DECISIONS ", 1)[1])


class Validation(Base):
    def test_a_valid_field_reads(self):
        out = self.decisions(full_field())
        self.assertIn("flags", out)

    def test_every_arm_is_needed_for_the_reading(self):
        arms = full_field()
        del arms["R"]
        self.assertIn("R: the reading needs every arm", self.assert_invalid(arms))

    def test_a_missing_pair_is_refused(self):
        arms = full_field()
        arms["P"] = [arms["P"][0][:-1]]
        self.assertIn("pairs missing", self.assert_invalid(arms))

    def test_a_duplicate_pair_is_refused(self):
        arms = full_field()
        arms["F"] = [arms["F"][0], arms["F"][0]]
        self.assertIn("more than once", self.assert_invalid(arms))

    def test_shards_merge_and_a_misfiled_row_is_refused(self):
        arms = full_field()
        res = strength_duels(list(range(N)))
        arms["F"] = [lines("F", res, shard=(0, 2)), lines("F", res, shard=(1, 2))]
        self.assertEqual(self.run_main(arms)[0], 0)
        wrong = lines("F", res, shard=(0, 2)) + [f"duel,{PAIRS[1][0]},{PAIRS[1][1]},0,1,0,1,0,1.0"]
        arms["F"] = [wrong, lines("F", {k: v for k, v in res.items() if k != PAIRS[1]}, shard=(1, 2))]
        self.assertIn("is not in shard 0/2", self.assert_invalid(arms))

    def test_the_wrong_config_is_refused(self):
        for arm, bad in (("F", lines("F", strength_duels(list(range(N))), seed=2000)),
                         ("P", [l.replace("pimc_samples: 16, pimc_cap: 160", "pimc_samples: 32, pimc_cap: 320")
                                for l in full_field()["P"][0]]),
                         ("R", [l.replace("config,Random", "config,VmcMaterial") for l in full_field()["R"][0]])):
            arms = full_field()
            arms[arm] = [bad]
            self.assertIn("config row is not the registered one", self.assert_invalid(arms))

    def test_a_file_needs_exactly_one_config_row(self):
        arms = full_field()
        arms["X"] = [arms["X"][0][1:]]
        self.assertIn("0 config rows", self.assert_invalid(arms))
        arms["X"] = [full_field()["X"][0] + [full_field()["X"][0][0]]]
        self.assertIn("2 config rows", self.assert_invalid(arms))

    def test_deck_rows_must_match_the_manifest(self):
        arms = full_field()
        swapped = [(0, NAMES[1]), (1, NAMES[0])] + list(enumerate(NAMES))[2:]
        arms["F"] = [lines("F", strength_duels(list(range(N))), decks=swapped)]
        self.assertIn("do not match the manifest", self.assert_invalid(arms))
        arms["F"] = [lines("F", strength_duels(list(range(N))), decks=list(enumerate(NAMES))[:-1])]
        self.assert_invalid(arms)

    def test_malformed_rows_are_refused(self):
        # Each replaces pair (0, 1)'s valid row, so that if its own check were
        # missing the field would be complete and valid: no other fault (a
        # duplicate, a missing pair) can stand in for the one under test.
        bad_rows = [
            ("duel,0,1,0,1,0,1", "malformed row"),
            ("duel,0,x,0,1,0,1,0,1.0", "not an integer"),
            ("duel,1,0,0,1,0,1,0,1.0", "is not one of the manifest's"),
            ("duel,0,9,0,1,0,1,0,1.0", "is not one of the manifest's"),
            ("duel,0,1,1,1,0,1,0,1.0", "registered one a pair"),
            ("duel,0,1,0,1,1,1,0,1.0", "do not sum to one"),
            ("duel,0,1,0,0.7,0.3,1,0,1.0", "is not 0, 0.5 or 1"),
            ("duel,0,1,0,1,0,1,0,-3", "is not a time"),
            ("duel,0,1,0,1,0,1,0,nan", "is not a time"),
        ]
        valid = full_field()["F"][0]
        target = next(k for k, l in enumerate(valid) if l.startswith("duel,0,1,"))
        for bad, why in bad_rows:
            arms = full_field()
            arms["F"] = [valid[:target] + [bad] + valid[target + 1:]]
            self.assertIn(why, self.assert_invalid(arms), bad)
        for bad, why in (("something,else", "unrecognized line"), ("deck,x,name", "malformed deck row")):
            arms = full_field()
            arms["F"] = [valid + [bad]]
            self.assertIn(why, self.assert_invalid(arms), bad)

    def test_check_only_never_reports_an_outcome(self):
        code, text = self.run_main({"F": full_field()["F"]}, extra=["--check-only"])
        self.assertEqual(code, 0, text)
        self.assertTrue(text.startswith("VALID"))
        self.assert_no_outcome(text)
        self.assert_invalid({"F": [full_field()["F"][0][:-1]]}, extra=["--check-only"])

    def test_sizing_prints_seconds_only_and_wants_the_sizing_seed(self):
        res = {p: v for p, v in strength_duels(list(range(N))).items() if PAIRS.index(p) % 200 == 0}
        sizing = {"F": [lines("F", res, shard=(0, 200), seed=4000)]}
        code, text = self.run_main(sizing, extra=["--sizing"])
        self.assertEqual(code, 0, text)
        self.assertIn("SIZING F: 1 rows", text)
        self.assert_no_outcome(text)
        self.assert_invalid({"F": [lines("F", res, shard=(0, 200))]}, extra=["--sizing"])
        self.assert_invalid({"F": [lines("F", res, shard=(0, 100), seed=4000)]}, extra=["--sizing"])

    def test_sizing_needs_every_selected_pair_once(self):
        # 22 decks, 231 pairs: the sizing shard selects pair indices 0 and 200.
        names = [f"Deck {k} [{k}]" for k in range(22)]
        pairs = field.pairs_of(22)
        both = {pairs[0]: WIN + WIN, pairs[200]: WIN + LOSS}
        run = lambda files: self.run_main({"F": files}, extra=["--sizing"], names=names)
        code, text = run([lines("F", both, shard=(0, 200), seed=4000, names=names)])
        self.assertEqual(code, 0, text)
        self.assertIn("SIZING F: 2 rows", text)
        one = {pairs[0]: WIN + WIN}
        code, text = run([lines("F", one, shard=(0, 200), seed=4000, names=names)])
        self.assertEqual(code, 2, text)
        self.assertIn("1 of 2 pairs missing", text)
        code, text = run([lines("F", one, shard=(0, 200), seed=4000, names=names)] * 2
                         + [lines("F", {pairs[200]: WIN + LOSS}, shard=(0, 200), seed=4000, names=names)])
        self.assertEqual(code, 2, text)
        self.assertIn("more than once", text)
        self.assertNotIn("SIZING", text)


class Statistics(Base):
    def test_average_ranks_make_tied_vectors_correlate_one(self):
        x = [fractions.Fraction(k // 2, 3) for k in range(8)]
        self.assertEqual(field.average_ranks(x)[:2], [1.5, 1.5])
        self.assertAlmostEqual(field.spearman(x, x), 1.0)
        self.assertAlmostEqual(field.spearman(x, [-v for v in x]), -1.0)

    def test_spearman_matches_pearson_on_average_ranks(self):
        rng = random.Random(4)
        for _ in range(50):
            x = [rng.randrange(4) for _ in range(12)]
            y = [rng.randrange(4) for _ in range(12)]
            rx, ry = field.average_ranks(x), field.average_ranks(y)
            mx, my = sum(rx) / 12, sum(ry) / 12
            num = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
            den = math.sqrt(sum((a - mx) ** 2 for a in rx) * sum((b - my) ** 2 for b in ry))
            self.assertAlmostEqual(field.spearman(x, y), num / den if den else 0.0)

    def test_percentile_bounds_take_the_51st_and_1950th(self):
        self.assertEqual(field.percentile_bounds(list(range(2000))), (50, 1949))

    def test_the_jackknife_matches_its_formula(self):
        rng = random.Random(9)
        res = {p: tuple(rng.choice(["0", "0.5", "1"]) for _ in range(4)) for p in PAIRS}
        rows = [(i, j, tuple(field.POINTS[g] for g in res[(i, j)]), 1.0) for i, j in PAIRS]
        x = field.head_to_head(rows, N)
        hp = {p: sum(field.POINTS[g] for g in res[p]) for p in PAIRS}
        loo = [sum(v for p, v in hp.items() if k not in p) / (8 * sum(1 for p in hp if k not in p)) for k in range(N)]
        m = sum(loo) / N
        se = math.sqrt((N - 1) / N * sum((v - m) ** 2 for v in loo))
        s = sum(hp.values()) / (8 * len(PAIRS))
        self.assertAlmostEqual(x["jackknife"][0], s - 1.96 * se)
        self.assertEqual(x["s_X"], fractions.Fraction(sum(hp.values()), 8 * len(PAIRS)))

    def test_the_bootstrap_is_reproducible(self):
        a, b = self.run_main(full_field())[1], self.run_main(full_field())[1]
        self.assertEqual(a, b)


class Decisions(Base):
    def blocks_with_half_points(self, total):
        """X results whose half-points sum to `total` over the 15 blocks."""
        res, left = {}, total
        for p in PAIRS:
            take = min(8, left)
            left -= take
            g = ["0"] * 4
            for k in range(take // 2):
                g[k] = "1"
            if take % 2:
                g[take // 2] = "0.5"
            res[p] = tuple(g)
        assert left == 0
        return res

    def test_x2_is_exact_at_its_boundary(self):
        # 15 blocks, 120 half-points: 17/30 + 1/20 = 37/60 = 74/120.
        at = self.decisions(full_field(x_results=self.blocks_with_half_points(74)))
        self.assertEqual(at["figures"]["X"]["s_X"], "37/60")
        self.assertTrue(at["flags"]["X2"])
        self.assertFalse(self.decisions(full_field(x_results=self.blocks_with_half_points(75)))["flags"]["X2"])
        # 17/30 - 1/20 = 31/60 = 62/120, also inside.
        self.assertTrue(self.decisions(full_field(x_results=self.blocks_with_half_points(62)))["flags"]["X2"])

    def test_x1_needs_the_jackknife_interval(self):
        # Every block 5 of 8 half-points: above one half with no spread.
        flat = {p: ("1", "1", "0.5", "0") for p in PAIRS}
        self.assertTrue(self.decisions(full_field(x_results=flat))["flags"]["X1"])
        # Above one half in point estimate, but one deck carries it all.
        lopsided = {p: (("1",) * 4 if 0 in p else ("0.5", "0.5", "0", "1")) for p in PAIRS}
        out = self.decisions(full_field(x_results=lopsided))
        self.assertGreater(fractions.Fraction(out["figures"]["X"]["s_X"]), fractions.Fraction(1, 2))
        self.assertLessEqual(out["figures"]["X"]["jackknife"][0], 0.5)
        self.assertGreater(out["figures"]["X"]["block"][0], 0.5)
        self.assertFalse(out["flags"]["X1"])

    def test_e2_clears_material_alone_only_on_its_interval_rule(self):
        agree = self.decisions(full_field())
        self.assertEqual(agree["flags"]["E2"], "vmc-material alone")
        self.assertTrue(agree["flags"]["F1"] and agree["flags"]["F2"])
        flipped = self.decisions(full_field(p_strength=list(reversed(range(N)))))
        self.assertEqual(flipped["flags"]["E2"], "both referees")
        self.assertFalse(flipped["flags"]["F1"])
        # P agrees as well as F' does, but random play agrees just as well: no gain over random.
        same = self.decisions(full_field(r_strength=list(range(N))))
        self.assertEqual(same["flags"]["E2"], "both referees")

    def test_a_constant_ranking_leaves_e2_unresolved(self):
        arms = full_field()
        arms["R"] = [lines("R", {p: DRAW + DRAW for p in PAIRS})]
        out = self.decisions(arms)
        self.assertIsNone(out["figures"]["ranking"]["rho"]["F_R"])
        self.assertEqual(out["flags"]["E2"], "unresolved")
        self.assertIsNone(out["flags"]["F1"])
        self.assertIsNone(out["flags"]["F2"])

    def five_alike_r(self):
        """R where deck 5 wins every duel and every other duel is drawn: five
        decks share one rate, so a resample without deck 5 is constant."""
        return [lines("R", {(i, j): (LOSS + LOSS) if j == N - 1 else (DRAW + DRAW) for i, j in PAIRS})]

    def test_degenerate_resamples_are_discarded_and_counted(self):
        arms = full_field()
        arms["R"] = self.five_alike_r()
        out = self.decisions(arms)
        rk = out["figures"]["ranking"]
        self.assertGreater(rk["discarded"], 0)
        self.assertIsNotNone(rk["L_interval"])
        self.assertNotEqual(out["flags"]["E2"], "unresolved")

    def test_too_many_degenerate_resamples_leave_e2_unresolved(self):
        arms = full_field()
        arms["R"] = self.five_alike_r()
        saved = field.MAX_DRAWS
        field.MAX_DRAWS = field.BOOT + 100
        try:
            out = self.decisions(arms)
        finally:
            field.MAX_DRAWS = saved
        self.assertIsNone(out["figures"]["ranking"]["L_interval"])
        self.assertEqual(out["flags"]["E2"], "unresolved")

    def test_e2_uses_the_interval_not_the_point(self):
        # P agrees with F on every deck but one swap: L's point is small,
        # its upper bound over six decks is not.
        p = list(range(N))
        p[2], p[3] = p[3], p[2]
        out = self.decisions(full_field(p_strength=p))
        rk = out["figures"]["ranking"]
        self.assertLessEqual(rk["L"], 0.15)
        self.assertGreater(rk["L_interval"][1], 0.15)
        self.assertEqual(out["flags"]["E2"], "both referees")


if __name__ == "__main__":
    unittest.main()
