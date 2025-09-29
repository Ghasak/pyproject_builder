use anyhow::{bail, Context, Result};
use clap::{ArgAction, Parser};
use owo_colors::OwoColorize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod scaffold;
mod templates;
mod util;

use scaffold::ScaffoldPlan;
use util::detect_system_python;

/// Fancy banner shown in --help
const BANNER: &str = r#"
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
"#;

#[derive(Parser, Debug)]
#[command(
    name = "py-proj",
    disable_help_flag = true,
    disable_version_flag = true,
    about = "Scaffold a minimal Python project (uv + VS Code) with logging package.",
    long_about = None
)]
struct Cli {
    /// Create a new project (non-interactive)
    #[arg(long = "create_project", action = ArgAction::SetTrue)]
    create_project: bool,

    /// Clean build/test caches under the project
    #[arg(long = "clean_project", action = ArgAction::SetTrue)]
    clean_project: bool,

    /// Delete (nuke) the entire project directory (requires --yes)
    #[arg(long = "delete_project", action = ArgAction::SetTrue)]
    delete_project: bool,

    /// Project name (default: <cwd_basename>_proj)
    #[arg(long, short = 'p')]
    project: Option<String>,

    /// Python version to install via uv (default: auto-detected)
    #[arg(long = "python", short = 'P')]
    py_full: Option<String>,

    /// Output directory; default: $PWD/<project>
    #[arg(long = "outdir")]
    outdir: Option<PathBuf>,

    /// Auto-confirm dangerous actions like --delete_project
    #[arg(long = "yes", short = 'y', action = ArgAction::SetTrue)]
    yes: bool,

    /// Show help with banner and color
    #[arg(long = "help", short = 'h', action = ArgAction::SetTrue)]
    help: bool,

    /// Show version
    #[arg(long = "version", short = 'V', action = ArgAction::SetTrue)]
    version: bool,
}

#[allow(clippy::print_literal)]
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("{} {}", "py-proj".bold(), env!("CARGO_PKG_VERSION").green());
        return Ok(());
    }

    // If help is requested or no primary action was provided, show help and exit.
    let no_action = !(cli.create_project || cli.clean_project || cli.delete_project);
    if cli.help || no_action {
        print_help();
        return Ok(());
    }

    // Resolve defaults
    let cwd = env::current_dir()?;
    let default_proj = format!(
        "{}_proj",
        cwd.file_name().unwrap_or_default().to_string_lossy()
    );
    let project = cli.project.unwrap_or(default_proj);
    let root = cli.outdir.unwrap_or_else(|| cwd.join(&project));
    let py_full = cli.py_full.unwrap_or_else(detect_system_python);

    // Derived versions used in templates
    let mm = py_full.split('.').take(2).collect::<Vec<_>>().join(".");
    let mm_nodec = mm.replace('.', "");

    if cli.create_project {
        println!("{} {}", ">>".cyan().bold(), "Create project".bold());
        println!("  {} {}", "Project:".dimmed(), project.blue().bold());
        println!(
            "  {} {}",
            "Root:   ".dimmed(),
            root.display().to_string().blue()
        );
        println!("  {} {}", "Python: ".dimmed(), py_full.magenta());

        create_project(&root, &project, &py_full, &mm, &mm_nodec)?;
        println!("{} {}", "OK".green().bold(), "Project created.");
    }

    if cli.clean_project {
        println!("{} {}", ">>".cyan().bold(), "Clean project caches".bold());
        clean_project(&root)?;
        println!("{} {}", "OK".green().bold(), "Project cleaned.");
    }

    if cli.delete_project {
        println!("{} {}", ">>".cyan().bold(), "Delete project (NUKE)".bold());
        if !cli.yes {
            bail!(
                "{} Use {} to confirm deletion.",
                "Refusing to delete without confirmation.".yellow(),
                "--yes".bold()
            );
        }
        delete_project(&root)?;
        println!("{} {}", "OK".green().bold(), "Project deleted.");
    }

    Ok(())
}
fn print_help() {
    use owo_colors::OwoColorize as _;

    // Keep the ASCII banner exactly as-is
    print!("{BANNER}");

    let cmd = "py-proj";

    // USAGE
    println!("{}", "USAGE".bold().underline());
    println!("  {}", format!("{cmd} [FLAGS] [OPTIONS]").italic());
    println!();

    // EXAMPLE
    println!("{}", "EXAMPLE".bold().underline());
    println!(
        "  {}",
        format!("{cmd} --create_project --project myproj --python 3.13.1").cyan()
    );
    println!();

    // FLAGS
    println!("{}", "FLAGS".bold());
    println!(
        "  {}  {}",
        "🆕  --create_project".green().bold(),
        "Create a new project in the target directory.".dimmed()
    );
    println!(
        "  {}  {}",
        "🧹  --clean_project".yellow().bold(),
        "Remove caches: .venv, __pycache__, .pytest_cache, .ruff_cache, etc.".dimmed()
    );
    println!(
        "  {}  {}",
        "💣  --delete_project".red().bold(),
        "Delete the entire project directory (requires --yes).".dimmed()
    );
    println!(
        "  {}  {}",
        "✅  -y, --yes".green().bold(),
        "Auto-confirm dangerous actions (e.g., delete).".dimmed()
    );
    println!(
        "  {}  {}",
        "❓  -h, --help".bold(),
        "Show this help.".dimmed()
    );
    println!(
        "  {}  {}",
        "🏷️  -V, --version".bold(),
        "Show version.".dimmed()
    );
    println!();

    // OPTIONS
    println!("{}", "OPTIONS".bold());
    println!(
        "  {}  {}",
        "📦  -p, --project <NAME>".bold(),
        "Project name (default: <cwd>_proj).".dimmed()
    );
    println!(
        "  {}  {}",
        "🐍  -P, --python <VER>".bold(),
        "Python version for uv (default: auto-detected).".dimmed()
    );
    println!(
        "  {}  {}",
        "📁  --outdir <PATH>".bold(),
        "Output directory (default: $PWD/<project>).".dimmed()
    );
    println!();

    // TIP
    println!("{}", "💡  TIP".bold());
    println!("  {}", "After creating the project, run".dimmed());
    println!("    {}", "`uv pip install -e \".[dev]\"`".bold());
    println!("  {}", "then".dimmed());
    println!("    {}", "`uv run python -m src.main`".bold());
}
/// Create the project using the existing scaffolder plan (non-interactive).
fn create_project(
    root: &Path,
    project: &str,
    py_full: &str,
    mm: &str,
    mm_nodec: &str,
) -> Result<()> {
    // Ensure directories (same layout you had, plus app_logging)
    for d in ["src", "tests", "Notebooks", ".vscode", "src/app_logging"] {
        fs::create_dir_all(root.join(d))?;
    }

    let plan = ScaffoldPlan {
        root: root.to_path_buf(),
        project: project.to_string(),
        py_full: py_full.to_string(),
        mm: mm.to_string(),
        mm_nodec: mm_nodec.to_string(),
    };

    plan.write_basic_src()?;
    plan.write_vscode()?;
    plan.write_envs()?;
    plan.write_pyrefly()?;
    plan.write_pyright()?;
    plan.write_pyproject()?;
    plan.write_gitignore()?;
    plan.write_readme()?;
    plan.write_app_logging()?; // include your logging package
    plan.install_uv_toolchain()?; // uv python install + venv
    plan.wirte_makefile()?; // wirte the makefile

    Ok(())
}

/// Remove common build/test caches under the project.
fn clean_project(root: &Path) -> Result<()> {
    use std::fs::{remove_dir_all, remove_file};

    let dirs = [
        ".venv",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".ruff_cache",
        ".ipynb_checkpoints",
        "build",
        "dist",
        "htmlcov",
        ".coverage", // sometimes a file
        ".cache",
        "src/__pycache__",
        "tests/__pycache__",
        "Notebooks/.ipynb_checkpoints",
    ];

    for rel in dirs {
        let p = root.join(rel);
        if p.is_dir() {
            println!(
                "  {} {}",
                "rm -rf".yellow(),
                p.display().to_string().dimmed()
            );
            let _ = remove_dir_all(&p);
        } else if p.is_file() && rel == ".coverage" {
            println!("  {} {}", "rm".yellow(), p.display().to_string().dimmed());
            let _ = remove_file(&p);
        }
    }
    Ok(())
}

/// Delete the entire project directory (dangerous).
#[allow(clippy::print_literal)]
fn delete_project(root: &Path) -> Result<()> {
    if root.exists() {
        println!("  {} {}", "rm -rf".red().bold(), root.display());
        fs::remove_dir_all(root).with_context(|| format!("Failed to delete {}", root.display()))?;
    } else {
        println!("  {} {}", "SKIP".dimmed(), "Project root does not exist.");
    }
    Ok(())
}
