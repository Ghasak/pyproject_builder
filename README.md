# PY-PROJ

```
  _ \ _ \   _ \     | __|   __| __ __|
  __/   /  (   | \  | _|   (       |
 _|  _|_\ \___/ \__/ ___| \___|   _|

  _ )  |  | _ _|  |     _ \  __|  _ \
  _ \  |  |   |   |     |  | _|     /
 ___/ \__/  ___| ____| ___/ ___| _|_\
```

**PY-PROJ** is a fast, minimal Python project scaffolder written in Rust. It
creates modern Python projects with all the essentials: `uv` for package
management, VS Code configuration, type checking with Pyright, linting with
Ruff, testing with Pytest, and Jupyter notebook support.

## Features

🐍 **Modern Python toolchain** - Uses `uv` for fast, reliable package management
⚙️ **Complete VS Code setup** - Debugging, formatting, linting, and IntelliSense configured
🔧 **Developer tools** - Ruff, Black, Pyright, Pytest with coverage
📊 **Jupyter ready** - Notebooks folder with proper PYTHONPATH configuration
🎯 **Zero configuration** - Everything works out of the box
📦 **Batteries included** - All dev dependencies and configs pre-configured

## Installation

### From source (recommended)

```bash
git clone <your-repo-url>
cd py-proj
cargo install --path .
```

### Prerequisites

- [Rust](https://rustup.rs/) (for building)
- [uv](https://docs.astral.sh/uv/getting-started/installation/) (for Python project management)
- Python 3.8+ installed on your system

## Usage

Simply run `pyproj` in any directory:

```bash

󰚩 INSERT 󰇌 on    main !1 ?1
╰─ ln -s ~/gCliHub/pyproject_builder/target/release/pyproj /usr/local/bin/pyproject_builder


pyproj
```

The tool will prompt you for:
- **Project name** (defaults to `{current_dir}_proj`)
- **Python version** (auto-detects your system Python)

## What gets created

### Directory structure
```
my_project/
├── src/
│   ├── __init__.py
│   └── main.py              # Entry point with hello world
├── tests/                   # Test directory
├── Notebooks/               # Jupyter notebooks
├── .vscode/                 # VS Code configuration
│   ├── launch.json          # Debug configurations
│   ├── settings.json        # Python and formatter settings
│   └── tasks.json           # Build tasks
├── .venv/                   # Virtual environment (created by uv)
├── pyproject.toml           # Project configuration
├── pyrightconfig.json       # Type checker configuration
├── pyrefly.toml            # Custom metadata
├── .env                     # Environment variables
├── .envrc                   # direnv configuration
├── .gitignore              # Python gitignore
└── README.md               # Project documentation
```

### Configured tools

- **uv** - Package manager and virtual environment
- **Ruff** - Lightning-fast linter and formatter
- **Black** - Code formatter (100 character line length)
- **Pyright** - Static type checker
- **Pytest** - Testing framework with coverage
- **Jupyter** - Notebook support with proper PYTHONPATH

### VS Code integration

- Python interpreter automatically set to `.venv/bin/python`
- Debugging configurations for current file and main module
- Format on save enabled
- Proper PYTHONPATH for imports from `src/` and `Notebooks/`
- Environment variables loaded from `.env`

## Quick start after project creation

```bash
cd my_project

# Activate environment (choose one)
direnv allow                    # if using direnv
source .venv/bin/activate      # manual activation

# Install development dependencies
uv pip install -e ".[dev]"

# Run the project
uv run python -m src.main

# Development workflow
uvx ruff check --fix           # Lint and fix
uvx black .                    # Format code
uv run pytest                  # Run tests
uvx pyright                    # Type checking
```

## Dependencies

The scaffolder creates projects with these development dependencies:

- **ruff** >= 0.6.0 - Linting and formatting
- **black** >= 24.0.0 - Code formatting
- **pyright** >= 1.1.380 - Type checking
- **pytest** >= 8.0.0 - Testing framework
- **pytest-cov** >= 5.0.0 - Coverage reporting
- **ipykernel** >= 6.0.0 - Jupyter kernel support
- **rich** >= 13.0.0 - Pretty terminal output

## Configuration files explained

- **`pyproject.toml`** - Modern Python project configuration
- **`pyrightconfig.json`** - Type checker settings with proper paths
- **`pyrefly.toml`** - Custom project metadata for future tooling
- **`.env`** - Environment variables (PYTHONPATH, ENV=dev)
- **`.envrc`** - direnv configuration for auto-activation
- **`.vscode/`** - Complete VS Code workspace setup

## Why PY-PROJ?

- **Fast**: Written in Rust, creates projects in seconds
- **Modern**: Uses latest Python tooling (uv, Ruff, Pyright)
- **Complete**: Everything configured, nothing left to set up
- **Opinionated**: Sensible defaults that work for most projects
- **Minimal**: No cruft, just what you need to be productive

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with `cargo test`
5. Submit a pull request

## License

MIT License - see LICENSE file for details
