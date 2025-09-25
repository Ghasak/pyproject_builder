# pyproject_builder

— Minimal Python Project Scaffolder

> uv + VS Code/Cursor + Pyright + Ruff + Pytest + Jupyter + structured logging (queue + colored + json)

```
  _ \ _ \   _ \     | __|   __| __ __|
  __/   /  (   | \  | _|   (       |
 _|  _|_\ \___/ \__/ ___| \___|   _|

  _ )  |  | _ _|  |     _ \  __|  _ \
  _ \  |  |   |   |     |  | _|     /
 ___/ \__/  ___| ____| ___/ ___| _|_\

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  🐍  pyproject_builder • Minimal uv + VS Code project scaffolder      ┃
┃  ⚙️  Venv, VS Code, Pyright, Ruff, Pytest, PyRefly, Jupyter           ┃
┃  📦  Batteries included — zero cruft, zero fuss                       ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

## ✨ What it does

- Creates a Python project with:
  - `pyproject.toml` (uv-powered), `.env`, `.envrc`
  - VS Code configs (`.vscode/launch.json`, `settings.json`, `tasks.json`)
  - `src/`, `tests/`, `Notebooks/`
  - **Logging package** at `src/app_logging/`:
    - `MyColoredFormatter.py` (ANSI colored logs)
    - JSON formatter + rotating file handler
    - QueueHandler + QueueListener fan-out to stdout/stderr/file
    - `config07.json`, `glogger.py`, `constants.py`, `myFilters.py`, `myCustomJsonClass01.py`
- Installs the requested Python with **uv** and creates `.venv`

---

## 🧩 Requirements

- Rust toolchain (`cargo`)
- [`uv`](https://docs.astral.sh/uv/) available on PATH
  (e.g., `curl -Ls https://astral.sh/uv/install.sh | sh` or `pipx install uv`)
- macOS/Linux terminal that supports UTF-8 icons/ANSI colors (for pretty output)

---

## 📦 Install the CLI

From the repo root:

```bash
cargo build --release
# then put the binary on your PATH, e.g.:
cp target/release/pyproject_builder /usr/local/bin/
# or
cargo install --path .
```

Check it works:

```bash
pyproject_builder --version
pyproject_builder --help
```

---

## 🚀 Usage

### Flags & Options

| Flag / Option            | Meaning                                                                              |
| ------------------------ | ------------------------------------------------------------------------------------ |
| `--create_project`       | Create a new project (non-interactive).                                              |
| `--clean_project`        | Remove caches: `.venv`, `__pycache__`, `.pytest_cache`, `.ruff_cache`, etc.          |
| `--delete_project`       | **Delete the entire project directory** (requires `--yes`).                          |
| `-y`, `--yes`            | Auto-confirm dangerous actions (e.g., `--delete_project`).                           |
| `-h`, `--help`           | Show help (with ASCII banner).                                                       |
| `-V`, `--version`        | Show version.                                                                        |
| `-p`, `--project <NAME>` | Project name. Default: `<cwd>_proj`.                                                 |
| `-P`, `--python <VER>`   | Python version for **uv** (e.g., `3.13.1`). Default: auto-detected from your system. |
| `--outdir <PATH>`        | Output directory. Default: `$PWD/<project>`.                                         |

> Tip: If `--outdir` is omitted, the project is created inside the **current directory** under `<project>`.

---

## 🧪 Examples

1. Create a project with defaults (name = `<cwd>_proj`, Python auto-detected)

```bash
pyproject_builder --create_project
```

2. Create with an explicit name & Python version

```bash
pyproject_builder --create_project -p acme_ml -P 3.13.1
```

3. Create into a custom directory

```bash
pyproject_builder --create_project -p acme_ml --outdir ./sandbox/acme_ml
```

4. Clean caches for an existing project directory

```bash
pyproject_builder --clean_project --outdir ./sandbox/acme_ml
```

5. **Nuke** (delete) a project directory

```bash
pyproject_builder --delete_project --outdir ./sandbox/acme_ml --yes
```

6. Show help / version

```bash
pyproject_builder --help
pyproject_builder --version
```

---

## 📂 What gets generated

```
<project>/
├─ .env
├─ .envrc
├─ .gitignore
├─ .vscode/
│  ├─ launch.json
│  ├─ settings.json
│  └─ tasks.json
├─ Notebooks/
├─ pyproject.toml
├─ pyrefly.toml
├─ pyrightconfig.json
├─ README.md
├─ src/
│  ├─ __init__.py
│  ├─ main.py
│  └─ app_logging/
│     ├─ __init__.py
│     ├─ config07.json
│     ├─ constants.py
│     ├─ glogger.py
│     ├─ myCustomJsonClass01.py
│     ├─ myFilters.py
│     └─ MyColoredFormatter.py
└─ tests/
```

---

## ▶️ After creation

Inside the project folder:

```bash
direnv allow    # or: source .venv/bin/activate
uv pip install -e ".[dev]"
uv run python -m src.main
uvx ruff check --fix
uvx pyright
uv run pytest
```

---

## 🧾 Using the logging package

In `src/main.py`:

```python
import logging
from src.app_logging.glogger import setup_logging, PROJECT_LOGGER

log = logging.getLogger(__name__)  # -> "src.main"

def main() -> None:
    setup_logging()
    log.info("hello from src.main (INFO)")
    log.warning("warning to stderr")
    log.error("error to stderr")

if __name__ == "__main__":
    main()
```

- Logs from `src.*` flow to a queue (`QueueHandler`) attached to the `src` logger.
- A `QueueListener` forwards to `src.sink` handlers:
  - **stdout**: DEBUG/INFO, colored
  - **stderr**: WARNING/ERROR/CRITICAL, colored
  - **file_json**: rotating JSON at `src/app_logging/project_log_file.log`

---

## 🧰 Notes & Tips

- Always run modules under `src` (e.g., `python -m src.main`) so logger names start with `src.` and pass the `only_src` filter.
- If you rename the top package (`src` → `acme`), also update:
  - `SINK_LOGGER` and the `only_src` filter in `config07.json`
  - The logger blocks `"src"` / `"src.sink"` in `config07.json`

---

## 🐛 Troubleshooting

- **Clippy warning `print_literal`**: We avoid it in `--help` by styling strings (e.g., `.dimmed()`). If you add plain `println!("{}", "literal")`, Clippy will warn.
- **uv not found**: Ensure uv is on your PATH. Reopen your terminal after install.

---

## 📜 License

MIT (or your choice). Happy scaffolding! 🛠️🐍
