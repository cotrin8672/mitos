mod error;
mod git;

use clap::{Parser, Subcommand, Args};
use crate::error::MitosResult;
use crate::git::{list_worktrees, create_worktree, delete_worktree};

/// Mitos - A Git Worktree Manager with integrated terminal
///
/// Git worktreeを簡単に管理し、統合ターミナルで
/// シームレスに作業できるCLIツールです。
#[derive(Parser, Debug)]
#[command(name = "mitos")]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short = 'd')]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List git worktrees
    List,

    /// Create a git worktree for a branch
    Create(CreateArgs),

    /// Delete a git worktree by path or name
    Delete(DeleteArgs),
}

#[derive(Args, Debug)]
struct CreateArgs {
    /// Target branch name. If missing, create new branch at HEAD.
    branch: String,

    /// Target path for the worktree directory (default: sanitized branch)
    #[arg(long, short)]
    path: Option<std::path::PathBuf>,
}

#[derive(Args, Debug)]
struct DeleteArgs {
    /// Worktree path or directory name
    target: String,

    /// Force removal of the worktree
    #[arg(long, short)]
    force: bool,
}

fn main() -> MitosResult<()> {
    // clapを使ってコマンドライン引数をパース
    let args = Cli::parse();
    
    // デバッグフラグが立っていたら、引数の詳細を表示
    if args.debug {
        println!("=== デバッグモード: 引数の詳細 ===");
        println!("{:#?}", args);
        println!("=================================");
        
        // プログラムの引数を直接確認（学習用）
        println!("\n実際のコマンドライン引数:");
        for (index, arg) in std::env::args().enumerate() {
            println!("  argv[{}]: {:?}", index, arg);
        }
    }

    match args.command {
        Some(Commands::List) => {
            let list = list_worktrees()?;
            if list.is_empty() {
                println!("No worktrees found.");
            } else {
                for wt in list {
                    match wt.branch {
                        Some(b) => println!("{}\t{}\t(branch: {})", wt.name, wt.path.display(), b),
                        None => println!("{}\t{}", wt.name, wt.path.display()),
                    }
                }
            }
        }
        Some(Commands::Create(CreateArgs { branch, path })) => {
            let created_path = create_worktree(&branch, path.as_deref())?;
            println!("Created worktree at {} for branch '{}'.", created_path.display(), branch);
        }
        Some(Commands::Delete(DeleteArgs { target, force })) => {
            delete_worktree(&target, force)?;
            println!("Removed worktree '{}'.", target);
        }
        None => {
            println!("Hello, Mitos!");
        }
    }
    Ok(())
}
