# LMStudio bake-off launcher

YAML-driven thin wrapper over the bake-off binary, for the Mac
single-machine workflow against LMStudio's OpenAI-compatible server.
Same binary, same outputs, same analyzer — just a Python
orchestration layer that reads `config.yaml` instead of building
CLI flags by hand.

## Files

- `config.yaml` — LMStudio endpoint + model lineup + bake-off
  parameters. Edit this for every run.
- `pyproject.toml` — `uv`-managed env (`openai`, `pyyaml`).
- `smoke_test.py` — pre-flight: connectivity, models loaded,
  latency, seed honoring. Run before any real bake-off.
- `run_bakeoff.py` — reads `config.yaml`, invokes the bake-off
  binary, prints the JSONL path on success.

## One-time setup

1. Install LMStudio (https://lmstudio.ai), open the **Local Server**
   tab, load the models named in `config.yaml`, click **Start
   Server**. Default endpoint is `http://localhost:1234`.

2. From the workspace root:
   ```
   cargo build --release -p arcana-gen --bin bakeoff
   ```

3. From this directory:
   ```
   uv sync
   ```

## Run

```
uv run python smoke_test.py        # pre-flight
uv run python run_bakeoff.py       # full bake-off
```

`smoke_test.py` exits non-zero if a configured model isn't loaded
or the endpoint is unreachable. Don't proceed to `run_bakeoff.py`
until smoke passes.

## Config knobs

`config.yaml` sections:

| Section          | Field                  | Notes                                                                 |
| ---------------- | ---------------------- | --------------------------------------------------------------------- |
| `lmstudio`       | `base_url`             | LMStudio's `/v1` endpoint. Default `http://localhost:1234/v1`.        |
|                  | `api_key`              | Placeholder; LMStudio ignores. Required by the OpenAI Python SDK.     |
|                  | `models`               | Repeatable list. Each becomes a `--openai-model` flag.                |
|                  | `extra_body`           | Optional. JSON object merged into every request. Use for Qwen3 thinking-mode toggle. |
| `bakeoff`        | `binary`               | Relative or absolute path to the compiled bake-off binary.            |
|                  | `sample_size_per_tier` | Cards/tier this run; 30 is a good default.                            |
|                  | `max_attempts`         | Per (card, model) retry budget. 3 by default.                         |
|                  | `tiers`                | List form: `[1, 2, 3]`.                                               |
|                  | `t4_control`           | T4 sanity-anchor sample. Catches classifier leaks.                    |
|                  | `card_seed`            | Hold constant across replicate runs.                                  |
|                  | `model_seed`           | Vary across replicate runs for independent trials of the same cards. |
|                  | `output_dir`           | JSONL goes here. Auto-created.                                        |
|                  | `no_preflight`         | Skip the engine pre-sweep (useful when you've already verified).      |
| `smoke`          | `prompt`               | One-word reply check.                                                 |
|                  | `max_tokens`           | Cap for the smoke prompt.                                             |
|                  | `seed`                 | For the seed-honoring check.                                          |

## Replication

The Mac path runs one inference at a time (until you profile and
decide otherwise). Replicate by running `run_bakeoff.py` multiple
times with different `model_seed` values:

```yaml
# in config.yaml:
bakeoff:
  card_seed: 0       # constant
  model_seed: 0      # bump each run: 0, 1, 2, ...
```

Output files are seed-tagged
(`lmstudio_<timestamp>_card<C>_model<M>.jsonl`) so they don't
collide. Aggregate with:

```
cat ../../../target/bakeoff-runs/lmstudio/*.jsonl > combined.jsonl
cargo run -p arcana-gen --bin bakeoff_analyze --release -- combined.jsonl
```

(See the analyzer caveat about sharded data in `README.oscar.md` —
the same applies here once you start aggregating multiple runs.)

## Thinking-mode toggle (Qwen3 etc.)

Uncomment the `lmstudio.extra_body` block in `config.yaml`:

```yaml
lmstudio:
  extra_body:
    chat_template_kwargs:
      enable_thinking: false
```

The Python layer JSON-encodes this and forwards via
`--openai-extra-body`; the Rust client merges it into every
`/v1/chat/completions` request body at the top level. Whether the
toggle takes effect depends on the underlying LMStudio engine and
the model's chat template — verify by inspecting a smoke-test
response's content for the absence/presence of a thinking block.
