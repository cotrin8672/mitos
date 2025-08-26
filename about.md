# mitos — 要件まとめ（コンパクト）

## コンセプト

* `git worktree` の作成・選択を簡略化し、対象ディレクトリで **単一ペインのシェル** と **AIエージェント**を即起動するワンストップツール。
* **TUI**（直観操作）と **CLI**（慣れた人向け）の **両インターフェース**を提供。
* **非目標**：ターミナル分割や多重管理は既存の tmux / wezterm / zellij に委譲。

## スコープ

* 単一ペインの TUI（分割はしない）。
* `git worktree` の生成・列挙・選択・整理（remove/prune など）。
* 対象ディレクトリをカレントにした **PTY シェル起動** と **エージェント起動**。

## ユースケース（例示）

* 新しい作業ブランチの worktree を作成 → その場でシェルを開き → AI エージェントを起動。
* 既存の worktree を選択して開き、すぐ作業を再開。

## 技術選定（Rust）

* **UI描画**：Ratatui（backend: crossterm）
* **端末制御**：crossterm（Raw/AlternateScreen、イベント、リサイズ）
* **PTY**：portable-pty（Linux/macOS/Windows）
* **ANSI パース**：vt100（制御シーケンスを解釈してスクリーン状態更新）
* **CLI**：clap
* **設定**：TOML（ユーザ設定・エージェント定義・フック）
* **その他**：anyhow（エラー処理）、tracing（ログ）

## アーキテクチャ（単一ペイン）

```
mitos::git  ── worktree 操作（list/create/open/remove/prune）
     │
     └─ 対象 path 決定 ──▶ mitos::pty（portable-pty） ─▶ PTY 出力
                                   │
                              vte::Parser
                                   │
                          mitos::screen（内部スクリーン）
                                   │
                           mitos::tui（Ratatui 描画）
```

## 機能要件（MVP）

* **Worktree 管理**：一覧／作成／開く／削除／整理（prune 相当）。
* **TUI**：

  * PTY シェル起動（対象ディレクトリを CWD に反映）
  * キー入力 → PTY へ転送、リサイズ伝搬
  * 枠・タイトルなど最小限の表示
* **AI エージェント起動**：

  * 設定に基づくコマンドを PTY へ投入し、対象ディレクトリ上で起動できる。
* **CLI 連携**：

  * TUI 起動や worktree 操作をコマンドラインから実行できる（※具体的なオプション設計は別途）。
* **設定**：

  * 既定シェル、初期化コマンド、エージェント定義、パス規約等を TOML で定義可能。

## 追加要件（拡張フロー）

### A. Worktree 作成時のフック

* **順序**：worktree 生成 → **シンボリックリンク作成** → **セットアップコマンド実行** →（必要に応じて）TUI/エージェント起動。
* **失敗時**：該当ステップで中断。リンク作成失敗時は作成済みリンクを削除。セットアップ失敗時は状態をログに残す。

### B. シンボリックリンク作成

* 宣言的なリンク定義（`src` → `dst`、worktree 相対推奨）。
* 既存 `dst` の扱いポリシー（エラー／スキップ／置換）を選べるようにする。
* **クロスプラットフォーム**：

  * Linux/macOS：`std::os::unix::fs::symlink`
  * Windows：`std::os::windows::fs::{symlink_file, symlink_dir}`。権限不足時はジャンクション等のフォールバック検討。
* 原子性・安全性：一時名で作成→rename、バックアップ（`*.bak`）など。

### C. セットアップコマンド実行

* CWD = worktree。環境変数（`MITOS_REPO_ROOT` / `MITOS_WORKTREE_PATH` / `MITOS_BRANCH` など）を提供可能。
* 引数は配列で安全に扱い、stdout/stderr をログに記録。エラーコードで中断。
* 再実行性（セットアップのみ再試行）を見据えた設計。

## 非機能要件

* **クロスプラットフォーム**：Linux/macOS/Windows（ConPTY）
* **安定性**：raw 解除・画面復帰の確実化、詳細ログ、障害時の明確なエラーメッセージ
* **安全性**：破壊的操作は確認必須、コマンド注入対策（配列引数・必要箇所のみクォート）
* **拡張性**：エージェントやフック、プロジェクト別設定（リポジトリ内設定）に対応しやすい構造

## モジュール構成（案）

* `mitos::cli`：CLI 入口（※詳細設計は別ドキュメント）
* `mitos::git`：worktree 操作（porcelain パース）
* `mitos::pty`：spawn/resize/read/write（portable-pty）
* `mitos::screen`：vte の結果保持（スクリーン）
* `mitos::tui`：Ratatui 描画ループ
* `mitos::agent`：設定 → コマンド解決・起動
* `mitos::config`：TOML ロード/検証

## 開発順序（MVP → 拡張）

1. git ラッパ最小（list/create/open）
2. 単一ペイン TUI（PTY + vte + Ratatui）
3. エージェント起動（設定連携）
4. remove/prune と安全確認、ログ強化
5. フック実装（リンク作成／セットアップ）
6. 属性・スクロールバックなど端末機能の拡張
