---
title: Properties
description: Parent to child communication
---
プロパティは、子コンポーネントと親コンポーネントが互いに通信できるようにします。

## マクロの継承

`Properties`を自分で実装しようとせず、代わりに`#[derive(Properties)]`を使ってください。

:::note
`Properties`を継承した型は`Clone`を実装していなければいけません。
これは`#[derive(Properties, Clone)`か`Clone`を手で実装することで可能です。
:::

### 必要な属性

デフォルトでは、`Properties` を導出する構造体内のフィールドは必須です。
フィールドが欠落していて `html!` マクロでコンポーネントが作成された場合、コンパイラエラーが返されます。
オプションのプロパティを持つフィールドについては、`#[prop_or_default]` 属性を使用して、propが指定されていない場合はその型のデフォルト値を使用します。
値を指定するには `#[prop_or(value)]` 属性を用います。
ここでvalueはプロパティのデフォルト値、あるいは代わりに `#[prop_or_else(function)]` を使用して、`function` はデフォルト値を返します。
例えば、ブール値のデフォルトを `true` とするには、属性 `#[prop_or(true)]` を使用します。オプションのプロパティでは、デフォルト値 `None` を持つ `Option` 列挙型を使うのが一般的です。

### PartialEq

もし可能ならpropsで `PartialEq` を継承するのが良いかもしれません。
`PartialEq`を使うことで、不必要な再レンダリングを避けることができます
(これについては、**最適化とベストプラクティス**のセクションで説明しています)。

## プロパティを使用する際のメモリと速度のオーバーヘッド

`Compoenent::view`ではコンポーネントの状態への参照を取り、それを使って `Html` を作成します。
しかし、プロパティは自身の値です。
つまり、それらを作成して子コンポーネントに渡すためには、`view` 関数で提供される参照を所有する必要があるのです。
これは所有する値を取得するためにコンポーネントに渡される参照を暗黙のうちにクローンすることで行われます。


これは、各コンポーネントが親から受け継いだ状態の独自のコピーを持っていることを意味し、コンポーネントを再レンダリングするときはいつでも、再レンダリングしたコンポーネントのすべての子コンポーネントのpropsがクローンされなければならないことを意味します。

このことの意味するところは、もしそうでなければ_大量の_データ \(10KBもあるような文字列\) をpropsとして渡してしまうのであれば、子コンポーネントを親が呼び出す `Html` を返す関数にすることを考えた方がいいかもしれないということです。

propsを介して渡されたデータを変更する必要がない場合は、実際のデータそのものではなく、データへの参照カウントされたポインタのみが複製されるように `Rc` でラップすることができます。

## 例

```rust
use std::rc::Rc;
use yew::Properties;

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

impl Default for LinkColor {
    fn default() -> Self {
        // The link color will be blue unless otherwise specified.
        LinkColor::Blue
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// The link must have a target.
    href: String,
    /// If the link text is huge, this will make copying the string much cheaper.
    /// This isn't usually recommended unless performance is known to be a problem.
    text: Rc<String>,
    /// Color of the link.
    #[prop_or_default]
    color: LinkColor,
    /// The view function will not specify a size if this is None.
    #[prop_or_default]
    size: Option<u32>,
    /// When the view function doesn't specify active, it defaults to true.
    #[prop_or(true)]
    active: bool,
}
```

