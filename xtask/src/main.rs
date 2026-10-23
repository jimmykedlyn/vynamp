mod agents;
mod branding;
mod catalog;
mod ci;
mod github;
mod package;
mod release;
mod repo;
mod signing;
mod util;
mod validate;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
#[derive(Parser)]
#[command(version, about = "Vynamp development automation — Rust only")]
struct Cli {
    #[arg(long, global = true)]
    root: Option<PathBuf>,
    #[command(subcommand)]
    command: Action,
}
#[derive(Clone, Copy, ValueEnum)]
enum Agent {
    Claude,
}
impl Agent {
    fn as_str(self) -> &'static str {
        match self {
            Self::Claude => "claude",
        }
    }
}
#[derive(Clone, Copy, ValueEnum)]
enum Owner {
    Claude,
}
impl Owner {
    fn as_str(self) -> &'static str {
        match self {
            Self::Claude => "claude",
        }
    }
}
#[derive(Clone, Copy, ValueEnum)]
enum Kind {
    Feature,
    Fix,
    Chore,
    Hotfix,
}
impl Kind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Feature => "feature",
            Self::Fix => "fix",
            Self::Chore => "chore",
            Self::Hotfix => "hotfix",
        }
    }
}
#[derive(Subcommand)]
enum Action {
    Doctor {
        #[arg(long, value_enum, default_value = "claude")]
        agent: Agent,
    },
    Install {
        #[arg(long, value_enum, default_value = "claude")]
        agent: Agent,
    },
    #[command(name = "check-docs")]
    CheckDocs,
    #[command(name = "check-delivery")]
    CheckDelivery,
    #[command(name = "guard-branch")]
    GuardBranch,
    #[command(name = "github-import")]
    GithubImport {
        #[arg(long)]
        destination: PathBuf,
        #[arg(long)]
        push: bool,
    },
    #[command(name = "quality-gate")]
    QualityGate,
    #[command(name = "tag-version")]
    TagVersion,
    Repo {
        #[command(subcommand)]
        command: Repo,
    },
    Ci {
        #[command(subcommand)]
        command: Ci,
    },
    Release {
        #[command(subcommand)]
        command: Release,
    },
    Package {
        #[arg(long,value_parser=["windows-x64","linux-x64","macos-universal","android-arm64"])]
        platform: String,
        #[arg(long)]
        version: String,
    },
    #[command(name = "bundle-spec")]
    BundleSpec {
        #[arg(long)]
        output: PathBuf,
    },
}
#[derive(Subcommand)]
enum Repo {
    Init,
    Worktree {
        #[arg(long, value_enum)]
        agent: Owner,
        #[arg(long)]
        task: String,
        #[arg(long)]
        slug: String,
        #[arg(long, value_enum, default_value = "feature")]
        kind: Kind,
    },
    #[command(name = "remove-worktree")]
    RemoveWorktree {
        #[arg(long, value_enum)]
        agent: Owner,
        #[arg(long)]
        task: String,
    },
    Protect {
        #[arg(long)]
        repo: String,
        #[arg(long,default_value_t=1,value_parser=clap::value_parser!(u8).range(0..=2))]
        reviews: u8,
    },
    /// Read-only report of privacy, default branch, merge methods and protections.
    Audit {
        #[arg(long)]
        repo: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}
#[derive(Subcommand)]
enum Ci {
    Docs,
    Detect,
    Core {
        #[arg(long)]
        target: String,
    },
    Android,
    Gate,
}
#[derive(Subcommand)]
enum Release {
    Preflight {
        #[arg(long)]
        version: String,
    },
    Digest,
    Build {
        #[arg(long)]
        version: String,
        #[arg(long,value_parser=["windows-x64","linux-x64","macos-universal","android-arm64"])]
        platform: String,
    },
    Assemble {
        #[arg(long)]
        version: String,
    },
    #[command(name = "publish-draft")]
    PublishDraft {
        #[arg(long)]
        version: String,
    },
}
fn root(cli: &Cli) -> Result<PathBuf> {
    if let Some(p) = &cli.root {
        return Ok(p.canonicalize()?);
    }
    let cwd = std::env::current_dir()?;
    cwd.ancestors()
        .find(|p| p.join("xtask/Cargo.toml").is_file())
        .map(PathBuf::from)
        .context("Run from Vynamp project or provide --root")
}
fn execute() -> Result<()> {
    let cli = Cli::parse();
    let root = root(&cli)?;
    match cli.command {
        Action::Doctor { agent } => agents::doctor(&root, agent.as_str()),
        Action::Install { agent } => agents::install(&root, agent.as_str()),
        Action::CheckDocs => validate::spec(&root),
        Action::CheckDelivery => validate::delivery(&root),
        Action::GuardBranch => repo::guard(&root),
        Action::GithubImport { destination, push } => github::import(&root, &destination, push),
        Action::QualityGate => ci::quality(&root),
        Action::TagVersion => release::tag_version(),
        Action::Repo { command } => match command {
            Repo::Init => repo::init(&root),
            Repo::Worktree {
                agent,
                task,
                slug,
                kind,
            } => repo::worktree(&root, agent.as_str(), &task, &slug, kind.as_str()),
            Repo::RemoveWorktree { agent, task } => repo::remove(&root, agent.as_str(), &task),
            Repo::Protect { repo, reviews } => repo::policy(&root, &repo, reviews),
            Repo::Audit { repo, out } => repo::audit(&root, &repo, out.as_deref()),
        },
        Action::Ci { command } => match command {
            Ci::Docs => ci::docs(&root),
            Ci::Detect => ci::emit_detect(&root),
            Ci::Core { target } => ci::core(&root, &target),
            Ci::Android => ci::android(&root),
            Ci::Gate => ci::gate(),
        },
        Action::Release { command } => match command {
            Release::Digest => release::command(&root, "digest", None, None),
            Release::Preflight { version } => {
                release::command(&root, "preflight", Some(&version), None)
            }
            Release::Build { version, platform } => {
                release::command(&root, "build", Some(&version), Some(&platform))
            }
            Release::Assemble { version } => {
                release::command(&root, "assemble", Some(&version), None)
            }
            Release::PublishDraft { version } => {
                release::command(&root, "publish-draft", Some(&version), None)
            }
        },
        Action::Package { platform, version } => package::build(&root, &platform, &version),
        Action::BundleSpec { output } => bundle(&root, &output),
    }
}
fn bundle(root: &std::path::Path, output: &std::path::Path) -> Result<()> {
    use serde_json::json;
    use std::{fs, io::Write};
    const DIRECTORY: &str = "vynamp-media-player";
    let cfg = util::read_json(root.join("config/project.json"))?;
    let wrappers = [
        ("INICIAR.ps1", "INICIAR.ps1"),
        ("AMBIENTE.ps1", "AMBIENTE.ps1"),
        ("ABRIR-CLAUDE.ps1", "ABRIR-CLAUDE.ps1"),
        ("PROMPT-CLAUDE.txt", "PROMPT-CLAUDE.txt"),
        ("LEIA-ME.txt", "LEIA-ME.md"),
    ];
    let mut entries = util::files(root)?
        .into_iter()
        .filter(|p| p.file_name().is_none_or(|n| n != "ARTIFACTS.json"))
        .map(|p| {
            let rel = p
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            (format!("pacote/{rel}"), p)
        })
        .collect::<Vec<_>>();
    for (source, name) in wrappers {
        let path = root.join("bootstrap").join(source);
        anyhow::ensure!(path.is_file(), "Bootstrap template absent: {source}");
        entries.push((name.to_owned(), path));
    }
    let rows = entries
        .iter()
        .map(|(name, path)| {
            Ok(json!({"path":name,"bytes":fs::metadata(path)?.len(),"sha256":util::sha(path)?}))
        })
        .collect::<Result<Vec<_>>>()?;
    util::write_json(
        root.join("ARTIFACTS.json"),
        &json!({"spec_version":cfg["spec_version"],"root_directory":DIRECTORY,"source_directory":"pacote","files":rows}),
    )?;
    entries.push((
        "pacote/ARTIFACTS.json".to_owned(),
        root.join("ARTIFACTS.json"),
    ));
    let out = if output.is_absolute() {
        output.to_path_buf()
    } else {
        root.join(output)
    };
    let canonical_root = root.canonicalize()?;
    let parent = out
        .parent()
        .context("Bundle output parent")?
        .canonicalize()?;
    anyhow::ensure!(
        !parent.starts_with(&canonical_root),
        "Bundle output must be outside the source package; use its parent project directory"
    );
    let mut archive = zip::ZipWriter::new(fs::File::create(&out)?);
    for (name, path) in entries {
        let mut options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        if path.parent() == Some(root.join(".githooks").as_path()) || path.ends_with("AppRun") {
            options = options.unix_permissions(0o755);
        }
        archive.start_file(format!("{DIRECTORY}/{name}"), options)?;
        archive.write_all(&fs::read(&path)?)?;
    }
    archive.finish()?;
    fs::write(
        format!("{}.sha256", out.display()),
        format!(
            "{}  {}\n",
            util::sha(&out)?,
            out.file_name().unwrap().to_string_lossy()
        ),
    )?;
    println!("{}", out.display());
    Ok(())
}
fn main() {
    if let Err(e) = execute() {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}
