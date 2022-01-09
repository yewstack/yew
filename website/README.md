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

## Production Build

```console
npm run build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

## Localization

Localization is done on [GitLocalize](https://gitlocalize.com/repo/4999/whole_project).
You can sign in with your GitHub account to start translating.

When you add to the docs in the `mdx` files,  
Contributors on GitLocalize will translate the added content
and the translation will get dumped under the [i18n](i18n) folder in a later time.

If you are a native speaker of one of the translated languages,
and you are interested in translating your edits yourself, 
you are welcome to navigate to the folder and do it yourself!

If you want to write displayed content in `html/jsx` instead of vanilla markdown,
You should wrap your text in `<Translate/>` tags.
It helps docusaurus to extract those texts and compile them in `.json` files to
get further translated in GitLocalize.

```jsx
import Translate from '@docusaurus/Translate';

<h2>
    <Translate id="header.translation.id" description="the header description">
        This header will be translated
    </Translate>
</h2>;
```

If your pull request adds new `<Translation>` tags,
a workflow in CI will generate the json files and commit to your branch directly.
If you have followup commits, don't panic and merge the generated json files.
You can also run `node ../ci/write-translations.js` from the `website` directory and generate stubs yourself,
and you are also welcome to add localization in your own language!
