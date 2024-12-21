+++
title = "NixのTips"
date = 2024-12-21
description = "NixのTips"

[extra]
comment = true

[taxonomies]
tags = ["Nix", "NixOS"]
+++

この記事は[Nix Advent Calendar](https://adventar.org/calendars/10086)の21日目の記事です。

---

もともとはコード例をまじえたチュートリアル的な記事にしようとおもったのですが、結局うまくまとまらなかったためちょっとしたTipsの記事になりました。

# Tips

いろいろ書いているので、自分に関係ないなと感じたものは適当に読みとばしてください。

## 検索

Nixについての情報を得たい場合、何かしらで検索することになると思います。その方法です。

### Google

一般的な方法。NixOS DiscourseとかRedditとかのページがよくでてきます。

### GitHub

GitHubの検索機能を使う方法です。`repo:NixOS/nixpkgs `をつけて検索する方法と、`lang:Nix `をつけて検索する方法があります。前者はパッケージングで困ったとき、後者は前者で見つからないときとその他の場合に使うと良いと思います。

さらにパッケージ関連で困っているときはパッケージ名などでIssueを検索すると良いと思います。

この方法はかなり強力なので困ったら一旦これを使っています。

### パッケージとオプション

- [Home Managerのオプションの検索](https://home-manager-options.extranix.com/)
- [NixOSのオプションの検索](https://search.nixos.org/options)
- [nixpkgsの検索](https://search.nixos.org/packages)

### [Noogle](https://noogle.dev/)

Nixには[Noogle](https://noogle.dev/)というものがあり、HaskellのHoogleと同じような感覚でnixpkgsのライブラリと標準(`builtins`)の関数を検索できます。

## [Nixpkgs Pull Request Tracker](https://nixpk.gs/pr-tracker.html)

nixpkgsには複数のブランチがあり、それぞれどの程度安定しているのかが違います。よく使うブランチには以下のようなものがあります。

- `nixpkgs-unstable`
- `nixos-unstable`
- `nixos-unstable-small`
- `release-<version>`
    - `<version>` にはリリースのバージョンが入ります。 `yy.mm` の形になっていて毎年5月と11月にリリースされます。記事投稿時の最新は `24.11` です。

[NixOSを使い始めた](/blog/kick-started-with-nixos)にも少し書きましたが、修正はnixpkgsに対してPull Request(以下PR)が作成され、まず `master` にマージされます。そのあとテストやビルドが実行され `nixos-unstable-small`、 `nixos-unstable` や `nixpkgs-unstable` にマージされます。`release-<version>` にはPRがバックポートするPRに指定されないとマージされません。

テストなどを実行するため、 `master` にマージされてから他のブランチにマージされるまでにはラグがあります。そこでPRがどこまで進んでいるのかを確認するためにこのツールが使えます。問題を修正するPRがマージされているのに手元では修正されない場合に確認してみてください。

## [Nix Version](https://lazamar.co.uk/nix-versions/)

nixpkgsは基本的にパッケージごとに個別のバージョンを指定することが出来ません。しかし、複数のnixpkgsを同時に使えば(少しHackyですが)できなくはないです。そのときに[Nix Version](https://lazamar.co.uk/nix-versions/)を使えばあるパッケージがどのnixpkgsに含まれているかを確認することが出来ます。

## `nix develop` 関連

### [nix-your-shell](https://github.com/mercurytechnologies/nix-your-shell) で好みのシェルを使う

なにもしていない場合、 `nix develop` のシェルにはbashが使われます。しかし、nix-your-shellを使えば好みの別のシェルも使えるようになります。

nix-your-shellはnixpkgsに `nix-your-shell` としてパッケージされています。インストールできたら、シェルのプロファイルに起動するためのスクリプトを追記する必要があります。
例えばzshなら以下ですが他のシェルについては [プロジェクトのREADME](https://github.com/MercuryTechnologies/nix-your-shell#usage)を確認してください。

```shell
if command -v nix-your-shell > /dev/null; then
  nix-your-shell zsh | source /dev/stdin
fi
```

### [nix-direnv](https://github.com/nix-community/nix-direnv) で自動的に `nix develop` を実行する

[direnv](https://github.com/direnv/direnv)というツールのNixのための拡張です。Home Mangerを使っている場合は以下のNix式でインストールできます。

```nix
{
  programs = {
    direnv = {
        enable = true;
        nix-direnv.enable = true;
      };
    };
  };
}
```

そして `shell.nix` を使うなら `use nix` を、FlakesのdevShellsを使うなら `use flake` を `.envrc` に追記します。そして `direnv allow` を実行すればそのディレクトリにcdすると自動で起動されるようになります。

# 言語ごとの話

ロックファイルが生成される言語またはパッケージマネージャーを使っているあなたは幸運です。すでにNixでのビルドをサポートするための何らかのツールが作られている可能性が高いです。[awesome-nixのプログラミング言語別の章](<https://github.com/nix-community/awesome-nix?tab=readme-ov-file#programming-languages>)を見に行ってみましょう。

# おわりに

Tipsの共有は大事なのでぜひみなさんも記事を書いてみてください！自分もいつか入門記事を書きます(多分)。

最近 [nix-jaのCosense](https://scrapbox.io/nix-ja) ができたのでそこにも書きます(多分)。

