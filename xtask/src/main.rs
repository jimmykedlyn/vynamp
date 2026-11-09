mod agents;
mod branding;
mod catalog;
mod ci;
mod github;
mod package;
mod release;
mod repo;
mod secrets;
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
    #[command(name = "check-tasks")]
    CheckTasks,
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
    #[command(name = "check-secrets")]
    CheckSecrets,
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
        Action::CheckTasks => validate::tasks(&root),
        Action::GuardBranch => repo::guard(&root),
        Action::GithubImport { destination, push } => github::import(&root, &destination, push),
        Action::QualityGate => ci::quality(&root),
        Action::CheckSecrets => secrets::check(&root),
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
    }
}
fn main() {
    if let Err(e) = execute() {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}
