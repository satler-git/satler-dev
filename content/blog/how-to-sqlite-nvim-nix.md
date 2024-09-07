
+++
title = "Nix上のNeovimでSQLiteを実行する方法"
date = 2024-09-07
description = "NeovimでSQLiteを使用するプラグインを実行させるときに少し詰まったのでメモ"

[extra]
comment = true

[taxonomies]
tags = ["neovim"]
+++

## 問題

[kkharji/sqlite.lua](https://github.com/kkharji/sqlite.lua)を使用しているプラグインを使っているときに`libsqlite3.so`が見つからないとエラーが出た。

## 解決方法

sqliteをインストールして

```nix
"let g:sqlite_clib_path = '${pkgs.sqlite.out}/lib/libsqlite3.so'"
```

を読み込めるようにする(気づいていなかったけど普通にリポジトリに書いてあった。。。)。
luaだと

```nix
"vim.g.sqlite_clib_path = '${pkgs.sqlite.out}/lib/libsqlite3.so'"
```

自分はまだNeovimはhome-managerで管理できていないから、`home.file`でtextからファイルを作ってそれを`init.lua`から読み込んだ。

## おまけ(何故気づかなかったのか)

エラーにばっかり気を取られてインストールしたのが昔だからGithubを見るのを忘れていた。

