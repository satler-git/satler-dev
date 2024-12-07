+++
title = "SurrealDBのTutorialsをやってみる"
date = 2024-10-04
description = "About blog post"

[extra]
comment = true

[taxonomies]
tags = ["Database"]
+++

SurrealDBはRustで書かれたdocument-graphなデータベース。Githubの[Repo](https://github.com/surrealdb/surrealdb)。初めてなら[Getting started](https://surrealdb.com/docs/surrealdb/introduction/start)から始めろと言われたので、そこから始める。

# Getting started

インメモリでデータベースを始めるには以下のようにする。

```powershell
surreal start memory -A --user root --pass root
```

SurrealDBのクエリにはSurrealQLという言語を使う(スニペットに色がついていなくて申し訳ない)のだが、ここではそれのためのかっこいいGUIのSurrealistを使う。

{{ img(id="https://res.cloudinary.com/dsexsi1cq/blog/surrealdb/ezlfih4fsw3btxe9bjlb.png", alt="左側五分の一程度の縦分割の画面とその他横分割の3つの画面です。横の上にはクエリが書ける場所があり下にはリザルトが出てくる場所があります", caption="surreal start", height="505", width="925") }}

以下のクエリを実行する。

```SurrealQL
CREATE account SET
  name = 'ACME Inc',
  created_at = time::now()
;
```

そうすると以下のような~~JSONまがいなもの~~JSONが出力される。

```
[
  {
    created_at: '2024-08-28T07:57:08.126Z',
    id: account:7tc1347h7y04vkxrv0v6,
    name: 'ACME Inc'
 }
]
```

上のクエリは、`account`というテーブルに新しいレコードを作成するもの。(テーブルにはスキーマフルとスキーマレスがあるよう。)主キーのidは指定しなければランダムだが、`table:id`のように指定して発行することもできる。

```SurrealQL
CREATE author:john SET
  name.first = 'John',
  name.last = 'Adams',
  name.full = string::join(' ', name.first, name.last),
  age = 29,
  admin = true,
  signup_at = time::now()
;
```

また、`table:id`の形で指定することで、リンクできる(document-graph要素)。

```SurrealQL
CREATE article SET
  created_at = time::now(),
  author = author:john,
  title = 'Lorem ipsum dolor',
  text = 'Donec eleifend, nunc vitae commodo accumsan, mauris est fringilla.',
  account = (SELECT VALUE id FROM account WHERE name = 'ACME Inc' LIMIT 1)[0]
;
```

`SELECT`はSQLのように一つのテーブルから取得することも、複数のテーブルから指し示しているデータをまとめて取得することもできる。

{{ img(id="https://res.cloudinary.com/dsexsi1cq/blog/surrealdb/piwzxwz3leursfkvl1gc.png", alt="画面の構成は上の画像と同じで、複数のテーブルからSELECTさせる文とその結果が書かれています", caption="SELECT", height="505", width="925") }}

`DELETE`はテーブルからレコードを`REMOVE`はレコード以外のものを削除するための文。

# Tutorials

チュートリアルは複数用意されているようだが、今回は(ちょっとSQLライクに考えられる)[Define a Schema](https://surrealdb.com/docs/surrealdb/tutorials/define-a-schema)をやってみる。テーブルの定義とフィールドの作成は別。

```SurrealQL
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD firstName ON TABLE user TYPE string;
DEFINE FIELD lastName ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string
  ASSERT string::is::email($value);
```

`string::is::email`が面白い。文字列のバリデーションのための関数(あとからもうちょい詳しくみたい)は[stringのドキュメント](https://surrealdb.com/docs/surrealdb/surrealql/functions/database/string)のisモジュールの部分で見れる。かなり豊富。`time::now()`も関数。

レコードの追加について、ドキュメントは先ほどとは違い、`CREATE table CONTENT`を使っている。ドキュメントを見た限り`CONTENT`と`SET`の違いは書き方だけのようだ。`CONTENT`はすでにJSONまたはSurrealQLの形式になっているときに便利と書いてあった。

```SurrealQL
CREATE user CONTENT {
    firstName: 'John',
    lastName: 'Doe',
    email: 'JohnDoe@someemail.com',
};
```

メールアドレスではない形で追加しようとすると怒られる。

```SurrealQL
CREATE user CONTENT {
    firstName: 'John',
    lastName: 'Doe',
    email: 'JohnDoe.com',
};
```

ちゃんと先ほど設定した`ASSERT string::is::email($value);`が働いているようだ。

{{ img(id="https://res.cloudinary.com/dsexsi1cq/blog/surrealdb/am9a9m3ykkm3qmvwxfqg.png", alt="画面の構成は上の画像と同じで上のクエリを実行しようとした結果、Assertが失敗したとリザルト欄に書かれています", caption="CREATE(失敗)", height="505", width="925") }}

スキーマに存在しないフィールドを記載した場合は無視される。Tutorialsにはスキーマレスの方法も書かれていた。基本的にはGetting startedでやった通りなのだが面白いことをやっていた。意外となんでもレコードのIDにできるのだ。レコードのIDにできる型は[Types of Record IDs](https://surrealdb.com/docs/surrealdb/surrealql/datamodel/ids#types-of-record-ids)に書かれている。`temperature:{ location: 'London', date: time::now() }`のようにオブジェクトベースのIDも作れるようだ。
これは構造や責任範囲をちゃんと考えて作らないととんでもないことになりそうだ。

### 関数

組み込みの関数は[ドキュメント](https://surrealdb.com/docs/surrealdb/surrealql/functions/database)で見れるほか、v2からは[匿名関数](https://surrealdb.com/docs/surrealdb/surrealql/datamodel/closures)も作れるようになるようだ。HTTPリクエストを送ったり、SHA256を取ったりできる。

### Futures

あと[ドキュメント](https://surrealdb.com/docs/surrealdb/surrealql/datamodel/futures)を見ていたら、Futuresという機能もあった。挿入時ではなく、SELECTされてリターンされる時に計算される。例示されていたのは、現在が誕生日に18年足した日よりも遅いかを判定して成人を判定することができるみたいなものだった。

# 終わり

結構面白いDBだと思う。RustのSDKもある。見た感じ非同期やserdeなどを活かしてORM的な使い方もできるようになっていてけっこう使いやすそうだった。すべて正確な記述ができている自信がないので詳しくはドキュメントを見てみてほしい。かなり高機能な分、どのくらいの責任をDBに任せるのかはしっかり考える必要がありそうだ。
