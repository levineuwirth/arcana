#!/usr/bin/env python3
"""
l3_blocker_taxonomy.py — classify card-gen verify-report failures by engine gap.

Ingests:
  <dir>/verify-report.jsonl   one record per emitted card
  <dir>/prompts/<idx>_<slug>.txt   per-card prompt (oracle text lives here)

Emits to stdout:
  - primary-bucket histogram (first taxonomy match wins)
  - any-match histogram (a card counts in every bucket it matches)
  - per-bucket sample slugs + oracle text
  - unclassified residue listing

The taxonomy is keyed off oracle-text regex patterns. Order matters in the
PRIMARY pass: more-specific patterns must precede more-general ones.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Iterable

# ---------------------------------------------------------------------------
# Taxonomy. Each entry is (bucket_name, compiled_pattern, subsystem_tag).
# subsystem_tag groups buckets under the engine-survey's high-level subsystems
# (Effect variant, TriggerCondition, PendingTrigger accessor, Subsystem, etc.)
# so the final report can roll up by ROI category.
# ---------------------------------------------------------------------------

# (bucket, pattern, subsystem) — order matters for the PRIMARY pass.
TAXONOMY: list[tuple[str, re.Pattern, str]] = [
    # --- Whole game-within-a-game subsystems (high-specificity first) -------
    ("ring_tempts",
     re.compile(r"\b(the ring tempts you|ring[- ]bearer)\b", re.I), "subsystem_status_game"),
    ("day_night",
     re.compile(r"\b(daybound|nightbound|it becomes day|it becomes night)\b", re.I), "subsystem_status_game"),
    ("monarch_initiative",
     re.compile(r"\b(become the monarch|take the initiative|you're the monarch|you have the initiative)\b", re.I),
     "subsystem_status_game"),
    ("suspect",
     re.compile(r"\b(suspect|suspected)\b", re.I), "subsystem_status_game"),
    ("max_speed",
     re.compile(r"\b(start your engines|max speed|your speed)\b", re.I), "subsystem_status_game"),

    # --- Vehicles / Crew / Saddle / Mount ----------------------------------
    ("vehicles_crew",
     re.compile(r"\b(crew \d+|saddle \d+|becomes a vehicle|mount)\b", re.I), "subsystem_vehicles"),

    # --- Un-set / joke / out-of-format mechanics (hard isolate first) ------
    ("unset_contraption",
     re.compile(r"\bcontraption\b", re.I), "subsystem_unset"),
    ("unset_attraction",
     re.compile(r"\battraction\b", re.I), "subsystem_unset"),
    ("unset_stickers",
     re.compile(r"\b(sticker|stickers)\b", re.I), "subsystem_unset"),
    ("unset_outside_game",
     re.compile(r"\b(outside the game|wager|lines of text|read aloud|ask a player)\b", re.I), "subsystem_unset"),
    ("unset_dice_misc",
     re.compile(r"\b(d20|twenty[- ]sided|nine[- ]sided)\b", re.I), "subsystem_unset"),

    # --- Big mechanics that map to a single missing Effect variant ---------
    ("effect_explore",
     re.compile(r"\bexplores?\b", re.I), "effect_variant"),
    ("effect_discover",
     re.compile(r"\bdiscover( \d+| again)?\b", re.I), "effect_variant"),
    ("effect_incubate",
     re.compile(r"\bincubate \d+\b", re.I), "effect_variant"),
    ("effect_amass",
     re.compile(r"\bamass \w+ \d+\b", re.I), "effect_variant"),
    ("effect_seek",
     re.compile(r"\bseek a\b", re.I), "effect_variant"),
    ("effect_conjure",
     re.compile(r"\bconjure a\b", re.I), "effect_variant"),
    ("effect_manifest_dread",
     re.compile(r"\b(manifest dread|cloak)\b", re.I), "effect_variant"),
    ("effect_perpetually",
     re.compile(r"\bperpetually\b", re.I), "effect_variant"),
    ("effect_plot",
     re.compile(r"\bplot\b", re.I), "effect_variant"),
    ("effect_bargain",
     re.compile(r"\bas an additional cost to cast this spell, you may sacrifice\b", re.I), "effect_variant"),
    ("effect_dungeon_venture",
     re.compile(r"\b(venture into the dungeon|complete a dungeon|take the initiative)\b", re.I), "effect_variant"),
    ("effect_lesson_learn",
     re.compile(r"\b(learn\b|lesson card)", re.I), "effect_variant"),
    ("effect_die_roll",
     re.compile(r"\broll (a|two|three) (six|d6)[\- ]?sided|\bwhenever you roll", re.I), "effect_variant"),
    ("effect_coin_flip",
     re.compile(r"\bflip a coin\b", re.I), "effect_variant"),
    ("effect_become_a_copy",
     re.compile(r"\bbecomes? a copy of\b", re.I), "effect_variant"),
    ("effect_meld",
     re.compile(r"\bmeld them into\b", re.I), "effect_variant"),
    ("effect_role_token",
     re.compile(r"\bcreate a [a-z\- ]*role token\b", re.I), "effect_variant"),
    ("effect_pay_or_decline",
     re.compile(r"\bunless\s+(you|that player|its controller|they)\s+(pay|pays|sacrifice|sacrifices|discard|discards|exile|exiles)\b", re.I),
     "effect_variant"),
    ("effect_may_pay_then_do",
     re.compile(r"\byou may pay (\{[^}]+\}|\d+ life)\.\s*(if|when) you do\b", re.I), "effect_variant"),
    ("effect_may_play_from_exile",
     re.compile(r"\b(you may (cast|play) (it|that card|the exiled card)|until end of turn,? you may play\b)", re.I),
     "effect_variant"),
    ("effect_grant_protection_from",
     re.compile(r"\bgains? protection from\b", re.I), "effect_variant"),
    ("effect_phasing",
     re.compile(r"\bphases? (out|in)\b", re.I), "effect_variant"),
    ("effect_boast",
     re.compile(r"\bboast\b", re.I), "effect_variant"),
    ("effect_specialize",
     re.compile(r"\b(specializes?|specialization)\b", re.I), "effect_variant"),
    ("effect_expend",
     re.compile(r"\bexpend \d+\b", re.I), "effect_variant"),
    ("effect_look_top1",
     re.compile(r"\blook at the top card of\b", re.I), "effect_variant"),
    ("effect_hand_zone_to_other",
     re.compile(r"\bfrom (their|your) hand (onto|on top of|to)\b", re.I), "effect_variant"),
    ("effect_spellbook_draft",
     re.compile(r"\bdraft a card from\b", re.I), "effect_variant"),
    ("effect_skip_untap",
     re.compile(r"\b(doesn't untap during|skip your next untap step|skips? (their|its controller's) next untap step)\b", re.I),
     "effect_variant"),
    ("effect_damage_divided",
     re.compile(r"\bdamage divided\b", re.I), "effect_variant"),
    ("effect_devotion",
     re.compile(r"\bdevotion to\b", re.I), "effect_variant"),
    ("effect_modified_predicate",
     re.compile(r"\bmodified (creature|permanent)\b", re.I), "effect_variant"),
    ("effect_win_lose_game",
     re.compile(r"\b(you win the game|that player loses the game|you lose the game)\b", re.I), "effect_variant"),
    ("effect_face_up_exile_interaction",
     re.compile(r"\b(face-up exiled|exiled (card|cards) (face up|face down))\b", re.I), "effect_variant"),
    ("effect_mana_any_color",
     re.compile(r"\b(adds? (one|two|three|that much) mana of any (one |type )?color|adds? one mana of any type)\b", re.I),
     "effect_variant"),
    ("effect_choose_creature_type",
     re.compile(r"\bchoose a creature type\b", re.I), "effect_variant"),
    ("effect_reveal_until_creature",
     re.compile(r"\breveal cards? from the top.* until you reveal\b", re.I), "effect_variant"),
    ("effect_count_then_filter",
     re.compile(r"\bcount the number of\b", re.I), "effect_variant"),
    ("effect_opponent_pays_optional_damage",
     re.compile(r"\bany opponent may have it deal\b", re.I), "effect_variant"),
    ("effect_blink_under_opponent_control",
     re.compile(r"\bunder (its|that player's|your|opponent's) control\b", re.I), "effect_variant"),
    ("trigger_player_taps_subtype_for_mana",
     re.compile(r"\bwhenever a player taps a (mountain|island|forest|plains|swamp)\b", re.I), "trigger_condition"),
    ("subtle_phase_steal",
     re.compile(r"\b(steal that phase|skips? each instance of the chosen)\b", re.I), "subsystem_continuous"),
    ("effect_attacking_creatures_anthem",
     re.compile(r"\b(each |all )?other attacking creatures? (get|gain)\b", re.I), "subsystem_static_anthem"),
    ("effect_distribute_counters",
     re.compile(r"\bdistribute (\w+|that many) (\+1/\+1|\-1/\-1) counters?\b", re.I), "effect_variant"),
    ("effect_exchange_pt",
     re.compile(r"\bexchange (its|the|target creature's) (power and toughness|base power)\b", re.I), "effect_variant"),
    ("effect_change_base_pt_indefinite",
     re.compile(r"\bchange (the )?base power and toughness\b", re.I), "effect_variant"),
    ("effect_attack_command",
     re.compile(r"\b(blocks it this combat if able|attacks .+ if able|attack each combat if)\b", re.I), "effect_variant"),
    ("effect_token_into_opponent_library",
     re.compile(r"\bshuffle (them|it|all of them) into (target opponent's|your) library\b", re.I), "effect_variant"),
    ("effect_exile_library",
     re.compile(r"\bexile all cards from (your|target opponent's|that player's) library\b", re.I), "effect_variant"),
    ("intervening_if_defender_filter",
     re.compile(r"\bif defending player controls (no|a|an?)\b", re.I), "trigger_condition"),
    ("intervening_if_count_named",
     re.compile(r"\bif you control \w+ or more (creatures? named|permanents? named)\b", re.I), "trigger_condition"),
    ("effect_search_basic_lands",
     re.compile(r"\bsearch (your|their) library for (a |up to \w+ )?basic land\b", re.I), "effect_variant"),
    ("effect_return_at_next_upkeep",
     re.compile(r"\breturn (it|this card) to the battlefield .{0,30}at the beginning of (their|your) next upkeep\b", re.I),
     "effect_variant"),

    # --- Modal effects (the highest-ROI single subsystem) ------------------
    ("modal_choose",
     re.compile(r"\bchoose (one|two|three|up to (one|two|three|x))\b[\s\S]{0,40}—", re.I), "subsystem_modal"),

    # --- Missing TriggerCondition variants ---------------------------------
    ("trigger_ability_activated",
     re.compile(r"\b(whenever|when) [^.]*?activates? an ability\b", re.I), "trigger_condition"),
    ("trigger_blocks_or_blocked",
     re.compile(r"\b(whenever|when) [^.]*?(blocks or (becomes |is )?blocked|is blocked or blocks)\b", re.I),
     "trigger_condition"),
    ("trigger_attacks_or_blocks",
     re.compile(r"\bwhenever this creature attacks or blocks\b", re.I), "trigger_condition"),
    ("trigger_attacks_alone",
     re.compile(r"\battacks alone\b", re.I), "trigger_condition"),
    ("trigger_leaves_battlefield",
     re.compile(r"\b(whenever|when) [^.]*?leaves (the|your) battlefield\b", re.I), "trigger_condition"),
    ("trigger_cycled",
     re.compile(r"\b(whenever|when) [^.]*?cycles?\b", re.I), "trigger_condition"),
    ("trigger_beginning_combat",
     re.compile(r"\bat the beginning of combat\b", re.I), "trigger_condition"),
    ("trigger_poison_gained",
     re.compile(r"\b(whenever|when) [^.]*?(gets? a poison counter|becomes corrupted)\b", re.I), "trigger_condition"),
    ("trigger_lifelost",
     re.compile(r"\b(whenever|when) [^.]*?loses life\b", re.I), "trigger_condition"),
    ("trigger_creature_etb_other",
     re.compile(r"\bwhenever another (creature|nontoken creature|artifact|enchantment) enters\b", re.I),
     "trigger_condition"),
    ("trigger_self_transforms",
     re.compile(r"\b(whenever|when) this creature transforms\b", re.I), "trigger_condition"),
    ("trigger_tapped_for_mana",
     re.compile(r"\btaps? (an? |another )?(artifact|creature|land) for mana\b", re.I), "trigger_condition"),
    ("trigger_state_based_self_sacrifice",
     re.compile(r"\b(when|whenever) you control no [^.]+,\s*sacrifice (this|it)\b", re.I), "trigger_condition"),

    # --- Missing PendingTrigger accessors ----------------------------------
    ("accessor_active_player",
     re.compile(r"\bat the beginning of (each|that) player(?:'s)? (upkeep|end step|draw step|main phase)\b", re.I),
     "trigger_accessor"),
    ("accessor_blocker_blocked_by",
     re.compile(r"\b(blocked by .{0,40} creature|that blocked creature|blocking creature|the creature it('s|s) blocking|blocked creatures?)\b", re.I),
     "trigger_accessor"),
    ("accessor_ability_source",
     re.compile(r"\bthat ability(?:'s)? (source|controller)\b", re.I), "trigger_accessor"),
    ("accessor_spell_color_or_type",
     re.compile(r"\b(that spell's color|that spell's type|that spell is a (red|blue|green|white|black))\b", re.I),
     "trigger_accessor"),

    # --- Dynamic target counts / scaling -----------------------------------
    ("dynamic_target_count",
     re.compile(r"\bup to (that many|x) target\b", re.I), "filter_or_count"),
    ("dynamic_for_each",
     re.compile(r"\bfor each (other |different )?(creature|opponent|land|artifact|spell|card|player)\b", re.I),
     "filter_or_count"),
    ("dynamic_equal_to_number_of",
     re.compile(r"\b(equal to (the |twice the )?number of|where x is the number of)\b", re.I), "filter_or_count"),
    ("dynamic_greatest_among",
     re.compile(r"\b(greatest|highest|lowest) (power|toughness|mana value|life total)\b", re.I), "filter_or_count"),
    ("dynamic_equal_to_its",
     re.compile(r"\bequal to its (power|toughness|mana value)\b", re.I), "filter_or_count"),
    ("per_player_relative_count",
     re.compile(r"\b(players? who control more|players? with more|each opponent who|player who has the highest|player who has the lowest)\b", re.I),
     "filter_or_count"),
    ("per_turn_typed_counter",
     re.compile(r"\b(died|entered|cast|drawn|discarded)[^.]{0,30}\bthis turn\b", re.I), "filter_or_count"),
    ("vote_mechanic",
     re.compile(r"\b(votes? for|finish voting|will of the council)\b", re.I), "filter_or_count"),
    ("sequential_per_player",
     re.compile(r"\bstarting with you, each (player|opponent)\b", re.I), "filter_or_count"),
    ("multi_counter_kind_enum",
     re.compile(r"\bfor each kind of counter\b", re.I), "filter_or_count"),

    # --- ObjectFilter expressiveness ---------------------------------------
    ("filter_subtype_or",
     re.compile(r"\b[A-Z][a-z]+ (creature )?or [A-Z][a-z]+ (creature|spell|card)\b"), "filter_or_count"),
    ("filter_legendary_supertype",
     re.compile(r"\b(legendary creature|legendary permanent|legendary spell|nonlegendary)\b", re.I), "filter_or_count"),
    ("filter_non_subtype",
     re.compile(r"\bnon-[A-Z][a-z]+ (creature|spell|permanent)\b"), "filter_or_count"),

    # --- Continuous / replacement / static-grant ---------------------------
    ("continuous_as_long_as",
     re.compile(r"\bas long as\b", re.I), "subsystem_continuous"),
    ("replacement_etb_tapped_or_copy",
     re.compile(r"\benters (the battlefield )?(tapped|as a copy)\b", re.I), "subsystem_replacement"),
    ("replacement_if_would",
     re.compile(r"\bif (a|that|this) .{0,40}would .{0,40} instead\b", re.I), "subsystem_replacement"),
    ("static_anthem_other_creatures",
     re.compile(r"\bother creatures you control (get|have)\b", re.I), "subsystem_static_anthem"),
    ("static_creatures_you_control_have",
     re.compile(r"\bcreatures you control (get|have)\b", re.I), "subsystem_static_anthem"),

    # --- Saga / Class / Adventure / Battle / MDFC / Planeswalker -----------
    ("type_saga",
     re.compile(r"^\s*Type line:.*\bsaga\b", re.I | re.M), "subsystem_typed_card"),
    ("type_class",
     re.compile(r"^\s*Type line:.*\bclass\b", re.I | re.M), "subsystem_typed_card"),
    ("type_battle",
     re.compile(r"^\s*Type line:.*\bbattle\b", re.I | re.M), "subsystem_typed_card"),
    ("type_planeswalker",
     re.compile(r"^\s*Type line:.*\bplaneswalker\b", re.I | re.M), "subsystem_typed_card"),

    # --- Targets / minor stuff ---------------------------------------------
    ("reveal_top_n_with_filter",
     re.compile(r"\b(look at|reveal) the top \w+ cards? of\b", re.I), "filter_or_count"),
    ("counter_special_kind",
     re.compile(r"\b(stun|shield|finality|incubation|study|defense|brick|wish|verse|page|chapter|lore) counter\b", re.I),
     "filter_or_count"),

    # --- Optional / "may" gates -- demoted LAST: catches cards whose
    # primary blocker is "the engine has no choice point", as opposed to
    # cards where "may" is a leaf optional on top of an already-modeled
    # effect (those have a more specific bucket above).
    ("may_optional",
     re.compile(r"\byou may\b", re.I), "subsystem_choice_may"),

    # --- L3 dynamic-literal class (verifier-flagged) -----------------------
    # This bucket is detected from the verify-report reason, not oracle text.
]

PRIMARY_BUCKET_ORDER = [b for (b, _, _) in TAXONOMY]
BUCKET_SUBSYSTEM = {b: s for (b, _, s) in TAXONOMY}

ORACLE_RE = re.compile(r"^Oracle text:\s*\n(.*?)\n\n", re.DOTALL | re.MULTILINE)
TYPELINE_RE = re.compile(r"^Type line:\s*(.+)$", re.MULTILINE)


def load_prompt_text(prompts_dir: Path, idx: int, slug: str) -> str | None:
    # Prompts are zero-padded to 3 digits in our pipeline.
    candidates = list(prompts_dir.glob(f"{idx:03d}_{slug}.txt"))
    if not candidates:
        candidates = list(prompts_dir.glob(f"*_{slug}.txt"))
    if not candidates:
        return None
    return candidates[0].read_text(encoding="utf-8", errors="replace")


def extract_oracle(prompt_text: str) -> str:
    m = ORACLE_RE.search(prompt_text)
    return m.group(1).strip() if m else ""


def extract_typeline(prompt_text: str) -> str:
    m = TYPELINE_RE.search(prompt_text)
    return m.group(1).strip() if m else ""


def classify(prompt_text: str, oracle: str) -> list[str]:
    """Return every taxonomy bucket whose pattern matches.
    Some patterns search the whole prompt (typeline / reveal); most use oracle."""
    hits: list[str] = []
    for bucket, pat, _subsys in TAXONOMY:
        haystack = prompt_text if bucket.startswith("type_") else oracle
        if pat.search(haystack):
            hits.append(bucket)
    return hits


def primary_bucket(hits: list[str]) -> str:
    return hits[0] if hits else "unclassified"


def report(dir_path: Path) -> int:
    report_path = dir_path / "verify-report.jsonl"
    prompts_dir = dir_path / "prompts"
    if not report_path.exists():
        print(f"error: {report_path} missing", file=sys.stderr)
        return 1
    if not prompts_dir.is_dir():
        print(f"error: {prompts_dir} missing", file=sys.stderr)
        return 1

    records = [json.loads(line) for line in report_path.read_text().splitlines() if line.strip()]
    outcomes = Counter(r["outcome"] for r in records)

    print("== Verify outcomes ==")
    for k in ("passed", "layer1_failed", "layer2_failed", "layer3_failed", "not_generated"):
        print(f"  {k:18s} {outcomes.get(k, 0):>5}")
    print(f"  TOTAL              {sum(outcomes.values()):>5}")
    print()

    # Bucket the L3 dynamic-literal class out before regex classification.
    dynlit_by_kind: Counter[str] = Counter()
    DYNLIT_KEY = "L3_dynamic_literal"

    primary_counter: Counter[str] = Counter()
    any_counter: Counter[str] = Counter()
    subsys_counter: Counter[str] = Counter()
    per_outcome_primary: dict[str, Counter[str]] = {"layer1_failed": Counter(), "layer3_failed": Counter()}
    samples: dict[str, list[tuple[str, str]]] = defaultdict(list)
    unclassified_l3: list[tuple[int, str, str]] = []
    classified_targets = ("layer3_failed", "layer1_failed")

    for r in records:
        if r["outcome"] not in classified_targets:
            continue

        # L3 dynamic-literal: classify from reason, not oracle text.
        if r["outcome"] == "layer3_failed" and "dynamic-literal" in (r.get("reason") or ""):
            # extract the cue: oracle says "for each" / "equal to ..." / etc.
            cue_match = re.search(r'oracle says "([^"]+)"', r["reason"])
            cue = cue_match.group(1) if cue_match else "unknown-cue"
            dynlit_by_kind[cue] += 1
            primary_counter[DYNLIT_KEY] += 1
            per_outcome_primary[r["outcome"]][DYNLIT_KEY] += 1
            any_counter[DYNLIT_KEY] += 1
            subsys_counter["filter_or_count"] += 1
            if len(samples[DYNLIT_KEY]) < 8:
                samples[DYNLIT_KEY].append((r["slug"], f"[{cue}]"))
            continue

        prompt_text = load_prompt_text(prompts_dir, r["idx"], r["slug"])
        if prompt_text is None:
            unclassified_l3.append((r["idx"], r["slug"], "(no prompt file)"))
            primary_counter["unclassified"] += 1
            per_outcome_primary[r["outcome"]]["unclassified"] += 1
            continue
        oracle = extract_oracle(prompt_text)
        hits = classify(prompt_text, oracle)
        bucket = primary_bucket(hits)
        primary_counter[bucket] += 1
        per_outcome_primary[r["outcome"]][bucket] += 1
        for h in hits:
            any_counter[h] += 1
            subsys_counter[BUCKET_SUBSYSTEM[h]] += 1
        if len(samples[bucket]) < 8:
            samples[bucket].append((r["slug"], oracle[:140]))
        if bucket == "unclassified" and r["outcome"] == "layer3_failed":
            unclassified_l3.append((r["idx"], r["slug"], oracle[:160]))

    # --- Primary histogram ---------------------------------------------------
    print("== PRIMARY-bucket histogram (L1+L3 fails, first match wins) ==")
    print(f"{'bucket':<35} {'count':>6} {'%':>7}  subsystem")
    total_primary = sum(primary_counter.values())
    for bucket, count in primary_counter.most_common():
        pct = count / total_primary * 100 if total_primary else 0
        subsys = BUCKET_SUBSYSTEM.get(bucket, "—" if bucket == "unclassified" else "L3_dynlit")
        print(f"  {bucket:<33} {count:>6} {pct:>6.1f}%  {subsys}")
    print(f"  {'TOTAL':<33} {total_primary:>6}")
    print()

    # --- L1 vs L3 split ------------------------------------------------------
    print("== PRIMARY-bucket split by outcome ==")
    all_buckets = sorted(
        set(per_outcome_primary["layer1_failed"]) | set(per_outcome_primary["layer3_failed"]),
        key=lambda b: -(per_outcome_primary["layer1_failed"][b] + per_outcome_primary["layer3_failed"][b]),
    )
    print(f"{'bucket':<35} {'L1':>5} {'L3':>5}")
    for b in all_buckets:
        l1 = per_outcome_primary["layer1_failed"][b]
        l3 = per_outcome_primary["layer3_failed"][b]
        print(f"  {b:<33} {l1:>5} {l3:>5}")
    print()

    # --- Any-match histogram (a card double-counts) --------------------------
    print("== ANY-match histogram (a card appears in every bucket it matches) ==")
    for bucket, count in any_counter.most_common():
        print(f"  {bucket:<33} {count:>6}")
    print()

    # --- Subsystem roll-up (PRIMARY basis: each card counted once) ----------
    primary_subsys_counter: Counter[str] = Counter()
    for bucket, count in primary_counter.items():
        if bucket == "unclassified":
            primary_subsys_counter["unclassified"] += count
        elif bucket == "L3_dynamic_literal":
            primary_subsys_counter["L3_dynlit"] += count
        else:
            primary_subsys_counter[BUCKET_SUBSYSTEM[bucket]] += count

    print("== Subsystem roll-up (PRIMARY basis — each card counted once) ==")
    for subsys, count in primary_subsys_counter.most_common():
        pct = count / total_primary * 100 if total_primary else 0
        print(f"  {subsys:<30} {count:>5} ({pct:>5.1f}%)")
    print()

    # --- Phase grouping (matches the engine-survey ROI ranking) -------------
    PHASE_MAP = {
        "trigger_condition": "PhaseA_cheap_wide",
        "trigger_accessor": "PhaseA_cheap_wide",
        "filter_or_count": "PhaseA_cheap_wide",
        "L3_dynlit": "PhaseA_cheap_wide",
        "subsystem_modal": "PhaseB_modal",
        "effect_variant": "PhaseC_effect_catalog",
        "subsystem_continuous": "PhaseD_subsystems",
        "subsystem_replacement": "PhaseD_subsystems",
        "subsystem_static_anthem": "PhaseD_subsystems",
        "subsystem_choice_may": "PhaseD_subsystems",
        "subsystem_status_game": "PhaseD_subsystems",
        "subsystem_vehicles": "PhaseD_subsystems",
        "subsystem_typed_card": "PhaseD_subsystems",
        "subsystem_unset": "Excluded_unset",
        "unclassified": "Unclassified_longtail",
    }
    phase_counter: Counter[str] = Counter()
    for subsys, count in primary_subsys_counter.items():
        phase = PHASE_MAP.get(subsys, "Other")
        phase_counter[phase] += count

    print("== Phase grouping (ROI buckets from engine-survey) ==")
    for phase, count in sorted(phase_counter.items(), key=lambda kv: -kv[1]):
        pct = count / total_primary * 100 if total_primary else 0
        print(f"  {phase:<28} {count:>5} ({pct:>5.1f}%)")
    print()

    # --- Subsystem roll-up (ANY-match basis: overstates total) --------------
    print("== Subsystem roll-up (ANY-match basis: cards may double-count) ==")
    for subsys, count in subsys_counter.most_common():
        print(f"  {subsys:<30} {count:>6}")
    print()

    # --- Dynamic-literal breakdown ------------------------------------------
    if dynlit_by_kind:
        print("== L3 dynamic-literal by cue ==")
        for cue, count in dynlit_by_kind.most_common():
            print(f"  '{cue}'  {count}")
        print()

    # --- Samples per bucket -------------------------------------------------
    print("== Samples per bucket (≤8 each) ==")
    for bucket, count in primary_counter.most_common():
        print(f"-- {bucket}  ({count} total) --")
        for slug, oracle in samples[bucket]:
            print(f"  {slug:32s}  {oracle}")
        print()

    # --- Unclassified residue listing ---------------------------------------
    print(f"== Unclassified L3 residue ({len(unclassified_l3)} cards) ==")
    for idx, slug, oracle in unclassified_l3[:40]:
        print(f"  {idx:>4}  {slug:32s}  {oracle}")
    if len(unclassified_l3) > 40:
        print(f"  ... and {len(unclassified_l3) - 40} more")
    print()

    return 0


def main(argv: Iterable[str] | None = None) -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--dir", required=True, help="card-gen run dir containing verify-report.jsonl + prompts/")
    args = p.parse_args(argv)
    return report(Path(args.dir))


if __name__ == "__main__":
    sys.exit(main())
