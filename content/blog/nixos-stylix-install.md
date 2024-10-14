+++
title = "NixOSでStylixを導入してみる"
date = 2024-10-14
description = "About blog post"

[extra]
comment = true

[taxonomies]
tags = ["Nix", "NixOS", "home-manager"]
+++

## Stylixとは

[StylixのドキュメントのIntroduction](https://stylix.danth.me/index.html)には以下のように書いてある。

> Stylix is a NixOS module which applies the same colour scheme, font and wallpaper to a range of applications and desktop environments.

雑に訳すと

> Stylixは同じカラースキーマ、フォントや壁紙を適用するNixOSモジュール。様々なアプリケーションとデスクトップ環境で使用できる。

## インストール

自分の環境配下の通り

```shell
❯ nixos-version
24.11.20241009.5633bcf (Vicuna)
❯ nix --version
nix (Nix) 2.18.8
```

まずはStylixをFlakeの入力に追加する
```nix
# ...
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  stylix.url = "github:danth/stylix";
};
# ...
```

そして、StylixではNixOS moduleが `stylix.nixosModules.stylix` で提供されているから、`modules` に追記する。

```nix
nixosConfigurations = {
  desktop = nixpkgs.lib.nixosSystem {
    # ...
    modules = [
      # ...
      stylix.nixosModules.stylix
      # ...
    ];
  };
};
```

また、ドキュメントでは[Home ManagerをNixOSモジュールとして導入すること](https://nix-community.github.io/home-manager/index.xhtml#sec-install-nixos-module)が推奨されている。
NixOS以外のインストール方法は[ドキュメント](https://stylix.danth.me/installation.html)を見てほしい。

## 設定

以下のような設定を書いて、home-managerのファイルからインポートする。自分はフォントはここで設定しないことにした。カラースキームは設定しなくても壁紙を設定すれば勝手に生成してくれる。また逆にカラースキームから壁紙を生成することもできる。
以下に自分の設定をのせる。長いからスクロールする必要がある。

```nix
{ pkgs, config, ... }:
let
  theme = "catppuccin-frappe";
in{
  stylix = {
    enable = true;

    image = pkgs.fetchurl {
        url = "https://raw.githubusercontent.com/NixOS/nixos-artwork/ea1384e183f556a94df85c7aa1dcd411f5a69646/wallpapers/nix-wallpaper-nineish.png";
        hash = "sha256-EMSD1XQLaqHs0NbLY0lS1oZ4rKznO+h9XOGDS121m9c=";
    };

    cursor.package = pkgs.bibata-cursors;
    cursor.name = "Bibata-Modern-Ice";

    base16Scheme = "${pkgs.base16-schemes}/share/themes/${theme}.yaml";

    fonts = {
      serif = {
        package = pkgs.noto-fonts-cjk-serif;
        name = "Noto Serif CJK JP";
      };

      sansSerif = {
        package = pkgs.noto-fonts-cjk-serif;
        name = "Noto Sans CJK JP";
      };

      monospace = {
        package = pkgs.jetbrains-mono;
        name = "JetBrainsMono Nerd Font";
      };

      emoji = {
        package = pkgs.noto-fonts-emoji;
        name = "Noto Color Emoji";
      };
  };


    # 無効にするターゲット
    targets.alacritty.enable = false;
    targets.firefox.enable = false;
    targets.neovim.enable = false;
    targets.waybar.enable = false;
    targets.gnome.enable = false;
    targets.zellij.enable = false;
  };
}
```

設定の詳細についてはドキュメントの[Home Manager options](https://stylix.danth.me/options/hm.html)などを見てもらえばいい。
また既存の設定とたくさんコンフリクトが発生したから解決していく。
~よくわからないけどフォントサイズが変わった。~

### エラー " The option `stylix.image' was accessed but has no value defined. Try setting the option."

これは正直よく分かっていないが、Home ManagerではなくNixOSの設定で以下のようにしたら消えた。

```nix
stylix = {
  homeManagerIntegration.followSystem = false;
  homeManagerIntegration.autoImport = true;

  enable = false;
};
```

# 終わり

おわり。上のエラーで結構詰まったがそれ以外はかんたんにできた。あと最初と最後でほとんど変わっていないから面白みはない。
本当は自分の好きなカラースキームを手動で登録したかったけどそれは12色しかなくてbase16に足りなかったから諦めた...

