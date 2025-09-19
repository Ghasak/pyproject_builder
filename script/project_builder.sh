#!/usr/bin/env bash
set -euo pipefail

# --- Inputs -------------------------------------------------------------
read -rp "Project name (kebab-or_snake): " PROJ
read -rp "Python version (e.g., 3.13.5): " PYV_FULL

# Derive MAJOR.MINOR (e.g., 3.13) from full version (e.g., 3.13.5)
PYV_MM="$(echo "$PYV_FULL" | awk -F. '{print $1"."$2}')"

# --- Create project root ------------------------------------------------
mkdir -p "$PROJ"
cd "$PROJ"

# --- Basic layout -------------------------------------------------------
mkdir -p src/"$PROJ" Notebooks .vscode
touch src/"$PROJ"/__init__.py

# --- Create uv virtual env ---------------------------------------------
# Ensure the exact interpreter is available to uv
uv python install "$PYV_FULL" >/dev/null
# Create venv pinned to that interpreter
uv venv --python "$PYV_FULL" .venv

# --- Activate the env (note: this only affects THIS script's process) ---
# The user will see how to activate after script finishes as well.
# shellcheck disable=SC1091
source .venv/bin/activate

# --- Minimal app entrypoint ---------------------------------------------
cat > src/main.py <<'PY'
def main() -> None:
    print("Hello from src.main!")

if __name__ == "__main__":
    main()
PY

# --- VS Code: launch.json ----------------------------------------------
cat > .vscode/launch.json <<'JSON'
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Python: Current file",
      "type": "debugpy",
      "request": "launch",
      "program": "${file}",
      "cwd": "${workspaceFolder}",
      "env": {
        "PYTHONPATH": "${workspaceFolder}:${workspaceFolder}/src:${workspaceFolder}/Notebooks"
      },
      "console": "integratedTerminal",
      "justMyCode": true,
      "subProcess": true
    },
    {
      "name": "Python: Module src.main",
      "type": "debugpy",
      "request": "launch",
      "module": "src.main",
      "cwd": "${workspaceFolder}",
      "env": {
        "PYTHONPATH": "${workspaceFolder}:${workspaceFolder}/src:${workspaceFolder}/Notebooks"
      },
      "console": "integratedTerminal",
      "justMyCode": true,
      "subProcess": true
    }
  ]
}
JSON

# --- VS Code: settings.json --------------------------------------------
cat > .vscode/settings.json <<'JSON'
{
  // --- keep your UI/editor prefs above ---

  // Use the project's venv everywhere (adjust if different)
  "python.defaultInterpreterPath": "${workspaceFolder}/.venv/bin/python",
  "python.terminal.activateEnvironment": true,

  // Make imports like `from src.ch01...` resolve in editors & tools
  "python.analysis.extraPaths": [
    "${workspaceFolder}",
    "${workspaceFolder}/src",
    "${workspaceFolder}/Notebooks"
  ],

  // Ensure both Python extension and Jupyter load your .env
  "python.envFile": "${workspaceFolder}/.env",
  "jupyter.envFile": "${workspaceFolder}/.env",

  // (Optional) pick your formatters
  "[python]": {
    "editor.defaultFormatter": "ms-python.black-formatter",
    "editor.formatOnSave": true
  },
  "black-formatter.importStrategy": "fromEnvironment",
  "black-formatter.path": ["${workspaceFolder}/.venv/bin/black"],
  "black-formatter.args": ["--line-length", "100"],
  "notebook.defaultFormatter": "ms-python.black-formatter"
}
JSON

# --- VS Code: tasks.json -----------------------------------------------
cat > .vscode/tasks.json <<'JSON'
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run (uv): src.main",
      "type": "shell",
      "command": "uv run python -m src.main",
      "options": {
        "cwd": "${workspaceFolder}",
        "env": { "PYTHONPATH": "${workspaceFolder}" }
      },
      "problemMatcher": []
    }
  ]
}
JSON

# --- Pyright config (version/OS-specific) -------------------------------
cat > pyrightconfig.json <<JSON
{
  "pythonVersion": "${PYV_MM}",
  "pythonPlatform": "Darwin",
  "typeCheckingMode": "basic",
  "reportMissingImports": "warning",
  "useLibraryCodeForTypes": true,
  "include": [".", "src/"],
  "exclude": ["**/__pycache__", ".venv"],

  "venvPath": ".",
  "venv": ".venv",

  "executionEnvironments": [
    {
      "root": ".",
      "extraPaths": [
        "./src",
        "./Notebooks/",
        ".venv/lib/python${PYV_MM}/site-packages"
      ]
    }
  ]
}
JSON

# --- pyrefly.toml (minimal; adjust to your needs) ----------------------
cat > pyrefly.toml <<TOML
[project]
name = "${PROJ}"
python = "${PYV_FULL}"

[paths]
src = "src"
notebooks = "Notebooks"
venv = ".venv"
env = ".env"

[lint]
enable = ["ruff"]
format = ["black"]

[test]
runner = "pytest"
coverage = true
TOML

# --- .env --------------------------------------------------------------
cat > .env <<ENV
# App defaults
ENV=dev
PYTHONPATH=${PWD}:${PWD}/src:${PWD}/Notebooks
ENV

# --- .gitignore --------------------------------------------------------
cat > .gitignore <<'GI'
.venv/
__pycache__/
*.pyc
.env
.ipynb_checkpoints/
.coverage
.htmlcov/
.mypy_cache/
.pytest_cache/
dist/
build/
GI

# --- pyproject.toml with dev/test/prod tooling -------------------------
cat > pyproject.toml <<TOML
[project]
name = "${PROJ}"
version = "0.1.0"
description = "Minimal project template generated by new_pyproj.sh"
readme = "README.md"
requires-python = ">=${PYV_MM}"
authors = [{ name = "Your Name" }]

# runtime deps go here (empty by default)
dependencies = []

[tool.uv]
# ensure uv respects local venv
index-strategy = "eager"

[tool.uv.sources]
# (optional) add custom indexes here

[tool.uv.pip]
# (optional) pip compatibility flags

# --- Dev/QA tools ------------------------------------------------------
[project.optional-dependencies]
dev = [
  "ruff>=0.6.0",
  "black>=24.0.0",
  "pyright>=1.1.380",
  "pytest>=8.0.0",
  "pytest-cov>=5.0.0",
  "ipykernel>=6.0.0",
  "rich>=13.0.0"
]
test = [
  "pytest>=8.0.0",
  "pytest-cov>=5.0.0"
]
prod = [
  # add production-only extras if any
]

# Ruff config
[tool.ruff]
line-length = 100
target-version = "py${PYV_MM/./}"
extend-exclude = [".venv"]
fix = true

[tool.ruff.lint]
select = ["E","F","I","W","UP","B","C90"]
ignore = []

[tool.ruff.format]
quote-style = "double"
indent-style = "space"

# Black config (kept simple; VS Code points Black to venv)
[tool.black]
line-length = 100
target-version = ["py${PYV_MM/./}"]

# Pytest config
[tool.pytest.ini_options]
addopts = "-ra -q --cov=src --cov-report=term-missing"
testpaths = ["tests"]

# Coverage.py config
[tool.coverage.run]
source = ["src"]
branch = true

[tool.coverage.report]
fail_under = 0
show_missing = true

TOML

# --- README ------------------------------------------------------------
cat > README.md <<MD
# ${PROJ}

Generated with \`new_pyproj.sh\`.

## Quickstart

\`\`\`bash
source .venv/bin/activate
uv pip install -e ".[dev]"
uv run python -m src.main
\`\`\`

### Lint / Format
- Lint: \`uvx ruff check\`
- Fix : \`uvx ruff check --fix\`
- Format: \`uv run black .\`

### VS Code
- Debug "Python: Module src.main"
- Task "Run (uv): src.main"

MD

# --- Done message ------------------------------------------------------
cat <<MSG

✅ Project scaffolded at: $(pwd)

Next steps:
1) Activate env:    source .venv/bin/activate
2) Install dev deps: uv pip install -e ".[dev]"
3) Run app:         uv run python -m src.main
4) Ruff quickfix:   uvx ruff check --fix

(Also available: \`uvx ruff\` and \`uvx ruff check --fix\`)

MSG

