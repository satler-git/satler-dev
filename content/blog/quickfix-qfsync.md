+++
title = "QfSync.nvim: QuickFixのバッファとのずれをなくして便利に使う"
date = 2025-09-08
description = "About blog post"

[extra]
comment = true

[taxonomies]
tags = ["Vim駅伝", "Neovim", "Vim"]
+++

この記事は[Vim駅伝](https://vim-jp.org/ekiden/)の2025-09-08の記事です。
Vim駅伝は常に参加者を募集しています。詳しくは[こちらのページ](https://vim-jp.org/ekiden/about/)をご覧ください。

前回の記事は [s-show](https://github.com/s-show) さんの [Neovim の小技集 - モード判定からカラーハイライトまで](https://kankodori-blog.com/post/2025-09-05/) でした。

# 始めに

皆さんはVimの機能の一つであるQuickFixを使っていますか？

私は、リファクタリングしたい箇所を `:vimgrep` だったりFuzzy Finderで検索して便利に使っています。
リファクタリングしたい箇所をQuickFix上にリストアップできたら、ジャンプしてリファクタリングを実行していきます。

しかし編集した結果、行数が変わると同じファイルのそれ以降のQuickFixの要素のジャンプ先がずれて少し面倒な思いをすることがあります。
これをプラグインの力で解決したい！！

ところで、Neovimにはextmark (`:h extmarks`) という機能があります。

[【Neovim】好きな位置にテキストを埋め込んだりハイライトできる「ExtMark」の使い方](https://www.rasukarusan.com/entry/2021/08/22/202248)

extmarkはなんと、編集に追従して位置を保持することが出来ます(どういうことか分からない方は上の記事のgifを見てください)。
それを使ってQuickFixの要素の位置を保存してみようと思います。

# 作った

extmarkを使ってQuickFixの位置を保存するプラグインを作りました。

[satler-git/qfsync.nvim](https://github.com/satler-git/qfsync.nvim)

## 簡単な使い方

```lua
local qfsync = require("qfsync")

qfsync.add_marks() -- 要素に対応するextmarkを作成
qfsync.sync() -- extmarkから位置を復元

qfsync.sync_all() -- 上記の両方を実行
```

これだけです。キーマップにするなり、autocmdにするなりして使えます。

(`BufEnter` に設定したら他のプラグインと競合して?(まだ原因があいまい) 要素が消えることがありました。もし再現できたら教えてください)

## ポイント

QuickFixには `user_data` というカスタムデータを保持できるフィールドがあります。
その中に `ext_id` というフィールドを作ってそこにextmarkのIDを保存しています。

# 終わりに

そもそもこれが初めてのプラグインでした。そのためLuaプラグインの構造や作法などが分からず、戸惑いました。
とっかかりは少し難しかったですが、ごちゃごちゃやっていたら最終形は意外と小さくてシンプルな実装になりました。
これがVimか〜という感じです。

また、ヘルプがめちゃくちゃありがたかったです。なんだかんだ使いこなせてない気がするのでもっと使いこなしたいです。

