+++
title = "Nixで作るRustプロジェクト用の開発環境"
date = 2024-12-21
description = "NixとRustでいい感じのRustプロジェクトを作ります"

[extra]
comment = true

[taxonomies]
tags = ["Nix", "Rust"]
+++

この記事は [Nix Advent Calendar](https://adventar.org/calendars/10086) の21日目の記事です。

---

## はじめに

現在開発しているRustプロジェクトで使っている `flake.nix` は大体以下のようなものです。長いので今は細かくみる必要はありません。また、トップレベルに `Cargo.toml` と `rust-toolchain.toml` があることを想定しています。

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix.url = "github:numtide/treefmt-nix";

    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs@{
      self,
      flake-parts,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      flake = { };

      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.git-hooks-nix.flakeModule
      ];

      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem =
        {
          config,
          system,
          pkgs,
          lib,
          self',
          ...
        }:
        let
          rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain rust-bin;

          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            buildInputs = with pkgs; [ ];

            nativeBuildInputs = with pkgs; [ ];
          };

          cargoArtifacts = craneLib.buildDepsOnly (
            commonArgs
            // {
              pname = "deps";
            }
          );
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };

          treefmt = {
            projectRootFile = "flake.nix";

            programs.actionlint.enable = true;
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
            programs.taplo.enable = true;
            programs.yamlfmt.enable = true;
          };

          pre-commit = {
            settings = {
              hooks = {
                flake-treefmt = {
                  enable = true;
                  name = "flake-treefmt";
                  entry = lib.getExe config.treefmt.build.wrapper;
                  pass_filenames = false;
                };

                clippy.enable = true;
                cargo-check.enable = true;
              };

              settings.rust.check.cargoDeps = pkgs.rustPlatform.importCargoLock {
                lockFile = ./Cargo.lock;
              };
            };
          };

          packages.default = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              pname = "template"; # TODO: rename
              version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
            }
          );

          devShells.default = pkgs.mkShell {
            inputsFrom = [ config.pre-commit.devShell ];

            buildInputs = with pkgs; [
              cargo-expand
              cargo-nextest

              rust-bin
            ];
          };
        };
    };
}
```

## 各部の説明

### `inputs`

`inputs`(flakeの入力部分)で以下のようなものを取得しています。

- [nixpkgs](https://github.com/NixOS/nixpkgs/)
- [flake-parts](https://github.com/hercules-ci/flake-parts)
    - NixOSの設定などで使われているModule Systemをflakeにも統合してくれます。またいい感じにクロスプラットフォームに出来ます(この機能は自前の関数で簡単に実装できたり、もっとシンプルなやつもある)
- [rust-overlay](https://github.com/oxalica/rust-overlay)
    - `rustc` や `cargo` のnixpkgsとは違うバージョンを提供してくれるflakeです。nixpkgsで保守される `rustc` や `cargo` は最新版のみなので使用します
- [git-hooks-nix](https://github.com/cachix/git-hooks.nix)
    - [Pre Commit](https://pre-commit.com/)のGit HooksとNixプロジェクトをいい感じに統合してくれます
- [treefmt-nix](https://github.com/numtide/treefmt-nix)
    - [https://github.com/numtide/treefmt](https://github.com/numtide/treefmt)という複数のフォーマッターをまとめて実行できるツールをNixに統合してくれます
- [crane](https://github.com/ipetkov/crane)
    - Rustのプロジェクトをビルドするのに便利な機能などを提供してくれます

### `outputs`

どちらかというと重要なのは `inputs` よりもこちらです。

#### flake-parts

```nix
{
  outputs =
    inputs@{
      self,
      flake-parts,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {

      flake = { };

      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.git-hooks-nix.flakeModule
      ];

      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem = { ... }: { };#...
    };
}
```

`systems` で `perSystem` 部分に記述したAtribute Setの生成先を指定します。例として、以下のようなNix式は

```nix
{
  outputs =
    inputs@{
      self,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem = { ... }: {
        hello = "world";
      };
    };
}
```

このようになります(多分実際に試すと `hello` なんてないよみたいな感じで怒られます)。

```nix

{
  outputs = {
    hello.aarch64-linux = "world";
    hello.x86_64-linux = "world";
  };
}
```

`imports` では読み込む Flake Moduleを指定しています。また `flake-parts.lib.mkFlake` の引数、 `flake` には `x86_64-linux` などを追加しないもともとのflake要素をいれられますが今回のものではなにもいれていません。

#### `perSystem`

##### 変数の定義

`rust-toolchain.toml` からRustのDerivation( `rust-bin` )を生成してそこから `craneLib` を作成します。
それを利用して、`commonArgs`(`crane` から生成するパッケージに共通で使う引数郡)と`commonArgs` と `craneLib` を使って `cargoArtifacts` を作成します。

`cargoArtifacts` はプロジェクトの依存だけのDerivationでこれを作成することで依存関係をキャッシュさせることが出来るようになります。

```nix
let
  rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  craneLib = (crane.mkLib pkgs).overrideToolchain rust-bin;

  commonArgs = {
    src = craneLib.cleanCargoSource ./.;
    strictDeps = true;

    buildInputs = with pkgs; [ ];

    nativeBuildInputs = with pkgs; [ ];
  };

  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      pname = "deps";
    }
  );
in
{
    # ...
}
```

##### 他

以下のようにすることで `inputs` の `rust-overlay` を読み込むことが出来ます。`_module.args.pkgs` はflake-partsの特殊な書き方で、通常は `let` の中で `pkgs` を定義します(`system` に自分のシステムをいれる)。

```nix
{
  _module.args.pkgs = import inputs.nixpkgs {
    inherit system;
    overlays = [
      inputs.rust-overlay.overlays.default
    ];
  };
}
```

`treefmt-nix` と `git-hooks-nix` の設定です。`nix flake fmt` でtreefmtが、`nix flake check` でclippyとcargo-checkが実行されます。コミット時にはどちらも実行されます。

```nix
{
  treefmt = {
    projectRootFile = "flake.nix";

    programs.actionlint.enable = true;
    programs.nixfmt.enable = true;
    programs.rustfmt.enable = true;
    programs.taplo.enable = true;
    programs.yamlfmt.enable = true;
  };

  pre-commit = {
    settings = {
      hooks = {
        flake-treefmt = {
          enable = true;
          name = "flake-treefmt";
          entry = lib.getExe config.treefmt.build.wrapper;
          pass_filenames = false;
        };

        clippy.enable = true;
        cargo-check.enable = true;
      };

      settings.rust.check.cargoDeps = pkgs.rustPlatform.importCargoLock {
        lockFile = ./Cargo.lock;
      };
    };
  };
}
```

RustのDerivationを作成しています。先に定義した、`cargoArtifacts` を使っています。また、versionには `Cargo.toml` からバージョンを取得してその値を入れています。

```nix
{
  packages.default = craneLib.buildPackage (
    commonArgs
    // {
        inherit cargoArtifacts;
        pname = "template"; # TODO: rename
        version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
    }
  );
}
```

cargo-expandとcargo-nextest、そしてバージョンを指定した `rustc` と `cargo` 付きのdevShellを定義しています。`nix develop` するとdevShellが起動します。また、`pre-commit` を使えるようにしています。

```nix
{
  devShells.default = pkgs.mkShell {
    inputsFrom = [ config.pre-commit.devShell ];

    buildInputs = with pkgs; [
      cargo-expand
      cargo-nextest

      rust-bin
    ];
  };
}
```

# おわりに

すぐに使える `flake.nix` ではありますが、いろいろもりもりで長めの記事になってしまいました。実際は部分的に導入していくことも出来ます。
最初の `flake.nix` は今作っている [satler-git/etymora](https://github.com/satler-git/etymora) というプロジェクトの `flake.nix` に部分的に変更を加えたものです。実は他に [satler-git/rust-template](https://github.com/satler-git/rust-template) というリポジトリもありますが、なかなかフィードバックが出来ていません。

