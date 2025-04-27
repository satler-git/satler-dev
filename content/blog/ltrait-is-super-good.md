+++
title = "LTraitっていうFuzzyFinderのようなものを作った"
date = 2025-04-27
description = "LTraitというFuzzyFinderを作ったので紹介がてらのチュートリアル記事" # TODO:

[extra]
comment = true

[taxonomies]
tags = ["rust"]
+++

[ltrait/core](https://github.com/ltrait/core) とちょっとした拡張機能郡を作りました。

# LTrait is 何?

LTraitはカスタマイズできるOS用のFuzzyFinderです。Launcherでもあります。たとえばRaycastとかと近い動作をさせられます。

名前はXMonad(Haskellで実装されているXサーバー)を参考にして、Rustで実装しているLauncherなのでL(auncher)Traitです。「えるとれいと」とと読みます。

コンセプト自体はfall.vimとかからインスパイアされています。

# 試しに設定

(これはltraitの[ドキュメント](https://github.com/ltrait/core/blob/cc843acc6e457fcb4aaf0be48bc89798d117b7b7/docs/Concepts.md)の翻訳です。翻訳をさらに翻訳している不思議)

LTraitにはfall.vimやその他のVim用のFuzzyFinderのようにいろいろな種類の拡張を組み合わせて使います。拡張はそれぞれトレイトになっています。

拡張の種類には以下のようなものがあります(名前をクリックするとトレイトの定義に飛びます)。

| 名前                                                                               | 説明                                           |
| -------------------------------------------------------------------------------- | -------------------------------------------- |
| [Source](https://docs.rs/ltrait/latest/ltrait/source/type.Source.html)           | ほとんど `Stream<Item = Item>`。 データのソース          |
| [Generator](https://docs.rs/ltrait/latest/ltrait/generator/trait.Generator.html) | Sourceに似ているけど、ユーザーの入力(テキスト)を受け取ってからItemを生成する |
| [Filter](https://docs.rs/ltrait/latest/ltrait/filter/trait.Filter.html)          | 一つのItemとユーザーからの入力を受け取ってそのItemを残すかどうか判断する     |
| [Sorter](https://docs.rs/ltrait/latest/ltrait/sorter/trait.Sorter.html)          | 2つのItemとユーザーからの入力を受け取って比較する                  |
| [UI](https://docs.rs/ltrait/latest/ltrait/ui/trait.UI.html)                      | ユーザーの入力を管理したり、表示したりする。                       |
| [Action](https://docs.rs/ltrait/latest/ltrait/action/trait.Action.html)          | Itemを受け取ってなにかを実行する                           |

{{ img(id="https://res.cloudinary.com/dsexsi1cq/blog/ltrait-is-super-good/fshq3ggfx04sxvj3okva.png", height="699", width="864") }}

## プロジェクトを作る

適当なディレクトリに移動して以下のコマンドを実行します。

```shell
cargo new hello-ltrait
cd hello-ltrait
cargo add ltrait
cargo add tokio --features=full
```

また、 `src/main.rs` に以下を書き込んで、エラーハンドラーとロガーを設定します。

```rust
use ltrait::color_eyre::Result;
use ltrait::{Launcher, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // keeping _guard is required to write log
    let _guard = ltrait::setup(Level::INFO)?;
    // TODO: Configure and run Launcher

    Ok(())
}
```

## Cusion

Cusionは、LTraitを設定する上で重要な概念です。複数のSourceを設定したいときに、同じ型を返さないので、

```rust
enum Item {
    First(DesktopEntry),
    Second(String),
}
```

のようにユーザーが定義します。
Sourceが返す型を一度Cusionに変換してから、次はそのCusionをFilterとかSorterが使う型に変換するみたいな感じで運用していきます。

## UIを設定してとりあえずランチャーみたいにする

ltrait-ui-tuiというUIがあります(今はそれしかないです)。~~面倒で~~crates.ioにはアップロードしていないので、GitHubを経由して追加します。

```shell
cargo add ltrait-ui-tui --git https://github.com/ltrait/ui-tui
```

そして`src/main.rs`を以下のようにします。詳細はコメントも参照してください。

```rust
use ltrait::color_eyre::Result;
use ltrait::{Launcher, Level};

use ltrait_ui_tui::{Tui, TuiConfig, TuiEntry, style::Style, Viewport};

// strum使うと便利
enum Item {
    // TODO: add source
}


impl Into<String> for &Item {
    fn into(self) -> String {
        match self {
            // Itemの要素をStringに変換する。あるとなにげに便利
            // TODO: Itemに追加したらここも実装
            _ => "unknown item".into()
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    // _guardをドロップするとログが取られない。
    let _guard = ltrait::setup(Level::INFO)?;

    let launcher = Launcher::default()
        .set_ui(
            Tui::new(TuiConfig::new( // TUIの表示の設定。
                Viewport::Fullscreen,
                '>', // 選択
                ' ',
                // キーコンフィグはClosureを渡すことで変更できる。とりあえずsample_keyconfig
                ltrait_ui_tui::sample_keyconfig,
             )),
            |c: &Item| TuiEntry {
                text: (c.into(), Style::new()),
            },
        );

    launcher.run().await
}
```

まだSourceやGeneratorを追加していないので、なにも表示されません。試しに実行してみてください。

```shell
cargo run
```

## Source, Filter, Sorterを追加してみる

`Launcher`はビルダーパターンを採用しています。拡張機能ごとに、`add_**`と`add_raw_**`が用意されています。

`add_**`はSourceだとCusionに変換する関数を、それ以外だとCusionから変換する関数を受け取ります。
`add_raw_**`は少し違う動作をします。変換関数を受け取らないので、SourceならCusionをそのまま返したり、それ以外ならCusionを直接受けとったりする拡張を追加できます。
つまりほとんど、`add_**(/* ... */, |c| c)`のような動作をします(lifetimeの関係で実際はこうは書けない)。

これは、[ltrait/extra](https://github.com/ltriat/extra)にある便利関数郡だったり、`ltrait::**::Closure**`(Closureで拡張機能を実装する)を使うときに便利です。
`add_raw_**`を使うとCusionを定義しなくても使えますが、拡張性が著しく下がるのでおすすめはしません。

SourceはStreamでアイテムを非同期に処理できますが、シンプルに実装するならそれは必要ではありません。`ltrait::source::from_iter`を使うことでIteratorからSourceを作成できます。

試しに `src/main.rs` を以下のように書き換えてみてください。

```rust
use ltrait::color_eyre::Result;
use ltrait::{
    Launcher,
    Level,
    filter::ClosureFilter,
    sorter::ClosureSorter,
};

use ltrait_ui_tui::{Tui, TuiConfig, TuiEntry, style::Style, Viewport};

use std::cmp;

enum Item {
    Num(u32)
}


impl Into<String> for &Item {
    fn into(self) -> String {
        match self {
            Item::Num(x) => format!("{x}"),
            _ => "unknown item".into()
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let _guard = ltrait::setup(Level::INFO)?;

    let launcher = Launcher::default()
        // 一番シンプルなSource
        .add_source(ltrait::source::from_iter(1..=5000), /* transformer */ Item::Num)
        // 偶数だけを残すFilter
        .add_raw_filter(ClosureFilter::new(|c, _ /* 入力。無視する */| {
            match c {
                Item::Num(x) => (x % 2) == 0,
                _ => true, // 将来的にItemに種類が追加されても無視する
            }
        }))
        .reverse_sorter(false)
        .add_raw_sorter(ClosureSorter::new(|lhs, rhs, _| {
            match (lhs, rhs) {
                (Item::Num(lhs), Item::Num(rhs)) => lhs.cmp(rhs),
                _ => cmp::Ordering::Equal
            }
        }))
        .batch_size(500)
        .set_ui(
            Tui::new(TuiConfig::new(
                Viewport::Fullscreen,
                '>',
                ' ',
                ltrait_ui_tui::sample_keyconfig,
            )),
            |c| TuiEntry {
                text: (c.into(), Style::new()),
            },
        );

    launcher.run().await
}
```

試しに実行してみてください。まだ入力してもなにも意味はありませんが、1~5000の偶数が順番に表示されるはずです。

```
cargo run
```

もっと拡張機能を追加したくなったら(とりあえずは) [ltrait/repositories](https://github.com/orgs/ltrait/repositories?type=source) を見てみてください。簡単に自作も出来ます！

## ちょっと高度な話

`batch_size`というのをLauncher経由で指定できます。これは一度に何個アイテムを取得してUIに表示するかという数字です。
よほどSourceから受けとる個数が多くないかぎりは `0` (一度に全てを取得)を指定してもパフォーマンスが顕著に下がることはないです。

Sourceからの取得の速度をベースに最適な値は決まるため、最適な値を出すのは難しいです。

# 自分の設定

自分の設定は [satler-git/yurf](https://github.com/satler-git/yurf) に置いてあります。
もしNixを使っているなら、

```shell
nix run github:satler-git/yurf -- launch
```

でLauncherを実行することが出来ます。

他にも何個かサブコマンドがあります。

- stdin
    - stdinから取得して結果をstdoutに出力する
- task
    - `$HOME/.config/yurf/config.toml` に定義されたタスク(名前とコマンドのペア)を読みとってそこから選んで実行します。
    - 輝度を調節したりできるようにしてある。結構便利
    - まだバグがある

サブコマンドごとに追加するActionやSorter、Sourceなどを変えています。くわしくはリポジトリを見てみてください。

