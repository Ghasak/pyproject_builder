use anyhow::{Context, Result};
use dialoguer::Input;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Find the system Python version or return a default.
fn detect_system_python() -> String {
    let candidate = which::which("python3")
        .or_else(|_| which::which("python"))
        .ok();
    if let Some(bin) = candidate {
        if let Ok(out) = Command::new(bin)
            .arg("-c")
            .arg("import sys;print('.'.join(map(str, sys.version_info[:3])))")
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }
    "3.11.0".to_string()
}

/// Write a string or formatted string to a file, creating parent dirs.
fn write<P: AsRef<Path>>(path: P, content: impl AsRef<[u8]>) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    f.write_all(content.as_ref())?;
    Ok(())
}

fn main() -> Result<()> {
    println!(
        r#"
  _ \ _ \   _ \     | __|   __| __ __|
  __/   /  (   | \  | _|   (       |
 _|  _|_\ \___/ \__/ ___| \___|   _|

  _ )  |  | _ _|  |     _ \  __|  _ \
  _ \  |  |   |   |     |  | _|     /
 ___/ \__/  ___| ____| ___/ ___| _|_\

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  🐍  PY-PROJ • Minimal uv + VS Code project scaffolder      ┃
┃  ⚙️  Venv, VS Code, Pyright, Ruff, Pytest, PyRefly, Jupyter ┃
┃  📦  Batteries included — zero cruft, zero fuss             ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#
    );

    // ---- prompt defaults ----
    let cwd = env::current_dir()?;
    let default_proj = format!("{}_proj", cwd.file_name().unwrap().to_string_lossy());
    let default_py = detect_system_python();

    let project: String = Input::new()
        .with_prompt("Project name")
        .default(default_proj.clone())
        .interact_text()?;

    let py_full: String = Input::new()
        .with_prompt("Python version (e.g. 3.13.5)")
        .default(default_py.clone())
        .interact_text()?;

    let mm = py_full.split('.').take(2).collect::<Vec<_>>().join(".");
    let mm_nodec = mm.replace('.', "");

    // ---- directory structure ----
    let root = cwd.join(&project);
    fs::create_dir_all(root.join("src"))?;
    fs::create_dir_all(root.join("tests"))?;
    fs::create_dir_all(root.join("Notebooks"))?;
    fs::create_dir_all(root.join(".vscode"))?;

    write(root.join("src/__init__.py"), "")?;
    write(
        root.join("src/main.py"),
        r#"def main() -> None:
    print("Hello from src.main!")

if __name__ == "__main__":
    main()
"#,
    )?;

    // ---- VS Code configs ----
    write(
        root.join(".vscode/launch.json"),
        r#"{
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
}"#,
    )?;

    write(
        root.join(".vscode/settings.json"),
        r#"{
  "python.defaultInterpreterPath": "${workspaceFolder}/.venv/bin/python",
  "python.terminal.activateEnvironment": true,
  "python.analysis.extraPaths": [
    "${workspaceFolder}",
    "${workspaceFolder}/src",
    "${workspaceFolder}/Notebooks"
  ],
  "python.envFile": "${workspaceFolder}/.env",
  "jupyter.envFile": "${workspaceFolder}/.env",
  "[python]": {
    "editor.defaultFormatter": "ms-python.black-formatter",
    "editor.formatOnSave": true
  },
  "black-formatter.importStrategy": "fromEnvironment",
  "black-formatter.path": ["${workspaceFolder}/.venv/bin/black"],
  "black-formatter.args": ["--line-length", "100"],
  "notebook.defaultFormatter": "ms-python.black-formatter"
}"#,
    )?;

    write(
        root.join(".vscode/tasks.json"),
        r#"{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run (uv): src.main",
      "type": "shell",
      "command": "uv run python -m src.main",
      "options": { "cwd": "${workspaceFolder}", "env": { "PYTHONPATH": "${workspaceFolder}" } },
      "problemMatcher": []
    }
  ]
}"#,
    )?;

    // ---- env files ----
    write(
        root.join(".env"),
        "PYTHONPATH=.:./src:./Notebooks\nENV=dev\n",
    )?;
    write(
        root.join(".envrc"),
        r#"export PYTHONPATH="${PYTHONPATH}:$PWD:$PWD/src:$PWD/Notebooks"
if [ -f ./.env ]; then
  set -a
  . ./.env
  set +a
fi
"#,
    )?;

    // ---- pyrefly.toml ----
    write(
        root.join("pyrefly.toml"),
        &format!(
            r#"[project]
name = "{project}"
python = "{py_full}"

[paths]
src = "src"
notebooks = "Notebooks"
venv = ".venv"
env = ".env"

[imports]
import_roots = ["src"]

[lint]
enable = ["ruff"]
format = ["black"]

[test]
runner = "pytest"
coverage = true
"#
        ),
    )?;

    // ---- pyrightconfig.json ----
    write(
        root.join("pyrightconfig.json"),
        &format!(
            r#"{{
  "pythonVersion": "{mm}",
  "pythonPlatform": "Darwin",
  "typeCheckingMode": "basic",
  "reportMissingImports": "warning",
  "useLibraryCodeForTypes": true,
  "include": [".", "src/"],
  "exclude": ["**/__pycache__", ".venv"],
  "venvPath": ".",
  "venv": ".venv",
  "executionEnvironments": [
    {{
      "root": ".",
      "extraPaths": [
        "./src",
        "./Notebooks/",
        ".venv/lib/python{mm}/site-packages"
      ]
    }}
  ]
}}"#
        ),
    )?;

    // ---- pyproject.toml ----
    write(
        root.join("pyproject.toml"),
        &format!(
            r#"[project]
name = "{project}"
version = "0.1.0"
description = "Minimal project template"
readme = "README.md"
requires-python = ">={mm}"
authors = [{{ name = "Your Name" }}]
dependencies = []

[tool.uv]

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

[tool.ruff]
line-length = 100
target-version = "py{mm_nodec}"
extend-exclude = [".venv"]
fix = true
"#,
            project = project,
            mm = mm,
            mm_nodec = mm_nodec
        ),
    )?;

    // ---- gitignore ----
    write(
        root.join(".gitignore"),
        r#".venv/
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
"#,
    )?;

    // ---- README ----
    write(
        root.join("README.md"),
        &format!(
            r#"# {project}

Generated by PY-PROJ scaffolder.

## Setup

```bash
cd {project}
direnv allow     # or: source .venv/bin/activate
uv pip install -e ".[dev]"
```

## Running

```bash
uv run python -m src.main
```

## Development

```bash
# Format code
uvx black .

# Lint code
uvx ruff check --fix

# Run tests
uv run pytest

# Type checking
uvx pyright
```

## Structure

- `src/` - Main source code
- `tests/` - Test files
- `Notebooks/` - Jupyter notebooks
- `.vscode/` - VS Code configuration
- `pyproject.toml` - Project configuration
- `pyrefly.toml` - Custom project metadata
"#,
            project = project
        ),
    )?;

    // ---- uv env creation ----
    println!("⚙️  Installing Python {py_full} via uv …");
    Command::new("uv")
        .args(["python", "install", &py_full])
        .current_dir(&root)
        .status()
        .context("uv python install failed")?;

    println!("🧪 Creating uv venv …");
    Command::new("uv")
        .args(["venv", "--python", &py_full, ".venv"])
        .current_dir(&root)
        .status()
        .context("uv venv failed")?;

    println!("\n🎉 Project `{}` created in {:?}\n", project, root);
    println!(
    "Next:\n  cd {}\n  direnv allow   # or: source .venv/bin/activate\n  uv pip install -e \".[dev]\"\n  uv run python -m src.main\n  uvx ruff check --fix\n",
    project
);

    Ok(())
}
