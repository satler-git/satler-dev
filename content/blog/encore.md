+++
title = "Encore freefordevシリーズ1"
date = 2024-01-01
description = "freefordevに載っていたPaaS又はクラウドを試していくシリーズの一つめ。encoreというサービスについての記事。試しにRust製のツールをデプロイする。"

[extra]
comment = true

[taxonomies]
tags = ["freefordev/cloud"]
+++

お金がない！！
ということで[freefordev](https://free-for.dev/#/)を見て試していくシリーズを書く。
第一弾は Paas の[Encore](https://encore.dev/)。

## Encore とは

ホームページには

> Encore is the Development Platform for building event-driven and distributed systems. Move faster with purpose-built local dev tools and DevOps automation for AWS/GCP.

と書いてある。訳すとこんな感じ。

> Encore はイベント駆動な分散システムを開発するためのプラットフォームです。専用のローカルな開発ツールと AWS/GCP のための DevOps 自動化でもっと速く

その下には

```powershell
iwr https://encore.dev/install.ps1 | iex
```

とある。

## 試す

[Quick start](https://encore.dev/docs/quick-start)に沿って進めたい。
上のワンライナーを Powershell で実行する。

そのあと

```powershell
encore app create
```

を実行する。そうするとアカウントを作るか聞かれて、そのあと Go か TS かを選ばされる。
Go は全く使えないのに対し、TS は殆ど使えないので TS で進める。

{% info() %}
正直、この時点で刺さらないことは結構確定しているので終ってもいいが、Encore の日本語情報が無いので続ける
{% end %}

テンプレートは Typescript の Hello world(Simple REST API という名前になっていた)を選んだ。
`hello/hello.ts`を見る。コメントを除くと以下のようになっている。

```ts
import { api } from "encore.dev/api";

export const get = api(
  { expose: true, method: "GET", path: "/hello/:name" },
  async ({ name }: { name: string }): Promise<Response> => {
    const msg = `Hello ${name}!`;
    return { message: msg };
  }
);

interface Response {
  message: string;
}
```

比較的分かりやすい。`get`というパブリックな関数を定義している。
非同期の無名関数と API の情報を Encore の SDK の api 関数に投げると API を登録できるようだ。
実行するには

```powershell
encore run
```

を実行する。そうすると開発ツールと API のエンドポイントがローカルで立ちあがる。

{{ img(id="https://res.cloudinary.com/dsexsi1cq/blog/encore/qehpjkwye6qptdtxisnx.png", alt="Encoreのローカルの開発ダッシュボード。左側(画面の1/5程度)は黒く、右側は白い。右側の左側には作ったAPIが並んでいる。右側の右側にはTRACEと書かれておりログが見れる。", caption="Encoreのダッシュボード", height="1712", width="992")}}
{{ img(id="https://res.cloudinary.com/dsexsi1cq/blog/encore/ex8lvcq1ouyu1xldsjfv.png", alt="encore runを実行してコマンドの結果が表示されている。ダッシュボードのurlとAPIのurlが表示されている。またヒントとして作ったAPIの呼び出し方の例もある。", caption="encore run", height="216", width="523") }}

開発ツールからも API を呼べる。

{{ img(id="https://res.cloudinary.com/dsexsi1cq/blog/encore/ztwvor459vlkv58l4cg2.png", alt="先ほどの説明に加え右側の左側の並んでいるAPIの下にAPIが呼び出されており結果とログが表示されている。TRACEには一回helloが呼ばれ成功したことが表示されている", caption="APIをEncoreのダッシュボードから呼んでいる様子", height="1616", width="897") }}

このあと Quick start ではデプロイを始めているが、この記事では試しにもう一個 API を書いてみる。ちなみに`encore run`はホットリロードにも対応している。

こんな感じになった。`/cat/world3`とすると`Meow world! Meow world! Meow world!`が帰ってくる。`log`は比較的使いやすかった。

```ts
import log from "encore.dev/log";

export const cat = api(
  { expose: true, method: "GET", path: "/cat/:param" },
  async ({ param }: { param: string }): Promise<Response> => {
    log.info(`${param}`, { is_subscriber: true });
    const num: number = Number(param.match(/\d+$/));
    if (Number.isNaN(num)) {
      throw new Error("The last letter is not a number.");
    }
    const name = param.split(/\d+$/)[0];

    const msg = Array(num).fill(`Meow ${name}!`).join(" ");
    return { message: msg };
  }
);
```

### デプロイ

本来なら専用のGithubリポジトリとリンクしてmainにpushした時に自動でデプロイしてくれるようだ。しかし、開始時に「なんかリポジトリできてる、消すか」と思って消してsandbox内にある。しかも見た感じrootにないとダメそう。`encore.app`などのファイルを作ることでビルドプロセスをカスタマイズできる模様。いろいろ見ていたがダメそうなので諦める。

### 値段

無料プランでは以下の通り。([ここ](https://encore.dev/docs/about/usage)から。変わっている可能性あり)

| | アプリケーション毎 |
| - | - |
| リクエスト | 100,000 / 日 |
| データベース容量 | 1 GB |
| PubSubメッセージ | 100,000 / 日 |
| Cron| 1時間に一回 |

結構安い。安すぎるくらいだと思う。(この表ダサいな)

## 所感

所感は以下の通り。

- 嬉しいところ
  - 開発体験がいい
    - ダッシュボードがサーバー側とクライアント側を両方いじれて使いやすい
    - SDKで比較的シンプルにAPIがかける
- つらいところ
  - 使える言語が少ない
    - これは人による
    - RustとかならWASMで頑張れば勝手に使えるようにできそう
  - SDK
    - Encoreが倒れたり、他のPaaSにAPIを移行しようとなったときに大変

そこまで大きいプロジェクトではなく、GoかTypescriptを採用できるなら刺さると思う。自分はどちらの言語もほとんどかけない(上のcatでも20分くらい調べながら作った)からめっちゃ刺さったというわけではない。今回のソースコードはGithubの[satler-git/sandbox](https://github.com/satler-git/sandbox/tree/main/typescript/hello-encore)の中で見れます。
