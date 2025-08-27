mod error;

use clap::Parser;
use crate::error::MitosResult;

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

    println!("Hello, Mitos!");
    Ok(())
}
