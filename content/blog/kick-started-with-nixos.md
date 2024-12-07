+++
title = "NixOSを使い始めた"
date = 2024-12-08
description = "今年の8月からNixOSを使いはじめたからその記録"

[extra]
comment = true

[taxonomies]
tags = ["Nix", "NixOS"]
+++
この記事は [Nix Advent Calendar 2024](https://adventar.org/calendars/10086) の8日の記事です。他の人の記事も面白くなる(期待)と思うのでぜひご覧ください！

## はじめに

この記事では、環境構築方法の詳細な紹介というより、dotfilesの成長過程とその記録を綴ったポエム的な内容をお届けする。

[dotfiles](https://github.com/satler-git/dotfiles) のコミットを振り返ると、初めての[コミット](https://github.com/satler-git/dotfiles/commit/4a7f323988c02a0f92c73d202aadb886d15bb9c0)は2024/8/26だ。その日にコミットされたのはNeovimとWindows向けタイル型ウィンドウマネージャーの[komorebi](https://github.com/LGUG2Z/komorebi/)(Windows向けのTilling WM)の設定のみだった。
そして初めて`.nix`ファイルが[コミット](https://github.com/satler-git/dotfiles/commit/ff1f288d4f8e755a2290444be7021d96d8354852)されたのは2024/8/29だ。この当時の設定は、Asahiさんの記事、[NixOSで最強のLinuxデスクトップを作ろう](https://zenn.dev/asa1984/articles/nixos-is-the-best)を読みながら `/etc/nixos/configuration.nix` とかをコピーしてきただけの段階だ。

現在のdotfilesは、1900行のNixと1300行のLuaによって支えられている。

## preInstallPhase

Linuxを使いたいという気持ち自体は1.5年くらい前からあってメインのデスクトップをWindowsとarchlinuxをデュアルブートしていた。archlinuxの方にはSwayとかを使ってある程度は環境構築がされていたが、めっちゃ使い心地が良いものではなかった。デュアルブートというのは結局どちらかの環境に傾いていくもので、そもそもそこまでニッチなことをしていなかったしWindowsにはWSLもあって何も困らずに作業できていた。

主にLinuxを使うモチベーションになっていたのはパフォーマンスと憧れだった。Neovimを今年の7月中旬くらいから使っていてlazy.nvimを使っていろいろプラグインを入れて、その後起動速度を最適化していたのだが限界があった。またCargo(rustc)のパフォーマンスも体感では少し遅かった。WindowsではDevDriveを使うことによってReFSが使用されるようにしてビルドが早くなるようにしていた。

自分をWindowsに引き止めていたのは慣れた環境だからというのもあったが、主にゲームだった。VALORANTやosu!などをやっていた。osu!は最近osu!lazerというプロジェクトでクロスプラットフォーム化していて遅延をそこまで気にしなければLinuxで使えるようになっていた。Linuxをメインに使用するため、VALORANTはプレイを断念した。

WindowsではscoopとChocolateyを併用していたので、それぞれのインストールしているパッケージをlistするコマンドを使ってファイルに出力してGitHub Gistにアップロードしておいた(余談だがGitHub CLIを使うと `gh gist create <filename>` でGistが簡単に発行できて便利)。

これでNixOSをインストールする準備が整った。

## installphase

自分はデスクトップにインストールしたから苦労することはなかったが、他の人はWiFiなどですこし苦労してるようだった。archlinux以外のLinuxをインストールするのはかなり久しぶりだったしなるべく簡単にインストールしたかったから、GUI Installer(GNOME)を選んでインストールした。Hyprland(Tilling WM)を使う予定だったが、なるべく簡単に次のステップにすすむためにGUIの踏み台としてGNOMEをインストールした。

## postinstallphase

今のdotfilesのフォルダ構成は以下のようになっている(一部を省略している)。最初は `nixos` ディレクトリにNixOSの設定を置いていたが最近は `hosts` に置いて共通化できるように変更していっている。

```
.
├── .sops.yaml
├── _sources
│   ├── generated.json
│   ├── generated.nix
├── config -- Nix以外の設定を置く場所
│   ├── SuperCollider
│   ├── bin
│   ├── nvim-ime
│   ├── nvim
│   └── zellij
├── flake.lock
├── flake.nix
├── home-manager -- Home Managerの設定
│   ├── linux.nix
│   ├── pkgs.nix
│   ├── programs
│   │   ├── alacritty.nix
│   │   ├── default.nix
│   │   ├── direnv.nix
│   │   └── ...
│   ├── services
│   │   ├── default.nix
│   │   ├── dunst.nix
│   │   └── gpg-agent.nix
│   └── stylix.nix
├── hosts
│   ├── desktop
│   │   ├── default.nix
│   │   ├── hardware-configrations.nix
│   │   └── hardware.nix
│   └── modules
│       └── hardware
│           ├── google-titan.nix
│           ├── microbit.nix
│           └── nvidia.nix
├── nixos
│   ├── default.nix
│   └── modules
│       └── ...
├── nvfetcher.toml
├── overlays
│   └── default.nix
├── secrets
│   └── ...
└── treefmt.nix
```

そして以下のようなinputsがある。

- nixpkgs
- nixpkgs-stable
- nixpkgs-unstable-small
- home-manager
    - 個人用の設定をする
- tidalcycles
- neovim-nightly-overlay
    - 最新に近いNeovimを使うために
- treefmt-nix
- zjstatus
- stylix
    - 統一的にテーマを設定できる
    - 詳しくは[NixOSでStylixを導入してみる](/blog/nixos-stylix-install/)
- sops-nix
    - dotfilesは公開するけどシークレットは使いたいから
- xremap
- nixos-hardware
- nix-gaming

nixpkgsには複数のブランチがあって設定をする上で選ぶ必要がある。

影響範囲が小さいPR（再ビルド数が500以下）はまずmasterにマージされ、その後、テストを経てnixos-unstable-small（テスト数が少ないブランチ）やnixos-unstable、nixpkgs-unstableに反映さる。影響範囲の大きいPRはもう少し違う動きをするが詳細は[nixpkgsのCONTRIBUTING.mdのstagingの章](https://github.com/NixOS/nixpkgs/blob/master/CONTRIBUTING.md#staging)を見てほしい。

基本的にはnixos-unstableを使っているが、たまにビルドできなくなることがある。そのため、nixpkgsのstable(今は24.11)とnixos-unstable-smallを使って、overrideしている。
なぜ2つあるのかについては、既にPRがマージされているがCIの関係(14日の記事にも書くが、[Nixpkgs Pull Request Tracker](https://nixpk.gs/pr-tracker.html)というサイトで追跡できる)でまだnixos-unstableに降りてきていないときがあるからだ。

## 感想

#### Nixは学習曲線が高い

Nixをいい感じに扱えるようになるまでにNix言語、NixOSの設定の仕方、Nixのパッケージマネージャーとしての仕組みなどいろいろなことをある程度理解する必要がある。しかも知識として依存しあっている。
Nix言語自体はちょっとJSONっぽい、純粋関数型言語だから比較的わかりやすいのだがそれをビルドに応用するというなかなか独特なことをしているから理解するのに時間がかかった。

#### NixOSの安心感

ネットワークとか音の設定が簡単にできたし安心感があった。archlinuxのときもそれぞれ設定していたけど簡単さではくらべものにならないくらいだった。

またネットワークと音の設定はそれぞれこんなかんじ(WiFiだともうちょっとめんどうくさいと思うけど)。

```nix
{
  networking = {
    networkmanager.enable = true;

    nameservers = [
      "8.8.8.8"
      "4.4.4.4"
    ];
  };
}
```

PipeWireの設定。

```nix
{
  services.pipewire = {
    enable = true;
    alsa = {
      enable = true;
      support32Bit = true;
    };
    pulse.enable = true;
    jack.enable = true;
  };
}
```

#### nixpkgsにコントリビュートできた

とあるパッケージが、バージョンアップしていないのにファイルを差し替える~~ちょっとNixと相性の悪い~~行為をしていてビルドできなくなっていたからハッシュを更新したPRを出した。それ以来コントリビュートできていないがまたやりたい気持ちはある。

## おわりに

結構NixOSを使い始めて時間がたって、最近はNix wayも知れてリファクタリングが捗るようになってきた。
あとこの記事はあまり実用的な内容が書けなかったのが心残りなので、14日に出す記事はもうすこし実用的なことを書きたいと思っている。

yasunoriさんの記事、[惰性でArchLinuxを使っていたが、必要に駆られてNixOSを使い出した](https://zenn.dev/yasunori_kirin/articles/0013-nixos-first-setup)のまとめの章、「まとめと、これからNixOSを始める人へ」に習ってこれから使い始める人に向けて書くと、

- Nixは難しいけどとりあえず始めてみるのが大事
    - Nixは他のLinuxとかUnixでも使えるし、いきなりNixOSをいれなくてもVMから始めてみるのもいい
- 分からないことは人に聞くのが一番なので、[vim-jp slack](https://vim-jp.org/docs/chat.html)の `#tech-nix` チャンネルや [ Nix日本語コミュニティ ](https://github.com/nix-ja) で聞くといいはず

## 参考文献

このほか様々な方のdotfilesを参照しましたが、省略しています。

- [NixOSで最強のLinuxデスクトップを作ろう](https://zenn.dev/asa1984/articles/nixos-is-the-best)
- [Nix入門](https://zenn.dev/asa1984/books/nix-introduction)
- [Nix入門: ハンズオン編](https://zenn.dev/asa1984/books/nix-hands-on)
- [NixOS Wiki (非公式)](https://nixos.wiki/)
- [NixOS Wiki (公式)](https://wiki.nixos.org/wiki/NixOS_Wiki)
- [NixOS & Flakes Book](https://nixos-and-flakes.thiscute.world/)
- [nix.dev](https://nix.dev/)
