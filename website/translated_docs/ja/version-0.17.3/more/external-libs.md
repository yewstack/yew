---
title: External libraries
description: Libraries that can help with yew development
---

# 外部ライブラリ

### Yewtil

YewtilはYewのプログラムを書きやすくするユーティリティ集です。
含まれているのは:

* NeqAssign - 先述の通り、再レンダリングを最小化するようpropsを割り当てる方法です
* PureComponents - 状態を更新しないコンポーネント。NeqAssignを使用するとマクロの中から通常のコンポーネントのように呼び出される関数がメモ化されます。

* Lrc - リンクされたリストは、`Rc`のようにカウントされたスマートポインタ関数を参照しますが、新しいデータ更新パターンを可能にします。
* Mrc/Irc - Mutable/Immutable 参照カウントのスマートポインタは `Rc` のように機能しますが、`Mrc` に対して `DerefMut` と `BorrowMut` を実装しているため、Yew の中でより使いやすくなっています。これにより、`Mrc` を `NeqAssign` と一緒に使うことができます。`Irc` はデータに対する不変のビューとして機能するので、表示のみのタスクで使用されるデータを保持するのに理想的です。

* History - `VecDeque` を用いて、表示した過去の値を保持する履歴追跡ラッパーです。
* Futures - コンポーネントの更新ループにメッセージを送信するのをサポートします。
* Fetch - `web_sys` と前述のfuturesの機能を用いたフェッチリクエストを処理するための抽象化です。

## お探しのものは

エコシステムが必要なライブラリですが、まだありません。

Bootstrap/MaterialUi/arbitraryといったCSSフレームワークのコンポーネントのラッパー。