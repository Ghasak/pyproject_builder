# Project Logging

Short answer: **yes—use `logging.getLogger(__name__)` in every Python module** (including `src/main.py`). That’s the idiomatic pattern and it plays perfectly with the config you have.

Here’s why and how it fits your setup:

- Your config routes all logs from the **`src` logger and its descendants** into the queue via the `queue_handler` attached to `"src"`, and then fans them out through handlers on `"src.sink"`.
- The `only_src` filter (`logging.Filter(name="src")`) allows **`src` and everything under it** (e.g., `src.main`, `src.app_logging.glogger`, `src.feature.x`).
- When you do `logging.getLogger(__name__)` inside a module under `src/`, that logger name will be e.g. `src.main` or `src.app_logging.glogger`, which:
  - passes the `only_src` filter,
  - bubbles (propagates) up to `src` (since you didn’t define those child loggers explicitly), and
  - gets handled by the queue handler on `src` and then by the sink handlers on `src.sink`.

So you **do not** need to hardcode `"src"` everywhere; `__name__` is better.

---

### What about the entry point (`main_py()`)—can I use `__name__` instead of `src.app` (or `src`)?

Yes. In `src/main.py`, when you run with `python -m src.main` (as your scaffold does), `__name__` is `"src.main"`, which is a descendant of `"src"`. That’s exactly what you want.

Example `main.py` that uses your logging stack:

```python
from src.app_logging.glogger import setup_logging
import logging

log = logging.getLogger(__name__)  # -> "src.main"

def main() -> None:
    setup_logging()
    log.debug("debug message (goes to stdout via sink)")
    log.info("info message")
    log.warning("warning -> stderr")
    log.error("error -> stderr")
    print("Hello from src.main!")

if __name__ == "__main__":
    main()
```

This will:

- enqueue each log via the `queue_handler` on `src`,
- have the `QueueListener` dispatch them to the handlers attached to `src.sink`,
- and the `only_src` filter will keep out non-`src` noise.

---

### When _not_ to use `__name__`

- If a script is **not** inside the `src` package and you run it directly (so `__name__ == "__main__"`), its logs **won’t** match the `only_src` filter. In that case either:
  - run it as a module under `src` (preferred): `python -m src.tools.my_script`, or
  - explicitly use `logging.getLogger("src.tools.my_script")`, or
  - relax/modify the `only_src` filter to cover your use case.

### Don’t redefine child loggers with `propagate = False`

Let module loggers use defaults (propagate=True). If you explicitly configure, say, `"src.main": {"propagate": false}`, you’ll **cut it off** from the queue on `"src"` unless you attach handlers there too.

### Keep the sink anchor as-is

Your `QueueListener` grabs handlers from `SINK_LOGGER = "src.sink"`. Leave that constant; it’s just the place where you mount stdout/stderr/file handlers. You don’t log **to** `src.sink`; you log to `src.*`, which flows to the sink via the queue.

---

## Quick checklist

- In **every** module under `src/`:

  ```python
  import logging
  log = logging.getLogger(__name__)
  ```

- In your entrypoint (`src/main.py`): use `__name__` as above and call `setup_logging()` once.
- Always run as a module (so names start with `src.`):
  `uv run python -m src.main`
- If you rename the top package (`src` → `acme`), update in one place:
  - `SINK_LOGGER = "acme.sink"`
  - `only_src` filter name in JSON (to `"acme"`)
  - the logger block names in JSON (`"acme"` and `"acme.sink"`)

That’s it—you’ll get clean, hierarchical log names, the color formatter, stdout/stderr split, and JSON file rotation, all wired through the queue listener.
