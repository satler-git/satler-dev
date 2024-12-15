+++
title = "Rustのコマンドラインパーサーのxflagsを試す"
date = 2024-11-22
description = "Rust用のシンプルなコマンドラインパーサーのxflagsを使ってとてもシンプルなCLIツールを実装した"

[extra]
comment = true

[taxonomies]
tags = ["Rust"]
+++

RustのLSであるrust-analyzerのコードを眺めていて(今作っているユーティリティのために他のRust製のLSの依存関係とかコードを調査していた)、xtaskの依存関係の中にxflagsが入っていることに気が付いてコードを見たかんじ便利そうだから試してみる。

[matklad/xflags](https://github.com/matklad/xflags)にxflagsのコードはある。また今回作成したコードは[satler-git/sandbox内](https://github.com/satler-git/sandbox/tree/c4ec939d35470c9866c6d0c5e9ba205ab65681c1/rust/try-xflags)にある。

## 注意

Rustの有名なコマンドラインパーサーにclapがある。どちらを選べばいいのかは[xflagsのドキュメント](https://docs.rs/xflags/0.3.2/xflags/)にこう記載がある。

> if you need all of the features and don’t care about minimalism, use clap.  
> if you want to be maximally minimal, need only basic features (eg, no help generation), and want to be pedantically correct, use lexopt.  
> if you want to get things done fast (eg, you want auto help, but not at the cost of waiting for syn to compile), consider this crate.

雑訳

> もし完全な機能が欲しくてミニマリズムにもこだわらないなら、clapを使う。  
> もし最大限にミニマル(最小限の機能しかなくhelpの生成などが必要ない場合)、そして厳密な正確性が欲しい場合、lexoptを使う。  
> もし事が素早く作業を終わらせたい(helpの生成は欲しいけどsynのコンパイルを待つコストを払いたくない)、このクレートを検討する。  

ようはxtaskみたいなものに使われるように設計されているということ。

# 基本

この記事で使っているRustのバージョンは以下の通り

```shell
sandbox/rust
❯ cargo --version
cargo 1.82.0 (8f40fc59f 2024-08-21)

sandbox/rust
❯ rustc --version
rustc 1.82.0 (f6e511eec 2024-10-15)
```

またxflagsのバージョンは0.3.2。

xflagsはproc-macroを使うことでパーサーを生成したりできる。xflagsのマクロには`xflags::xflags!`と`xflags::parse_or_exit!`があり、前者はパーサーのコードが生成され後者はパーサーのコードが生成されその場でperseして構造体を返す。`xflags::xflags!`で`parse_or_exit!`と同じ機能を実現するには(例えば)`flags`モジュールに`xflags!`マクロを書いてそこに生成された構造体の`from_env_or_exit`関数を呼び出して利用する。
またそれぞれのマクロで特殊な構文を使用してコマンドを定義する。

# 実践

まず`parse_or_exit!`を追加する。

```rust
fn main() {
    let flags = xflags::parse_or_exit! {};
}
```

## 必須の引数を定義する

必須の引数を定義するにはマクロのなかでまず`required`としてから名前と型を指定する。ヘルプの生成に使用されるドキュメンテーションコメントも記載する必要がある。

```rust
fn main() {
    let flags = xflags::parse_or_exit! {
        /// num to succ
        required num: u32
    };
}
```

## オプションを定義する

さらにオプションを定義するには先頭に`optional`を付けることが出来る。

```rust
+ /// Decrease the input number
+ optional -d, --decrease
```

そして機能を足したコードは以下のようになる。

```rust
fn main() {
    let flags = xflags::parse_or_exit! {
        /// Decrease the input number
        optional -d, --decrease
        /// num to succ
        required num: u32
    };

    if flags.decrease {
        println!("{}", flags.num.checked_sub(1).unwrap_or_default());
    } else {
        println!("{}", flags.num + 1);
    }
}
```

実行すると以下のよう。分かるようにビルドはとても早かった(マシンによって差は出るが自分のマシンの場合は0.29sとなっている)。またヘルプが自動的に生成されている。

```shell
sandbox/rust/try-xflags
❯ cargo run --bin succ -- 1
   Compiling xflags v0.3.2
   Compiling try-xflags v0.1.0 (/home/satler/repos/github.com/satler-git/sandbox/rust/try-xflags)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/succ 1`
2
sandbox/rust/try-xflags
❯ ./target/debug/succ 1 --decrease
0
sandbox/rust/try-xflags
❯ ./target/debug/succ --help
ARGS:
    <num>
      num to succ

OPTIONS:
    -d, --decrease
      Decrease the input number

    -h, --help
      Prints help information.
```

またcargo-expandした結果の[gist](https://gist.github.com/satler-git/b33fe0d7159bbbe5a678d9c18551eec0)を載せておく。

## サブコマンド、`repeated`

~~サブコマンドは`cmd`キーワードを用いて作成できる。また複数名前を書くことでエイリアスを追加できる。しかし実践はしなかったから詳しくは[ドキュメント](https://docs.rs/xflags/0.3.2/xflags/#syntax-reference)を見て欲しい。~~

```rust
fn main() {
    let flags = xflags::parse_or_exit! {
        cmd run r exec {}
    }
}
```

追記[^1]
`cmd` キーワードは `parse_or_exit!` の中では使えないので、以下のようにする必要がある。詳しい使い方はこのブログの [xtask](https://github.com/satler-git/satler-dev/blob/c69007b646de2452b731737afff12e378bbe264e/xtask/src/main.rs) で見てほしい。


```rust

mod flags {
    xflags::xflags! {
        cmd xtask {
            /// add new blog post
            cmd run r exec {
                /// force file creation
                /// if the file exits, it will be replaced
                optional -f, --force
            }
        }
    }
}
```

# おわり

まだxtaskを活かせるような大きなプロジェクトは出来ていないがそのうち活用していきたい。シンプルに記述できてやりやすかった。

---

2024/12/15日追記
[^1]: `cmd` キーワードは `parse_or_exit!` マクロでは使えない。おそらく複数の `struct` を生成する必要があるから。



