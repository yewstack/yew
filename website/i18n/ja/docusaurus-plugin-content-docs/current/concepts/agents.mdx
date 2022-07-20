---
title: Agents
description: Yew's Actor System
---

エージェントはAngularの[サービス](https://angular.io/guide/architecture-services)に似ており\(ただし依存性インジェクションはありません\)、
[アクターモデル](https://en.wikipedia.org/wiki/Actor_model)を提供します。
エージェントはコンポーネント階層のどこに位置するかに関わらず、コンポーネント間でメッセージをルーティングしたり、共有状態を作成したり、UIをレンダリングするメインスレッドから計算量の多いタスクをオフロードするために使用することができます。
また、Yew アプリケーションがタブをまたいで通信できるようにするためのエージェントのサポートも\(将来的には\)計画されています。

エージェントが並行に動くようにYewは[web-workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers)を使用しています。

## ライフサイクル

![エージェントのライフサイクル](https://user-images.githubusercontent.com/42674621/79125224-b6481d80-7d95-11ea-8e6a-ab9b52d1d8ac.png)

## エージェントの種類

### Reaches

* Context - Contextエージェントのインスタンスは、常に最大1つ存在します。
  Bridgesは、UIスレッド上で既にスポーンされたエージェントをスポーンするか、接続します。
  これは、コンポーネントまたは他のエージェント間の状態を調整するために使用することができます。
  このエージェントにBridgesが接続されていない場合、このエージェントは消滅します。

* Job - 新しいブリッジごとにUIスレッド上で新しいエージェントをスポーンします。
  これは、ブラウザと通信する共有されているが独立した動作をコンポーネントの外に移動させるのに適しています。
  (TODO 確認) タスクが完了すると、エージェントは消えます。

* Public - Contextと同じですが、独自のweb workerで動作します。

* Private - Jobと同じですが、独自のweb workerで動作します。

* Global \(WIP\)

## エージェントとコンポーネントのやり取り

### Bridges

Bridgeは、エージェントとコンポーネント間の双方向通信を可能にします。
また、Bridgeはエージェント同士の通信を可能にします。

### Dispatchers

Dispatcherは、コンポーネントとエージェント間の一方向通信を可能にします。
Dispatcherは、コンポーネントがエージェントにメッセージを送信することを可能にします。

## オーバーヘッド

独自の独立したweb worker(プライベートとパブリック)にあるエージェントは、送受信するメッセージにシリアライズするオーバーヘッドが発生します。
他のスレッドとの通信には[bincode](https://github.com/servo/bincode)を使用するので、関数を呼び出すよりもコストはかなり高くなります。
計算コストがメッセージの受け渡しコストを上回る場合を除き、ロジックを UI スレッドエージェント\(JobまたはContext\)に格納する必要があります。

## 参考資料

* [pub\_sub](https://github.com/yewstack/yew/tree/master/examples/pub_sub)の例でコンポーネントがどのようにエージェントと通信させているかがわかります。