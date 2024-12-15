+++
title = "services-flakeとかを使って開発環境を作る"
date = 2024-12-14
description = "services-flakeを使ってプロジェクトごとにNixOSでいうservicesのようなものを使う方法について"

[extra]
comment = true

[taxonomies]
tags = ["Nix"]
+++

この記事は [Nix Advent Calendar 2024](https://adventar.org/calendars/10086) の14日目の記事です。

---

NixOSには `services` という設定があります。この設定を書くことで、例えばollamaなどの設定をすることが出来ます。自分のdotfilesでも以下のような形でollamaが設定されています。

```nix

{
  services.ollama = {
    enable = true;
    openFirewall = true;
    loadModels = [
      "llama3.2"
      # ...
    ];
  };
}
```

他にもたくさんの[ソフトウェア](https://search.nixos.org/options?query=services.)を `services` を介して設定することが出来ます。

NixのFlakesの機能の一つにdevShells[^1]というものがあります。これを使用することでグローバルの環境を汚すことなくプロジェクト専用のツール等をインストールすることが出来ます。

例えばこのブログは、[Zola](https://github.com/getzola/zola/)という静的サイトジェネレーターで生成されているのですが、グローバルにzolaのCLIを入れることなく、以下のような `flake.nix` を使ってdevShellsでインストールしています。

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          zola
        ];
      };
    };
}
```

この記事で紹介する [services-flake](https://github.com/juspay/services-flake) はこのdevShellsと同じような感覚でプロジェクトごとにservicesを設定しようというものです。
services-flakeはNixOS Modulesと同じようなことをするためのものである [flake-parts](https://github.com/hercules-ci/flake-parts) に依存しているわけではありませんが、あると書きやすいので使用しています。


## 環境

この記事で使ったファイルなどは [satler-git/sb-nix-services-flake](https://github.com/satler-git/sb-nix-services-flake)にあります。

またNixのバージョンは

```shell
❯ nix --version
nix (Nix) 2.24.10
```

nixpkgsとその他の `inputs` のバージョンはリポジトリの [flake.lock](https://github.com/satler-git/sb-nix-services-flake/blob/dd8f2ad0d368a03ab95d651a547d194b83f7186b/flake.lock) を確認してください。

## 実践

### ただのdevShells

まず適当な名前のディレクトリを作成し、そのディレクトリでflakeを初期化します。

```shell
❯ nix flake init
```

`flake.nix` を編集していきます。まずデフォルトで入っている `outputs` を消します。
そして `inputs`にflake-partsなどを追記して、flake-partsの `inputs.flake-parts.lib.mkFlake` を使い、flake.nixを以下のようにします。

```nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:juspay/services-flake";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      perSystem =
        {
          self',
          pkgs,
          config,
          lib,
          ...
        }:
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ ];
            buildInputs = with pkgs; [
              hello
              # Add more packages!
            ];
          };
        };
    };
}
```

この状態で `nix develop` を実行するとシステムに `hello` が入っていなくても使用できるようになっているはずです。

```shell
❯ hello
Hello, world!
```

### 無のprocess-compose

`flake-parts.lib.mkFlake` の引数の `imports` と `perSystem` の出力に `process-compose` を追加します。

```diff
+      imports = [
+        inputs.process-compose-flake.flakeModule
+      ];
       perSystem =
```

`perSystem` 周辺。

```diff
         {
+          process-compose."default-service" =
+            { config, ... }:
+            {
+              imports = [
+                inputs.services-flake.processComposeModules.default
+              ];
+
+              services = { };
+            };
+
           devShells.default = pkgs.mkShell {
-            inputsFrom = [ ];
+            inputsFrom = [
+              config.process-compose."default-service".services.outputs.devShell
+            ];
             buildInputs = with pkgs; [ hello ];
           };
         };
```

この時点では `nix run .#default-service` をしても何のサービスも起動されません。

### サービスの追加

#### Redis

以下のような設定をすることでRedisを使うことが出来ます。この設定を書いた状態で ` nix run .#default-service`を実行してみましょう。

また、ポートの6379はデフォルトですが例として書いてあります。
```diff
-              services = { };
+              services = {
+                redis."r1" = {
+                 enable = true;
+                  port = 6379; # port 6379 is the default
+                };
+              };
```

`nix develop` すれば、redis-cliも自動で追加されます。

```shell
❯ nix develop
❯ redis-cli
127.0.0.1:6379> SET test hello
OK
127.0.0.1:6379> GET test
"hello"
```

## おわりに

redisしか紹介していませんが、他のサービスは [ドキュメント](https://community.flake.parts/services-flake/services) に記載されています。DB等はかなり使えるようになっている印象です。また必要ならカスタムのサービスを作成したり、process-compose-flakeの機能を使ってプロセスが実行されるようにすることが可能です。

本当は今日はNixのTipsを投稿する予定でしたが、うまく記事がまとまらなかったため内容を変えました。

# 参照

- [services-flakeのドキュメント](https://community.flake.parts/services-flake/start)
- [services-flakeのexample/](https://github.com/juspay/services-flake/tree/9cf03e68a1fe33822f1a444ea47a7a9bce15e01e/example)

---

[^1]: `shell.nix`も同じような機能ではありますが、この記事で紹介するservices-flakeはFlakesにしか対応していないため焦点を絞っています
