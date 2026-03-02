# Website

This website is built using [Docusaurus 2](https://docusaurus.io/), a modern static website generator.

Most of the content sits inside the [docs](docs) folder and the [versioned_docs](versioned_docs) folder in the form of
markdown.

## Installation

```console
npm install
```

## Local Development

```console
npm start
```

This command starts a local development server and opens up a browser window. Most changes are reflected live
without having to restart the server.
Note this only builds for English locale unlike a production build.

> Documentation is written in `mdx`, a superset of markdown empowered with jsx.
> JetBrains and VSCode both provide MDX plugins.

## Testing

```console
cargo make website-test
```

[`website-test`](../tools/website-test) is a tool to test all code blocks in the docs as Rust doctests.
It gathers the Rust code blocks automatically, but by default they're all tested separate. In case of a 
walkthrough, it makes more sense to combine the changes described in the blocks & test the code as one.
For this end `website-test` scans all doc files for a special flag:

```html
<!-- COMBINE CODE BLOCKS -->
```
If a file ends with this specific comment (and an optional newline after it), all code blocks will be
sown together, with respect to the diff markers in them. For example:

```md
\`\`\`rust
fn main() {
    println!("Hello, World");
}
\`\`\`

\`\`\`rust
fn main() {
-   println!("Hello, World");
+   println!("Goodbye, World");
}
\`\`\`

<!-- COMBINE CODE BLOCKS -->
```

Will be tested as:
```rust
fn main() {
    println!("Goodbye, World");
}
```

:::warning
The current implementation only uses the code before the diff or the code to remove as context,
so make sure there's enough of it. The test assembler will tell you if there isn't.
:::

While assembling the code blocks, the test assembler will put special meaning into a code
line `// ...`. This line tells the test assembler to disregard any previous context for applying a diff

## Production Build

```console
npm run build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

## Localization

Localization is done on [GitLocalize](https://gitlocalize.com/repo/7052/whole_project).
You can sign in with your GitHub account to start translating.

When you add to the docs in the `mdx` files,  
Contributors on GitLocalize will translate the added content
and the translation will get dumped under the [i18n](i18n) folder in a later time.

If you are a native speaker of one of the translated languages,
and you are interested in translating your edits yourself,
you are welcome to navigate to the folder and do it yourself!

### Localizing headings

If you want to write displayed content in `html/jsx` instead of vanilla markdown,
You should wrap your text in `<Translate/>` tags.
It helps docusaurus to extract those texts and compile them to `.json` files to
get further translated in GitLocalize.

```jsx
import Translate from '@docusaurus/Translate'

<h2>
    <Translate id="header.translation.id" description="the heading description">
        This heading will be translated
    </Translate>
</h2>
```

If your pull request adds new `<Translation>` tags,
make sure you do `npm run write-translations` to generate the new stubs for later localization.
And you are always welcome to add localization yourself in your native languages!

### Common issues in localization

Pages (.mdx) are translated one-to-one and the english text is used as fallback if no translation
exists. Sometimes, when building you might see a warning, and subsequent error,
like this

> [WARNING] Docs markdown link couldn't be resolved: (../components/refs.mdx) in
> <omitted>/yew/website/versioned_docs/version-0.18.0/concepts/html/events.mdx for version 0.18.0

This means that the _non-translated_ page at `versioned_docs/version-0.18.0/concepts/html/events.mdx`
contains a relative link - `../components/refs.mdx` - to a page that _has_ been translated.
Change the link to be relative to the doc root folder, in this case `concepts/components/res.mdx`, or,
if you find the time, also translate the offending page.
