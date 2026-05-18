#!/usr/bin/env python3
"""Promote verify-passed candidates into the arcana-cards catalog.

This is the *landing* step of the subagent card-gen loop:

    bakeoff --dump-prompts DIR      # render prompts
    <subagents write DIR/NNN_slug.rs>
    verify_dir --dir DIR            # two-layer verify -> verify-report.jsonl
    land_cards.py --dir DIR         # <-- you are here

For every row whose verify outcome is `passed`, this copies
`DIR/<idx>_<slug>.rs` to `arcana-cards/src/<set>/<slug>.rs`, creating
the set module (`src/<set>/mod.rs`) and the `pub mod <set>;` line in
`src/lib.rs` as needed.

Deliberately conservative:

* **Dry-run by default.** Nothing is written without `--apply`.
  Landing is gated on human semantic review — layer-2 proves the
  card's *bones* match Scryfall, not that its rules text is
  correctly implemented (parametrised keywords, ability bodies, and
  non-integer P/T are not checked). Read the diff before applying.
* **Idempotent.** An existing destination is skipped unless
  `--force`; re-runs are safe.
* **Does not touch `register_seed`.** That list in `lib.rs` is a
  curated seed corpus, not the catalog. Landed cards are catalog
  modules; wiring them into a registry is a separate, intentional
  decision.

Exit status is non-zero if any selected card could not be landed
(missing source, or destination exists without `--force`).
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
CARDS_SRC = REPO / "arcana-cards" / "src"
LIB_RS = CARDS_SRC / "lib.rs"


def load_rows(report_path: Path, manifest_path: Path) -> list[dict]:
    """Join verify-report rows (outcome) with manifest rows (set)."""
    manifest: dict[int, dict] = {}
    for line in manifest_path.read_text().splitlines():
        if line.strip():
            r = json.loads(line)
            manifest[r["idx"]] = r
    rows = []
    for line in report_path.read_text().splitlines():
        if not line.strip():
            continue
        rep = json.loads(line)
        m = manifest.get(rep["idx"])
        if m is None:
            print(f"warn: report idx {rep['idx']} not in manifest", file=sys.stderr)
            continue
        rows.append({**rep, "set": m["set"], "manifest": m})
    return rows


def mod_ident(set_code: str) -> str:
    """Rust module identifier for a Scryfall set code.

    Scryfall codes are 3-5 chars and some are digit-leading
    (`10e`, `9ed`, `6ed`) — invalid Rust identifiers. Prefix those
    with `s` (`s10e`, `s9ed`); alpha-leading codes (`lea`, `m20`,
    `me3`) are valid idents and pass through unchanged. The set
    directory, the `pub mod` line, and the module path all use this
    sanitized form; the true code is preserved in the mod.rs doc.
    """
    return set_code if (set_code[:1].isalpha() or set_code[:1] == "_") \
        else f"s{set_code}"


def ensure_set_module(set_code: str, apply: bool) -> list[str]:
    ident = mod_ident(set_code)
    """Ensure src/<set>/mod.rs exists and lib.rs declares the set.

    Returns a list of human-readable actions (taken if `apply`,
    otherwise the actions that *would* be taken).
    """
    actions: list[str] = []
    set_dir = CARDS_SRC / ident
    mod_rs = set_dir / "mod.rs"

    if not mod_rs.exists():
        actions.append(f"create {mod_rs.relative_to(REPO)}")
        if apply:
            set_dir.mkdir(parents=True, exist_ok=True)
            note = "" if ident == set_code else \
                f" (Scryfall set code `{set_code}`, prefixed for a valid ident)"
            mod_rs.write_text(
                f"//! {set_code.upper()} — set module{note}. Auto-created by "
                f"land_cards.py; add a proper set summary when curating.\n"
            )

    lib = LIB_RS.read_text()
    decl = f"pub mod {ident};"
    if re.search(rf"^pub mod {re.escape(ident)};\s*$", lib, re.M) is None:
        actions.append(f"add `{decl}` to lib.rs")
        if apply:
            # Insert just before the `generated` staging module so
            # set modules stay grouped above it.
            anchor = "/// Staging area for arcana-gen card generations."
            if anchor in lib:
                lib = lib.replace(anchor, f"{decl}\n\n{anchor}", 1)
            else:  # fallback: after the last `pub mod <xxx>;`
                last = list(re.finditer(r"^pub mod \w+;\s*$", lib, re.M))[-1]
                lib = lib[: last.end()] + f"\n{decl}" + lib[last.end():]
            LIB_RS.write_text(lib)
    return actions


def ensure_mod_line(set_code: str, slug: str, apply: bool) -> list[str]:
    mod_rs = CARDS_SRC / mod_ident(set_code) / "mod.rs"
    decl = f"pub mod {slug};"
    text = mod_rs.read_text() if mod_rs.exists() else ""
    if re.search(rf"^{re.escape(decl)}\s*$", text, re.M):
        return []
    if apply:
        if text and not text.endswith("\n"):
            text += "\n"
        mod_rs.write_text(text + decl + "\n")
    return [f"add `{decl}` to {set_code}/mod.rs"]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--dir", required=True, type=Path,
                    help="dump dir (holds manifest.jsonl + verify-report.jsonl + candidates)")
    ap.add_argument("--report", type=Path,
                    help="verify report JSONL (default: <dir>/verify-report.jsonl)")
    ap.add_argument("--cards-dir", type=Path,
                    help="where the candidate .rs files live (default: --dir)")
    ap.add_argument("--apply", action="store_true",
                    help="actually write (default: dry-run)")
    ap.add_argument("--force", action="store_true",
                    help="overwrite an existing destination card file")
    args = ap.parse_args()

    dump_dir: Path = args.dir
    report = args.report or dump_dir / "verify-report.jsonl"
    cards_dir = args.cards_dir or dump_dir
    manifest = dump_dir / "manifest.jsonl"

    for p in (report, manifest):
        if not p.exists():
            print(f"error: {p} not found", file=sys.stderr)
            return 2

    rows = load_rows(report, manifest)
    passed = [r for r in rows if r.get("outcome") == "passed"]
    print(f"{len(rows)} report rows, {len(passed)} passed verify "
          f"({'APPLY' if args.apply else 'dry-run'})\n")

    if not passed:
        print("nothing to land.")
        return 0

    failures = 0
    for r in passed:
        set_code, slug, name = r["set"], r["slug"], r["name"]
        src = cards_dir / f"{r['idx']:03d}_{slug}.rs"
        dst = CARDS_SRC / mod_ident(set_code) / f"{slug}.rs"
        rel_dst = dst.relative_to(REPO)

        if not src.exists():
            print(f"  SKIP  {name}: source {src} missing")
            failures += 1
            continue
        if dst.exists() and not args.force:
            print(f"  SKIP  {name}: {rel_dst} exists (use --force)")
            failures += 1
            continue

        actions = ensure_set_module(set_code, args.apply)
        actions += ensure_mod_line(set_code, slug, args.apply)
        actions.append(f"{'write' if args.apply else 'would write'} {rel_dst}")
        if args.apply:
            dst.write_text(src.read_text())

        verb = "LAND " if args.apply else "PLAN "
        print(f"  {verb} {name}  [{set_code}]")
        for a in actions:
            print(f"         - {a}")

    print()
    if not args.apply:
        print("dry-run only. Re-run with --apply after human review of the "
              "generated sources (semantic correctness is NOT verified by "
              "layer 2).")
    if failures:
        print(f"{failures} card(s) could not be landed.", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
