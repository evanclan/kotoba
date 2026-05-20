<h1 align="center">Kotoba (言葉)</h1>

<p align="center">
  <em>ローカルファースト・ターミナル中心・スクリプト可能な言語学習ツールキット。</em>
</p>

<p align="center">
  <em>ChatGPT は何でも一度教えてくれる。<br/>
  Kotoba は、あなたが学んだことを覚え、忘れる前に復習を促し、すでに使っているすべてのツールに溶け込む。</em>
</p>

<p align="center">
  <a href="#-クイックスタート"><strong>クイックスタート</strong></a> ·
  <a href="docs/architecture.md"><strong>アーキテクチャ</strong></a> ·
  <a href="docs/roadmap.md"><strong>ロードマップ</strong></a> ·
  <a href="CONTRIBUTING.md"><strong>コントリビュート</strong></a> ·
  <a href="README.md"><strong>English</strong></a>
</p>

---

## なぜ Kotoba？

多くの言語学習アプリは、あなたのデータ・スケジュール・サブスクリプションを「囲い込もう」とします。Kotoba は逆の発想で作られています。

- **データはあなたのフォルダにあるプレーンテキスト。** Markdown のデッキを Git で管理。年単位で語彙の成長を diff できる。友人のデッキを fork し、先生のカリキュラムに PR を送れる。
- **あなたが普段いる場所で動く。** ターミナル用 CLI、集中レビュー用 TUI、エディタ・ブラウザ拡張のためのデーモン、AI エージェント向けの MCP サーバ。Electron 不要・ログイン不要・テレメトリ無し。
- **AI は対等な相棒。** Ollama・Claude・OpenAI・OpenRouter — 好きな LLM を持ち込める。Kotoba は土台、AI はその上で動くツールの一つ。
- **オフライン・飛行機・社内 FW・Raspberry Pi、永久に動く。** 単一バイナリ、プレーンテキストのデータ、オープンな辞書ソース。

主たる対象は **日本語 ↔ 英語** の学習者ですが、設計はあらゆる言語ペアに対応できる汎用アーキテクチャです。

---

## 誰のためのツール？

| あなたが… | Kotoba が提供するもの |
|---|---|
| プログラマで日本語（または英語）を学んでいる | シェルプロンプトに今日の単語、エディタで瞬時の辞書、dotfiles リポジトリにデッキ |
| Duolingo を卒業した本気の独学者 | FSRS ベースのモダンな SRS と、自分が所有するデッキ |
| 教師・先生 | カリキュラムをコードのように公開・fork・バージョン管理 |
| 海外留学する学生 | オフラインでも持ち歩ける個人語彙ツールキット |
| 多言語学習者 | 一つのエンジンで複数の言語ペアをスクリプト処理 |
| コントリビューター | 整然とした Rust コア、プレーンテキストのデータ、`good first issue` 満載のロードマップ |

詳細なペルソナと将来の AI 時代のユースケースは [docs/use-cases.md](docs/use-cases.md) を参照。

---

## 動作イメージ

```
$ kotoba today
[木曜日] 食卓 (しょくたく) — dining table   3 reviews due

$ kotoba lookup 留学
留学 (りゅうがく)
  noun, suru-verb
  studying abroad
  JLPT N3 · 出現頻度 4,212

$ kotoba add 留学
✓ ~/.kotoba/decks/personal.md に追加しました

$ kotoba review
┌─ Kotoba — 8 件のカードが期限 ────────────────────────────┐
│                       留学                                │
│   [space] 答えを表示    [q] 終了    [s] 保留              │
└──────────────────────────────────────────────────────────┘
```

90 秒の毎日のルーチン。それが Kotoba のループです。

---

## 🚀 クイックスタート

> **ステータス:** Kotoba は早期アルファ版です。以下は v0.0.1 リファレンス実装の手順です。

```bash
git clone https://github.com/your-org/kotoba.git
cd kotoba
cargo build --release
./target/release/kotoba init
```

最初の 5 分:

```bash
kotoba init                    # ~/.kotoba にスターターデッキを作成
kotoba lookup ありがとう
kotoba add 食卓
kotoba review
kotoba today
```

シェルプロンプトに今日の単語を表示:

```bash
# zsh — ~/.zshrc に追記
eval "$(kotoba shell init zsh)"

# fish
kotoba shell init fish | source
```

---

## コントリビューションについて

**Rust が書けなくても貢献できます。**

- 🗂️ デッキを作る（Markdown を編集して PR を送るだけ）
- 🌏 UI の翻訳（あなたの母語で他の人を助けてください）
- 🎙️ 音声録音（ネイティブの発音は宝物です）
- ✏️ ドキュメント改善
- 🐛 バグ報告
- ⚙️ シェル統合・エディタプラグイン・ブラウザ拡張・モバイルアプリ

詳しくは [CONTRIBUTING.md](CONTRIBUTING.md) を参照。

---

## ライセンス

[MIT](LICENSE) — 自由に使えます。著作権表示だけ残してください。

辞書データ: [JMdict](https://www.edrdg.org/jmdict/edict_doc.html) および [KANJIDIC2](https://www.edrdg.org/kanjidic/kanjd2index.html) は EDRDG ライセンスの下で利用しています。

---

<p align="center">
  東京で丁寧に作っています。⛩️<br/>
  <em>「言葉は橋。」</em>
</p>
