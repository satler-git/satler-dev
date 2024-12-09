+++
title = "Nix入門入門とTips"
date = 2024-12-12
description = "Nixの入門に入門するのとTips"

[extra]
comment = true

[taxonomies]
tags = ["Nix", "NixOS"]
+++
# Tips

## 検索

Nixについての情報を得たい場合、何かしらで検索することになると思います。その方法です。

### Google

一般的な方法。NixOS DiscourseとかRedditとかのページがよくでてきます。

### GitHub

GitHubの検索機能を使う方法です。`repo:NixOS/nixpkgs `をつけて検索する方法と、`lang:Nix `をつけて検索する方法があります。前者はパッケージングで困ったとき、後者は前者で見つからないときとその他の場合に使うと良いと思います。

さらにパッケージ関連で困っているときはパッケージ名などでIssueを検索すると良いと思います。

### パッケージとオプション

- [Home Managerのオプションの検索](https://home-manager-options.extranix.com/)
- [NixOSのオプションの検索](https://search.nixos.org/options)
- [nixpkgsの検索](https://search.nixos.org/packages)

### [Noogle](https://noogle.dev/)

Nixには[Noogle](https://noogle.dev/)というものがあり、HaskellのHoogleと同じような感覚でnixpkgsのライブラリと標準(`builtins`)を検索できます。

## [Nixpkgs Pull Request Tracker](https://nixpk.gs/pr-tracker.html)

nixpkgsには複数のブランチがあり、それぞれどの程度安定しているのかが違います。[NixOSを使い始めた](/blog/kick-started)にも少し書きましたが、

## https://lazamar.co.uk/nix-versions/

# 言語ごとの話

ロックファイルが生成される言語またはパッケージマネージャーを使っているあなたは幸運です。すでにNixでのビルドをサポートするための何らかのツールが作られている可能性が高いです。[awesome-nixのプログラミング言語別の章] (<https://github.com/nix-community/awesome-nix?tab=readme-ov-file#programming-languages>)を見に行ってみましょう。
