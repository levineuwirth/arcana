#!/usr/bin/env python3
"""Synthetic checks for r4_field_count.py; no real decklists or Scryfall data.

    python3 -I arcana-ai/scripts/test_r4_field_count.py
"""

import contextlib
import gzip
import importlib.util
import io
import json
import os
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("count", os.path.join(HERE, "r4_field_count.py"))
count = importlib.util.module_from_spec(spec)
spec.loader.exec_module(count)

ORACLE = [
    {"name": "Modal Thing", "keywords": [], "type_line": "Instant", "mana_cost": "{1}{U}",
     "oracle_text": "Choose one —\n• Draw a card.\n• Counter target spell."},
    {"name": "Big X", "keywords": [], "type_line": "Sorcery", "mana_cost": "{X}{R}", "oracle_text": "Deal X damage."},
    {"name": "Modal X", "keywords": [], "type_line": "Sorcery", "mana_cost": "{X}{G}",
     "oracle_text": "Choose one —\n• Gain X life.\n• Draw X cards."},
    {"name": "Stage Light", "keywords": ["Spectacle"], "type_line": "Sorcery", "mana_cost": "{R}", "oracle_text": "…"},
    {"name": "Masked One", "keywords": ["Megamorph"], "type_line": "Creature — Elf", "mana_cost": "{3}{G}",
     "oracle_text": "Megamorph {G}"},
    {"name": "Old Car", "keywords": ["Crew"], "type_line": "Artifact — Vehicle", "mana_cost": "{3}",
     "oracle_text": "Crew 2"},
    {"name": "Engine Aura", "keywords": [], "type_line": "Enchantment", "mana_cost": "{2}",
     "oracle_text": "{2}, Sacrifice Engine Aura: Draw a card."},
    {"name": "Plain Land", "keywords": [], "type_line": "Land", "mana_cost": "", "oracle_text": "{T}: Add {C}."},
    {"name": "Front // Back", "keywords": [], "card_faces": [
        {"name": "Front", "type_line": "Instant", "mana_cost": "{1}", "oracle_text": "Choose one —\n• A.\n• B."},
        {"name": "Back", "type_line": "Instant", "mana_cost": "{2}", "oracle_text": "Draw."}]},
    {"name": "Phoenix", "keywords": [], "type_line": "Creature — Phoenix", "mana_cost": "{2}{R}",
     "oracle_text": "When Phoenix dies, create a token."},
    {"name": "Graveyard Watcher", "keywords": [], "type_line": "Creature — Zombie", "mana_cost": "{2}{B}",
     "oracle_text": "Whenever another creature dies, gain 1 life."},
]


def deck(fmt, src, resolved, missing):
    return {"format": fmt, "source": src, "resolved": resolved,
            "listed": resolved + sum(c for _, c in missing), "missing": missing}


class Count(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.oracle = os.path.join(self.tmp.name, "oracle.jsonl.gz")
        with gzip.open(self.oracle, "wt", encoding="utf-8") as f:
            for c in ORACLE:
                f.write(json.dumps(c) + "\n")
        self.by_name = count.load_oracle(self.oracle)

    def tearDown(self):
        self.tmp.cleanup()

    def fams(self, name):
        return count.families(count.lookup(self.by_name, name))

    def test_families_from_oracle_data(self):
        self.assertEqual(self.fams("Modal Thing"), {"modal"})
        self.assertEqual(self.fams("Big X"), {"x_spell"})
        self.assertEqual(self.fams("Modal X"), {"modal", "x_spell"})
        self.assertEqual(self.fams("Stage Light"), {"spectacle"})
        self.assertEqual(self.fams("Masked One"), {"morph"})
        self.assertEqual(self.fams("Old Car"), {"vehicle"})
        self.assertEqual(self.fams("Engine Aura"), {"activated_enchantment"})
        self.assertEqual(self.fams("Plain Land"), set())
        self.assertEqual(self.fams("Front"), {"modal"}, "a face name finds its card")
        self.assertEqual(self.fams("Front // Back"), {"modal"})

    def test_a_deck_is_sized_by_its_listed_copies(self):
        # The regression: 56 resolved and 4 missing is a 60-card deck. Sized
        # by resolved copies it would never count as blocked at 60.
        decks = [deck("PI", "a", 56, [["Modal Thing", 4]])]
        self.assertEqual(len(count.blocked_sixty(decks)), 1)
        req = count.requirements(decks, self.by_name)
        table = {r["family"]: r for r in count.family_table(decks, req, ["modal"])}
        self.assertEqual(table["modal"]["alone"], 1)

    def test_a_card_needing_two_families_needs_both(self):
        decks = [deck("PI", "a", 57, [["Modal X", 3]])]
        req = count.requirements(decks, self.by_name)
        table = {r["family"]: r for r in count.family_table(decks, req, ["modal", "x_spell"])}
        self.assertEqual(table["modal"]["alone"], 0)
        self.assertEqual(table["x_spell"]["alone"], 0)
        self.assertTrue(req["Modal X"] <= {"modal", "x_spell"})

    def test_outside_and_unknown_cards_block_any_family(self):
        decks = [deck("PI", "a", 56, [["Modal Thing", 2], ["Plain Land", 2]]),
                 deck("PI", "b", 59, [["No Such Card", 1]])]
        req = count.requirements(decks, self.by_name)
        self.assertEqual(req["Plain Land"], frozenset({"outside"}))
        self.assertEqual(req["No Such Card"], frozenset({"unknown_name"}))
        table = {r["family"]: r for r in count.family_table(decks, req, ["modal"])}
        self.assertEqual(table["modal"]["alone"], 0)

    def test_greedy_takes_the_card_completing_most_decks(self):
        decks = [deck("PI", "a", 59, [["A", 1]]), deck("PI", "b", 59, [["A", 1]]),
                 deck("PI", "c", 58, [["B", 1], ["C", 1]]), deck("PI", "d", 59, [["C", 1]]),
                 deck("PI", "e", 50, [["A", 1]])]  # 51 listed: not a 60-card deck
        order, _, blocked = count.greedy(decks, n=3)
        self.assertEqual(blocked, 4)
        self.assertEqual(order[0], ("A", 2, 2))
        self.assertEqual(order[1][0], "C")
        self.assertEqual(order[2], ("B", 1, 4))

    def test_greedy_breaks_ties_by_touch_then_name(self):
        # A and B each complete one deck; B also touches a deck still short of
        # C, so B goes first although A comes first by name.
        decks = [deck("PI", "a", 59, [["A", 1]]), deck("PI", "b", 59, [["B", 1]]),
                 deck("PI", "c", 58, [["B", 1], ["C", 1]])]
        order, _, _ = count.greedy(decks, n=1)
        self.assertEqual(order[0][0], "B", "equal completions: the card touching more decks first")
        order, _, _ = count.greedy([deck("PI", "a", 59, [["Q", 1]]), deck("PI", "b", 59, [["P", 1]])], n=1)
        self.assertEqual(order[0][0], "P", "then by name")

    def test_self_leaves_trigger_is_the_cards_own(self):
        self.assertTrue(count.self_leaves_trigger(count.lookup(self.by_name, "Phoenix")))
        self.assertFalse(count.self_leaves_trigger(count.lookup(self.by_name, "Graveyard Watcher")))

    def test_the_families_command_reads_listed_size(self):
        extract = os.path.join(self.tmp.name, "x.jsonl")
        with open(extract, "w", encoding="utf-8") as f:
            f.write(json.dumps(deck("PI", "a", 56, [["Modal Thing", 4]])) + "\n")
            f.write(json.dumps(deck("PI", "b", 60, [])) + "\n")
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            count.main(["families", extract, self.oracle])
        text = out.getvalue()
        self.assertIn("== PI: 2 decks, 1 playable at 60, 1 blocked at 60", text)
        self.assertIn("unblocked by closing every R4 family together: 1", text)


if __name__ == "__main__":
    unittest.main()
