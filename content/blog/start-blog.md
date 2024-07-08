+++
title = "技術ブログを始めた話"
date = 2024-07-07
description = "技術ブログをSSGでほとんど無料で始めたことについて書かれている記事。テーマのカスタマイズも行っている。"

[extra]
comment = true

[taxonomies]
tags = ["meta/blog"]
+++

このポストはあなたが今見ているブログを始めた時の記録。

### ドメイン

ドメインはCloudflare Registerで取っている。Cloudflareのドメインは安い。

### 本体

このブログは(フッターを見てもらっても分かるが)Rust製のSSGの[Zola](https://getzola.org/)でMarkdownと[Tera](https://keats.github.io/tera/)のテンプレートによって生成されている。テーマは[anemone](https://github.com/Speyll/anemone)をフォークしたものが使われている。

#### テーマの変更点

- CSS
  - 日本語フォントを追加
  - 読み込みを高速化
- HTML
  - ファビコンをSVGに出来るように

### ホスト

ホステイングサービスはCloudflare Pagesを利用している。ドメインとホストをCloudflareで統一していて同じダッシュボードから設定できて楽。

## 存在意義

目的は広告収入ではないから広告は置かない。どちらかというと誰かに見てもらってリアクションをもらうことを目標にかく。あと同じことを思った誰かの役に立てるように。

## 書くこと

主に技術系。最近ハマっていて好きな言語は

- Rust
- Elixir

だからしばらくはそのへんについてが多くなると思う。

短い(CloudflareとZolaがいいプロダクト)けどここで終わり。また
