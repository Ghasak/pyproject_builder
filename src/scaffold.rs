use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::templates::*;
use crate::util::{run, write};

pub struct ScaffoldPlan {
    pub root: PathBuf,
    pub project: String,
    pub py_full: String,
    pub mm: String,
    pub mm_nodec: String,
}

impl ScaffoldPlan {
    pub fn write_basic_src(&self) -> Result<()> {
        write(self.root.join("src/__init__.py"), "")?;
        write(self.root.join("src/main.py"), main_py())?;
        Ok(())
    }

    pub fn write_vscode(&self) -> Result<()> {
        write(self.root.join(".vscode/launch.json"), vscode_launch_json())?;
        write(
            self.root.join(".vscode/settings.json"),
            vscode_settings_json(),
        )?;
        write(self.root.join(".vscode/tasks.json"), vscode_tasks_json())?;
        Ok(())
    }

    pub fn write_envs(&self) -> Result<()> {
        write(self.root.join(".env"), dotenv())?;
        write(self.root.join(".envrc"), envrc())?;
        Ok(())
    }

    pub fn write_pyrefly(&self) -> Result<()> {
        write(
            self.root.join("pyrefly.toml"),
            pyrefly_toml(&self.project, &self.py_full),
        )?;
        Ok(())
    }

    pub fn write_pyright(&self) -> Result<()> {
        write(
            self.root.join("pyrightconfig.json"),
            pyrightconfig_json(&self.mm),
        )?;
        Ok(())
    }

    pub fn write_pyproject(&self) -> Result<()> {
        write(
            self.root.join("pyproject.toml"),
            pyproject_toml(&self.project, &self.mm, &self.mm_nodec),
        )?;
        Ok(())
    }

    pub fn write_gitignore(&self) -> Result<()> {
        write(self.root.join(".gitignore"), gitignore())?;
        Ok(())
    }

    pub fn write_readme(&self) -> Result<()> {
        write(self.root.join("README.md"), readme_md(&self.project))?;
        Ok(())
    }
    pub fn wirte_makefile(&self) -> Result<()> {
        write(
            self.root.join("Makefile"),
            app_make_file_creator(),
        )?;

        Ok(())
    }

    /// NEW: create the `src/app_logging` package with all files you asked for.
    pub fn write_app_logging(&self) -> Result<()> {
        let base = self.root.join("src/app_logging");
        write(base.join("__init__.py"), "")?;
        write(
            base.join("MyColoredFormatter.py"),
            app_logging_my_colored_formatter_py(),
        )?;
        write(base.join("config07.json"), app_logging_config07_json())?;
        write(base.join("constants.py"), app_logging_constants_py())?;
        write(base.join("glogger.py"), app_logging_glogger_py())?;
        write(
            base.join("myCustomJsonClass01.py"),
            app_logging_my_custom_json_class01_py(),
        )?;
        write(base.join("myFilters.py"), app_logging_my_filters_py())?;
        Ok(())
    }

    pub fn install_uv_toolchain(&self) -> Result<()> {
        println!("⚙️  Installing Python {} via uv …", self.py_full);
        run(
            "uv",
            &["python", "install", &self.py_full],
            Path::new(&self.root),
        )?;

        println!("🧪 Creating uv venv …");
        run(
            "uv",
            &["venv", "--python", &self.py_full, ".venv"],
            Path::new(&self.root),
        )?;
        Ok(())
    }
}
