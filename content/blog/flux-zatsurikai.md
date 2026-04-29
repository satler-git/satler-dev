+++
title = "Protoflu(x)の実行と値の同期に関するメモ"
date = 2026-04-30
description = ""

[extra]
comment = true

[taxonomies]
tags = ["Resonite"]
+++

この記事は筆者の雑な理解で構成されています！！
"正しい"情報が必要な方はResonite Wikiの [Impulses](https://wiki.resonite.com/Impulses) の項などを参照してください。

# Resoniteとは

ソーシャルVR(VRがなくてもできる)。ゲームの中でいろいろな物を作れることが特徴。Protofluxというノードベースのプログラミング言語でいろいろできる。
また、P2Pでそれぞれのセッション(VRCでいうインスタンス)にはHostがいる。

# Impulsesについて

Protofluxには、Impulsesという概念がある。これは命令的なプログラミングをするためのもので、ノード間を伝わる信号のような概念。Impulsesが来たノードはそのタイミングで実行(例えば、値を書き込むなど)される。
Impulsesはそれを発火した人(例えばボタンを押した人)のコンピューターで1ゲームティックの内に実行される。複数のゲームティックをまたいだ処理を作るにはAsyncを使う必要がある。
Resoniteでなにかしら処理を行なった後にアウトプットをするには、なにかしら(例えば、変数など)に値を書き込む必要がある(HTTP RequestをしたりWebSocketを使う場合は別)。Impulsesを使う場合、そのために [Write](https://wiki.resonite.com/ProtoFlux:Write) ノードを使う。これについてはもう少し詳しく扱う。

Impulsesを使わずに宣言的なプログラムを作ることもできる。`+` などのオペレーターはImpulsesを受け取らない。この場合も、`Fire on True` などで、Impulsesを発生させられる。
Impulsesを使わない場合、アウトプットにはDriveという機能を使う。これは決まった実行タイミングがある訳ではなく、常に値を書き込み続ける。常に書き込み続けるのでDriveは排他的で、Driveされているフィールドは他の場所でWriteなどによって書き込むことはできない。

通常、両者を組み合わせて使う。

# 同期について

基本的に同期は1つのImpulse(Contextともいわれる)が終了したタイミングで行なわれる(つまり、一つのImpulseで複数回Writeしても送信されるのは最後の値だけ)。なので、Driveは同期されずユーザーによって値が異なることがある。


| 変数の種類 | 特徴 | 同期されるか |
| ---------- | ---- | ------------ |
| フィールド | コンポーネントやスロットに関連付けられた値 | Drive以外ははい |
| Local | ひとつのImpulseごとに初期化されて、終了すると破棄される | いいえ |
| Store | Impulseをまたいで、ユーザーごとに初期化される | いいえ |
| DataModelStore | Impulseとネットワークをまたぐ | はい |
| Dynamic Variables| パス(変数名)を変えることで動的に変数を作成できる。Impulseをまたぐ | はい |
| Cloud Variables | ワールドやアイテムに関連付けられるのでは**なく**ユーザーやグループに関連付けられる | はい |


