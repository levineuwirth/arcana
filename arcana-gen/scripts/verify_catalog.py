#!/usr/bin/env python3
"""Belt-and-suspenders verifier for the *landed* arcana-cards catalog.

`verify_dir` grades candidate sources in a cardgen run dir. This tool
grades the bytes that are actually committed under
`arcana-cards/src/<set>/<slug>.rs`, so a catalog can be re-attested
independently of whatever run dir produced it (e.g. after an engine
or structural-harness change tightens Layer 2).

It does NOT reimplement verification. It stages each landed card into
the flat `<idx>_<slug>.rs` layout `verify_dir` expects, then runs the
already-vetted `verify_dir --batch` against those staged catalog
bytes. The set→module mapping is imported from `land_cards.py` so it
can never drift from how cards were landed.

Manifest = the deterministic Scryfall expectation set
(`bakeoff --dump-prompts`, card_seed 0). Cards present in the catalog
but absent from the manifest are reported as `unmapped` (can't be
attested without an expectation row); manifest rows with no landed
file are simply `not_landed` and ignored in the pass/fail tally.

Usage:
    python3 arcana-gen/scripts/verify_catalog.py \
        [--manifest target/cardgen/run01/manifest.jsonl] \
        [--cards-src arcana-cards/src] [--report PATH] [--no-batch]

Exit code is non-zero iff a landed, manifest-mapped card fails.
"""
from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

# Reuse the exact set→module-ident rule cards were landed with.
sys.path.insert(0, str(Path(__file__).resolve().parent))
from land_cards import mod_ident  # noqa: E402

REPO = Path(__file__).resolve().parents[2]
VERIFY_DIR_BIN = REPO / "target" / "debug" / "verify_dir"


def build_verify_dir() -> None:
    if VERIFY_DIR_BIN.exists():
        return
    print("verify_catalog: building verify_dir …")
    subprocess.run(
        ["cargo", "build", "-q", "-p", "arcana-gen", "--bin", "verify_dir"],
        cwd=REPO, check=True,
    )


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--manifest", type=Path,
                    default=REPO / "target/cardgen/run01/manifest.jsonl",
                    help="deterministic manifest.jsonl (card_seed 0)")
    ap.add_argument("--cards-src", type=Path,
                    default=REPO / "arcana-cards" / "src",
                    help="catalog source root (default: arcana-cards/src)")
    ap.add_argument("--report", type=Path,
                    help="copy the per-card JSONL report here")
    ap.add_argument("--no-batch", action="store_true",
                    help="serial verify (default: batched)")
    args = ap.parse_args()

    if not args.manifest.exists():
        print(f"manifest not found: {args.manifest}\n"
              f"  generate one (deterministic) with:\n"
              f"  cargo run -q -p arcana-gen --bin bakeoff -- "
              f"--dump-prompts <dir> --tiers 1,2 "
              f"--shapes vanilla,french-vanilla --all-sets "
              f"--sample-size-per-tier 100000 --t4-control 0 "
              f"--no-preflight --card-seed 0", file=sys.stderr)
        return 2

    rows = [json.loads(l) for l in args.manifest.read_text().splitlines() if l.strip()]
    supported = [r for r in rows if r.get("supported")]

    build_verify_dir()

    staged = 0
    not_landed = 0
    with tempfile.TemporaryDirectory(prefix="verify_catalog_") as td:
        stage = Path(td)
        # verify_dir reads <dir>/manifest.jsonl and writes the report
        # there; point it at the hermetic staging dir so the real
        # run dirs are never touched.
        shutil.copyfile(args.manifest, stage / "manifest.jsonl")
        for r in supported:
            landed = args.cards_src / mod_ident(r["set"]) / f"{r['slug']}.rs"
            if not landed.exists():
                not_landed += 1
                continue
            # Exact layout verify_dir pairs by: {idx:03}_{slug}.rs.
            shutil.copyfile(landed, stage / f"{r['idx']:03d}_{r['slug']}.rs")
            staged += 1

        if staged == 0:
            print("no landed catalog cards mapped to manifest rows")
            return 2

        cmd = [str(VERIFY_DIR_BIN), "--dir", str(stage)]
        if not args.no_batch:
            cmd.append("--batch")
        print(f"verify_catalog: attesting {staged} landed card(s) "
              f"({not_landed} manifest rows not landed) …")
        subprocess.run(cmd, check=False)

        report_path = stage / "verify-report.jsonl"
        rep = [json.loads(l) for l in report_path.read_text().splitlines() if l.strip()]
        if args.report:
            shutil.copyfile(report_path, args.report)

    # Catalog-scoped tally: only rows we actually staged (landed +
    # mapped). NotGenerated here == manifest row we didn't stage.
    landed_rows = [x for x in rep if x["outcome"] != "not_generated"]
    by = {}
    fails = []
    for x in landed_rows:
        by[x["outcome"]] = by.get(x["outcome"], 0) + 1
        if x["outcome"] != "passed":
            fails.append(x)

    total = len(landed_rows)
    passed = by.get("passed", 0)
    print("\n=== verify_catalog summary (landed catalog only) ===")
    print(f"  landed & attested : {total}")
    print(f"  passed            : {passed}"
          f"  ({100.0 * passed / total:.1f}%)" if total else "  passed: 0")
    for layer in ("layer1_failed", "layer2_failed", "layer3_failed"):
        if by.get(layer):
            print(f"  {layer:<18}: {by[layer]}")
    if fails:
        print("\n  FAILING LANDED CARDS (catalog is not clean):")
        for x in sorted(fails, key=lambda x: x["idx"])[:50]:
            print(f"    [{x['outcome']:<13}] {x['idx']:>4} {x['slug']}")
        if len(fails) > 50:
            print(f"    … and {len(fails) - 50} more")
        return 1

    print("\n  catalog clean — every landed, manifest-mapped card passes "
          "strict 3-layer verify.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
