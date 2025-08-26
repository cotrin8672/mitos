# Implementation Plan

## プロジェクト基盤とテストフレームワーク

- [x] 1. Rustプロジェクトのセットアップと依存関係
  - Cargo.tomlに本体依存関係を追加（ratatui, crossterm, portable-pty, git2, tokio, thiserror, serde, toml）
  - Cargo.tomlにテスト依存関係を追加（mockall, proptest, rstest, assert_matches, tempfile）
  - src/main.rsに基本的なアプリケーションエントリポイントを作成
  - src/lib.rsでモジュール構成を定義
  - tests/ディレクトリ構造を作成（unit/, integration/, e2e/）
  - _Requirements: 全要件の基盤セットアップ_

- [x] 2. エラーハンドリングのテストと実装
  - tests/unit/error_test.rsでMitosErrorのテストケースを作成
  - src/error.rsにMitosError型を実装（thiserror使用）
  - エラー変換のテストケースを追加
  - Result型エイリアスのテストと実装
  - _Requirements: 全要件のエラーハンドリング基盤_

- [x] 3. 共通型定義のテストと実装
  - tests/unit/types_test.rsでPaneId, Size, Positionのテストを作成
  - src/types.rsに共通型を実装
  - 境界値テスト（最小サイズ5x10）を追加
  - プロパティベーステスト（proptest）でバリデーション確認
  - _Requirements: 1.5_

## ペインデータモデル（TDD）

- [x] 4. Layoutデータモデルのテストと実装
  - tests/unit/layout_test.rsでLayout列挙型のテストを作成
  - src/layout.rsにLayout列挙型（Single/HSplit/VSplit）を実装
  - レイアウト計算アルゴリズムのテスト
  - 分割比率の境界値テスト
  - _Requirements: 1.1, 1.2, 1.3_

- [x] 5. Paneデータモデルのテストと実装
  - tests/unit/pane_test.rsでPane構造体のテストを作成
  - src/pane.rsにPane構造体とPaneTitle列挙型を実装
  - ペインサイズ制約のテスト（最小5x10）
  - タイトル切り詰めロジックのテスト
  - _Requirements: 1.5, 2.5_

## MVP Core 実装（最小限動作版）

- [x] 6. PTY基本実装
  - src/pty/mod.rsにPtyHandler traitを定義
  - Windowsならportable-ptyのConPTY、Unix系ならUnix PTYを直接実装
  - 基本的なコマンド実行とI/O処理のみ
  - _Requirements: 4.1, 4.2, 4.3_

- [x] 7. 基本的なペインマネージャー
  - src/pane_manager.rsを実装
  - ペイン作成・削除・フォーカス管理を一括実装
  - 最小限のレイアウト管理（Single/HSplit/VSplitのみ）
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 1.7_

- [x] 8. TUI基本実装
  - src/ui/mod.rsでratatuiのセットアップ
  - crosstermでrawモード・マウス・キーボード処理
  - 基本的な画面レイアウト（ペイン描画とフッター）
  - _Requirements: 5.1, 5.2_

- [x] 9. ペインビューレンダリング
  - src/ui/pane_view.rsでペイン内容の表示
  - PTY出力をバッファリングして表示
  - 基本的なスクロール処理
  - _Requirements: 1.1, 2.1_

- [ ] 10. イベントループとキーバインディング
  - src/event_manager.rsでイベント処理
  - Ctrl+h/v: 分割、Ctrl+w: 削除、矢印キー: 移動
  - 非同期でPTY出力とユーザー入力を処理
  - _Requirements: 1.4, 3.2_

## MVP統合

- [ ] 11. メインアプリケーション統合
  - src/main.rsでアプリケーション起動
  - 状態管理とコンポーネント間の連携
  - 基本的なエラーハンドリング
  - パニック時のクリーンアップ処理
  - _Requirements: 全要件のMVP統合_

## 拡張機能（MVP後）

- [ ] 12. 基本的なプロセス監視
  - src/process_monitor.rsで現在のプロセス情報取得
  - ペインタイトルにプロセス名表示
  - _Requirements: 2.1, 2.2_

- [ ] 13. Git Worktree基本機能
  - src/git/mod.rsでWorktreeリスト取得
  - フッターにWorktree情報表示
  - _Requirements: 3.1, 3.7_

- [ ] 14. リサイズ処理
  - ウィンドウリサイズ時のレイアウト調整
  - 最小サイズ保証
  - _Requirements: 1.6_

- [ ] 15. 基本的なパフォーマンス最適化
  - 差分レンダリング
  - PTY出力バッファリング
  - _Requirements: 5.1, 5.3_