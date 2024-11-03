+++
title = "NixOSでKMonadを導入して、Home Row Modsを設定する"
date = 2024-11-03
description = "KmoandのNixOSモジュールを使ってHome Row Modsを設定する"

[extra]
comment = true

[taxonomies]
tags = ["Nix", "NixOS", "home-manager"]
+++

## KMonadとは

マルチプラットフォームなキーボードコンフィギュレーター。要はQMKファームウェア以外のキーボードでも同じような動作を実現させるためのツール。Lispみたいな言語で設定できる。

## インストール

NixOSなら`services.kmonad`で設定できるがkmonadのNixOSモジュールを導入するところからやってみる。使っているバージョンは[flake.lock](https://github.com/satler-git/dotfiles/blob/394bd3ac066a1c881ce41ca07245c81da6aebda0/flake.lock)を見て欲しい。
[公式ドキュメント](https://github.com/kmonad/kmonad/blob/master/doc/installation.md#nixos)に詳しくのっているが、簡単に書くと、

```nix
{
  inputs.kmonad = {
    url = "git+https://github.com/kmonad/kmonad?submodules=1&dir=nix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { kmonad, ... }:
    {
      nixosConfigurations.<<systemName>> = nixpkgs.lib.nixosSystem {
        modules = [
          kmonad.nixosModules.default
        ];
      };
    };
}
```

として、通常のように `services.kmonad`に設定を書く。

## Home Row Modsとは？

詳細は[A guide to home row mods](https://precondition.github.io/home-row-mods)にのっているのだが、Home Row(QWERTY配列の場合は、ASDFJKL;)のキーをタップホールドにしてSHIFT, CTRL, Alt, Superを2回ずつ割り当てることをさす。左手(ASDF)と右手(JKL;)のModキーは対称になる。
順番はいろいろあって[^1]、迷ったのだが定番とされているGACS[^2]を使用する。

## 設定する

[Using Home Row Mods with KMonad](https://precondition.github.io/home-row-mods#using-home-row-mods-with-kmonad)という章がA guide to home row modsにもある。

```shell
ls /dev/input/by-id/
```
を実行して接続されているキーボードのデバイス名を調べて、以下の様にする。
```nix
{
  services.kmonad = {
    enable = true;
    keyboards = {
      "<<name>>" = {
        device = "/dev/input/by-id/<<device-name>>";
        defcfg = {
          enable = true;
          fallthrough = true;
        };
      };
    };
  };
}
```

次に `services.kmonad.keyboards.<name>.config`を設定する。
適当な場所に `<name>.kbd`などのファイルを作り、以下をコピペする。

```kbd
(defsrc
    a    s    d    f    g    h    j    k    l    ;
)

(defalias
    met_a (tap-hold-next-release 200 a lmet)
    alt_s (tap-hold-next-release 200 s lalt)
    ctl_d (tap-hold-next-release 200 d lctl)
    sft_f (tap-hold-next-release 200 f lsft)

    sft_j (tap-hold-next-release 200 j rsft)
    ctl_k (tap-hold-next-release 200 k rctl)
    alt_l (tap-hold-next-release 200 l lalt)
    met_; (tap-hold-next-release 200 ; rmet)
)

(deflayer homerowmods
    @met_a   @alt_s   @ctl_d   @sft_f   g   h   @sft_j   @ctl_k   @alt_l   @met_;
)
```

そして
```nix
{
  # keyboards = {
    # "<<name>>" = {
      # ...
  config = builtins.readFile <name>.kbd;
  # ...
}
```

#### KMonadがエラーで起動しない

`journalctl -u <service-name>`でログを確認する。Permission deniedの場合ユーザーが本来必要なグループに入れていない可能性がある。

以下のudevルールを作成する。

```udev
SUBSYSTEM=="misc", KERNEL=="uinput", MODE="660", GROUP="input"
```

そして自分を`input`グループに追加する。


## おわりに

KMonadの設定方法を調べながら頑張るぞ！と書きはじめたらA guide to home row modsにほとんどのっていた。そのうちHome Row Modsの使い心地の感想も書きたいところ

## 追記

しばらく試していてHome Row Mods自体は慣れれば悪くはなさそうだったのだが、KMonadの特性か分からないが遅延が大きくなってしまって(特にJKL;の遅延が大きいのはつらい)消した。今度キーボードを新しく作ろうと思っているからそのときに採用できるのかもう一度試したい。

JKL;が長押しできなくなってしまうが、そのこと自体はそこまで問題ないと思っている。なぜなら現在 `[count]j`などを使うように矯正中だから。
また最近、[kanata](https://github.com/jtroo/kanata)というツールがあることを知ったからそれで遅延が減るのか試したい。

[^1]: [Home Row Mods Order](https://precondition.github.io/home-row-mods#home-row-mods-order)を参照
[^2]: Super Alt Ctrl SHIFT, SHIFT, Ctrl, Alt, Superのような順番の略

