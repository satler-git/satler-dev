+++
title = "Nixでcargo-competeをwrap(パッケージング?)する"
date = 2025-01-25
description = "Nixでパッケージングするときのテクニックなど"

[extra]
comment = true

[taxonomies]
tags = ["Nix"]
+++

(この人またNixの記事書いてる...)(なんだかんだ日本語の情報が少ないから増やしたい)

始めにとりあえず成果物。

[satler-git/sb-nix-cargo-compete -- GitHub](https://github.com/satler-git/sb-nix-cargo-compete)

- `packages.${system}.cargo-compete`
- `packages.${system}.cargo-compete-unwrapped`
- `packages.${system}.wrapped-rustup`

を含んでいる。

## 課題

普通のRustのソフトウェアなら、以下のようなNix式でビルドできる。

```nix
{
  packages.cargo-compete-unwrapped = pkgs.rustPlatform.buildRustPackage {
    pname = "cargo-compete";
    version = "${cargo-compete-src.rev}";

    buildInputs = [ pkgs.openssl ];
    nativeBuildInputs = [ pkgs.pkg-config ];

    src = cargo-compete-src;
    cargoHash = "sha256-r5QjwexX7btgT31xn59vG91g8DSMoUKWbi+nQxIdTvo=";

    doCheck = false; # tests in cargo-compete require network access

    meta = {
      description = "Unwrapped version of cargo-compete";
      mainProgram = "cargo-compete";
    };
  };
}
```

ただなにが問題かというと `cargo-compete` は、 `rustup` を使用して以下のようにバージョンを指定するのだ。

```shell
rustup run 1.70.0 cargo run --bin a
```

しかし、Nixユーザーはrustupを入れていない場合があるから、下手に依存したくないということだ。
ただrustupに依存している問題を解決したい場合は `buildInputs` に含めればいい。しかし様々な問題が発生する。

- 上のコマンドを実行する場合、すでにtoolchainをインストールしていなければならないため失敗する。しかも `rustup` はユーザーからは触れないから追加もできない
- `rustup` はデフォルトで `~/.rustup` フォルダを作る

## 解決策

### rustupにする小細工

上のパッケージの `wrapped-rustup` で行っている小細工について。パッケージの定義は以下のようになっている。シェルスクリプトは普段書かないから、LLMと一緒に書いた。

```nix
{
  packages.wrapped-rustup = pkgs.writeShellApplication {
    name = "rustup";

    runtimeInputs = with pkgs; [
      rustup
    ];

    # $2がrunの場合に--installを$2と$3の間に付ける
    text = ''
      if [ "''${1:-}" = "run" ]; then
        # Shift the first argument (removing 'run')
        command=$1
        shift

        # Ensure $2 and $3 exist
        if [ $# -ge 3 ]; then
          # Insert --install between $2 and $3
          new_args=("$command" "$1" "--install" "$2" "$3" "''${@:4}")
          rustup "''${new_args[@]}"
        else
          echo "Error: Missing required arguments for --install insertion."
          exit 1
        fi
      else
        rustup "$@"
      fi
    '';
  };
}
```

やっていることはコメントの通りだが、 `rustup run` の時に 自動的に `--install`(toolchainを自動でインストールする) フラグをつけ、
それ以外のコマンドはそのまま通すようになっている。そしてこれを `rustup` という名前で保存している。

### cargo-competeにする小細工

まず、 `cargo-compete` の定義は以下のようになっている(`flake-parts` を使っているから `self'`)。

```nix
{
  packages.cargo-compete = pkgs.stdenvNoCC.mkDerivation {
    pname = "cargo-compete";
    inherit (self'.packages.cargo-compete-unwrapped) version meta;

    nativeBuildInputs = with pkgs; [
      makeWrapper
    ];

    src = null;
    dontUnpack = true;

    postFixup = ''
      makeWrapper ${lib.getExe self'.packages.cargo-compete-unwrapped} $out/bin/cargo-compete \
        --prefix PATH : ${
          lib.makeBinPath (
            with pkgs;
            [
              self'.packages.wrapped-rustup
              gcc
            ]
          )
        } \
        --run 'export PATH=$PATH:~/.cache/cargo-compete/rustup/bin' \
        --run 'export RUSTUP_HOME=''${RUSTUP_HOME-~/.cache/cargo-compete/rustup}' \
        --run 'export CARGO_HOME=''${CARGO_HOME-~/.cache/cargo-compete/rustup}' \
        --run 'rustup default stable &> /dev/null'
    '';
  };
}
```

Nixのパッケージング向けの便利なパッケージ、 `makeWrapper` を使ってwrapした `rustup` と `gcc`(Rustのコンパイルの時に `cc` が必要)を `PATH` に加えている。さらに `RUSTUP_HOME` と `CARGO_HOME` を `~/.cache/cargo-compete/rustup` にすることで `~/.rustup` が作られないようにしている。また、`cargo metadata` が呼ばれるときのtoolchainを `stable` にしている。

`makeWrapper` はシェルスクリプトを書くまでもない簡単なwrapを作るのに便利。

