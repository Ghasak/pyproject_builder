# Project Logging

Perfect—here’s a tight, “day-to-day dev” playbook so you (and future you) use your isolated logging without thinking about it.

## 0) One-time setup (already done)

- Package is renamed to `app_logging` (no stdlib shadowing).
- `config07.json` lives in `src/app_logging/` (no `"root"` section, `propagate:false`, `only_src` filter).
- `setup_logging()` in `src/app_logging/glogger.py` loads that config and starts the `QueueListener`.

## 1) How to write code in any module

At the **top of every module under `src/…`** where you want logs:

```python
import logging
logger = logging.getLogger(__name__)
```

Use it anywhere:

```python
logger.info("starting step X")
logger.debug("payload=%s", payload)          # avoid f-strings in logs; let logging format lazily
logger.warning("retrying…")
logger.exception("failed to process item")   # inside except, adds traceback
```

## 2) How to start your app (entrypoint pattern)

Make sure your entrypoint (CLI, script, web server, task) calls `setup_logging()` **once** at the beginning:

```python
from src.app_logging.glogger import setup_logging
import logging

def main():
    setup_logging()
    logger = logging.getLogger("src.app")
    logger.info("App booted")
    # … your app …

if __name__ == "__main__":
    main()
```

Run:

```bash
uv run -m src.main
```

## 3) Dev vs. Prod knobs (use env vars)

Let’s give you quick control without editing code.

In `glogger.setup_logging()`, optionally read envs:

```python
import os

LEVEL = os.getenv("APP_LOG_LEVEL", "INFO")         # DEBUG/INFO/WARNING…
LOGFILE = os.getenv("APP_LOG_FILE", str(HERE / "project_log_file.log"))

# after json.load:
config["loggers"]["src"]["level"] = LEVEL
for hname in ("stdout", "stderr", "file_json"):
    if hname in config["handlers"]:
        if hname == "file_json":
            config["handlers"][hname]["filename"] = LOGFILE
```

Now you can do:

```bash
APP_LOG_LEVEL=DEBUG uv run -m src.main
APP_LOG_FILE=/tmp/metalgear.log uv run -m src.main
```

## 4) Where the logs go (and what you should see)

- **Console (stdout/stderr)** → your **colored** formatter for `src.*`.
- **File** `src/app_logging/project_log_file.log` → **JSON** records for `src.*`.
- **Libraries/root** → unaffected (their own formatting/handlers).
  If you want to see them while debugging only, add once in your dev script _after_ `setup_logging()`:

  ```python
  import logging
  if not logging.getLogger().handlers:
      logging.basicConfig(level=logging.WARNING)  # dev-only
  ```

## 5) Adding structured fields (super useful in dev)

You can attach arbitrary fields with `extra=…`; your JSON formatter already includes non-builtin attributes:

```python
logger.info("user logged in", extra={"user_id": 42, "tenant": "jp-east"})
```

Check `project_log_file.log`—you’ll see `"user_id": 42, "tenant": "jp-east"`.

## 6) Per-file quick pattern (copy/paste)

```python
# src/feature/worker.py
import logging
logger = logging.getLogger(__name__)

def run_job(job_id: str):
    logger.info("job received", extra={"job_id": job_id})
    try:
        1/0
    except ZeroDivisionError:
        logger.exception("job crashed", extra={"job_id": job_id})
```

## 7) Using inside Airflow (safe)

- **Do not** configure logging at DAG import time.
- In task callables only:

  ```python
  def my_task():
      from src.app_logging.glogger import setup_logging
      setup_logging()
      log = logging.getLogger("src.airflow.task")
      log.info("task start")
  ```

- Your lines appear in task logs (captured stdout) with your colorized format, while Airflow’s own logs keep their formatting.

## 8) Unit tests (pytest) with `caplog`

```python
# tests/test_worker.py
import logging
from src.app_logging.glogger import setup_logging
from src.feature.worker import run_job

def test_run_job_logs(caplog):
    setup_logging()
    with caplog.at_level(logging.INFO, logger="src"):
        run_job("123")
    msgs = [r.message for r in caplog.records if r.name.startswith("src.")]
    assert any("job received" in m for m in msgs)
```

## 9) Common “oops” (avoid these)

- ❌ Calling `setup_logging()` inside library modules (keep it in entrypoints/tasks only).
- ❌ Using `basicConfig()` anywhere in your project (it touches root).
- ❌ Logging from files **outside** `src/…` (they won’t be in your namespace and won’t hit your handlers).
- ❌ Re-adding a `"root"` section to your JSON.

## 10) Quick health checks

- **Namespace isolation**

  ```bash
  rg -n "getLogger\(" src | wc -l       # you’re using loggers per module
  ```

- **File log contains only your records**

  ```bash
  tail -n +1 src/app_logging/project_log_file.log | jq '.name' | sort -u
  # Should list only names beginning with "src."
  ```

- **Stdlib really is stdlib**

  ```bash
  uv run python -c "import logging; print(logging.__file__)"
  # path should point to Python’s stdlib, not your project
  ```

That’s it. Day-to-day, you just:

1. call `setup_logging()` at startup,
2. write `logger = logging.getLogger(__name__)` in each module, and
3. log normally.

Everything else (colors/JSON, isolation from libraries/Airflow, rotation) happens automatically.
