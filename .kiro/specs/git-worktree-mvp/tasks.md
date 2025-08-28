# Implementation Plan - 学習向け段階的実装

## Phase 1: 基礎構築とHello World

- [x] 1.1 Cargo プロジェクトの作成
  - `cargo new mitos`でプロジェクト作成
  - Cargo.tomlに最小限の依存（clap, thiserror）を追加
  - `cargo run`で動作確認
  - _Requirements: プロジェクト基盤_

- [x] 1.2 最初のCLIコマンド実装
  - src/main.rsでclapを使った`mitos --version`実装
  - printlnデバッグで引数を表示
  - `cargo run -- --help`でヘルプ表示確認
  - _Requirements: 要件4.5_

- [x] 1.3 エラー型の基礎実装
  - src/error.rsファイル作成
  - thiserrorで最小限のMitosError enum定義
  - main.rsからエラー型を使用してみる
  - _Requirements: 要件5.1_

## Phase 2: Git操作の基礎

- [x] 2.1 git2クレートの導入と実験
  - Cargo.tomlにgit2を追加
  - main.rsで現在のリポジトリを開く実験コード
  - printlnでリポジトリパスを表示
  - _Requirements: Git操作の学習_

- [x] 2.2 worktree一覧表示の実装
  - src/git/mod.rsとworktree.rs作成
  - list_worktrees()関数の実装（シンプル版）
  - main.rsから呼び出してprintlnで表示
  - _Requirements: 要件1.1_

- [x] 2.3 CLIサブコマンド`mitos list`の実装
  - clapでlistサブコマンド追加
  - Git関数を呼び出して整形表示
  - エラーハンドリングの追加
  - _Requirements: 要件4.1_

## Phase 3: Git worktree操作の完成

- [x] 3.1 worktree作成機能の実装
  - create_worktree()関数の実装
  - エラーケース（既存パス）の処理
  - デバッグ用のprintln追加
  - _Requirements: 要件1.2, 1.3_

- [x] 3.2 worktree削除機能の実装
  - remove_worktree()関数の実装  
  - 確認プロンプトの追加
  - ロック状態のチェック
  - _Requirements: 要件1.4, 1.5, 1.6_

- [x] 3.3 CLIサブコマンドの統合
  - `mitos create <branch>`実装
  - `mitos remove <path>`実装
  - 全コマンドの動作確認
  - _Requirements: 要件4.2, 4.3_

## Phase 4: PTY基礎実験

- [ ] 4.1 portable-ptyの導入と最小実験
  - Cargo.tomlにportable-pty追加
  - 単純なシェル起動プログラムを別ファイルで作成
  - 標準入出力の確認（同期版）
  - _Requirements: PTY理解_

- [ ] 4.2 PTYモジュールの基礎実装
  - src/pty/mod.rsとsession.rs作成
  - PtySession構造体の定義
  - spawn_shell()の同期版実装
  - _Requirements: 要件2.1_

- [ ] 4.3 簡易対話プログラムの作成
  - キー入力を受け取ってPTYに送信
  - PTY出力を画面に表示
  - Ctrl+Dで終了する仕組み
  - _Requirements: 要件2.2, 2.3, 2.5_

## Phase 5: 非同期処理の導入

- [ ] 5.1 tokioランタイムの基礎
  - Cargo.tomlにtokio追加（rt, macros feature）
  - main関数を`#[tokio::main]`に変更
  - 簡単な非同期関数の実験
  - _Requirements: 非同期基盤_

- [ ] 5.2 PTY非同期I/Oの実装
  - PTY読み書きを非同期化
  - tokio::spawnで並行処理
  - デバッグログの追加
  - _Requirements: 要件2.2, 2.3_

- [ ] 5.3 `mitos enter`コマンドの基礎実装
  - enterサブコマンドの追加
  - worktreeパスでシェル起動
  - 基本的な入出力（エスケープシーケンス無し）
  - _Requirements: 要件4.4, 2.1_

## Phase 6: VT100パーサー導入

- [ ] 6.1 vt100クレートの実験
  - Cargo.tomlにvt100追加
  - 単独テストプログラムでパーサー実験
  - エスケープシーケンスのprintlnデバッグ
  - _Requirements: VT100理解_

- [ ] 6.2 ターミナルモジュールの基礎
  - src/terminal/mod.rsとparser.rs作成
  - VT100Handler構造体の実装
  - PTY出力をパーサーに通す
  - _Requirements: 要件3.1_

- [ ] 6.3 パース結果のデバッグ表示
  - スクリーン状態をprintlnで可視化
  - カーソル位置の追跡
  - 色情報の確認
  - _Requirements: デバッグスキル向上_

## Phase 7: TUI基礎（Ratatui導入）

- [ ] 7.1 Ratatuiの最小実験
  - Cargo.tomlにratatui, crossterm追加
  - Hello Worldを表示する単独プログラム
  - Raw modeの入り方/出方を学習
  - _Requirements: TUI基礎理解_

- [ ] 7.2 画面描画の基礎実装
  - src/terminal/emulator.rs作成
  - TerminalEmulator構造体の定義
  - 単純なテキスト描画
  - _Requirements: 要件3.2_

- [ ] 7.3 イベントループの実装
  - crossterm::eventでキー入力受付
  - qキーで終了する仕組み
  - 画面更新の基本ループ
  - _Requirements: 要件3.5_

## Phase 8: 統合とブラッシュアップ

- [ ] 8.1 VT100とRatatuiの統合
  - パース済みスクリーンをRatatuiで描画
  - カーソル表示の実装
  - 色・スタイルの反映
  - _Requirements: 要件3.1, 3.2_

- [ ] 8.2 リサイズ対応
  - crossterm::event::Event::Resize処理
  - PTYへのサイズ伝播
  - 画面再描画
  - _Requirements: 要件2.4, 3.7_

- [ ] 8.3 エラー処理とパニック対策
  - Drop traitで端末復元
  - Result型の適切な使用
  - ユーザーフレンドリーなエラーメッセージ
  - _Requirements: 要件5.1, 5.2, 5.3_

## Phase 9: テストとデバッグ技術

- [ ] 9.1 単体テストの作成
  - git操作のテスト（tempfileで仮リポジトリ）
  - エラー型のテスト
  - モックを使ったテスト手法
  - _Requirements: テストスキル_

- [ ] 9.2 統合テストの作成
  - tests/ディレクトリにE2Eテスト
  - assert_cmdでCLIテスト
  - 出力の検証方法
  - _Requirements: 要件4 - CLI全体_

- [ ] 9.3 デバッグ技術の実践
  - dbg!マクロの活用
  - 環境変数でデバッグモード切り替え
  - ログ出力の追加（eprintln）
  - _Requirements: デバッグスキル向上_

## Phase 10: 最終調整

- [ ] 10.1 パフォーマンス確認
  - 不要なクローンの削除
  - バッファサイズの調整
  - 描画頻度の最適化
  - _Requirements: 実装品質向上_

- [ ] 10.2 ドキュメント追加
  - README.mdの作成
  - 主要関数のdocコメント
  - 使用方法の記載
  - _Requirements: 保守性向上_

- [ ] 10.3 リリースビルドと配布
  - `cargo build --release`
  - バイナリサイズの確認
  - 各プラットフォームでの動作確認
  - _Requirements: 完成_

## 学習のポイント

各フェーズで学べること：
- **Phase 1-2**: Rustの基礎、CLIツール作成の基本
- **Phase 3-4**: 外部ライブラリ統合、システムプログラミング
- **Phase 5-6**: 非同期処理、ターミナル制御の仕組み
- **Phase 7-8**: TUI開発、イベント駆動プログラミング
- **Phase 9-10**: テスト技法、デバッグ手法、最適化

推奨作業時間：
- 各小タスク: 30分〜1時間
- 各Phase: 2〜4時間
- 全体: 20〜40時間（学習ペースによる）