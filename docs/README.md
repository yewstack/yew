# Contributing to the documentation

Firstly, thanks for considering contributing to Yew. We're a friendly community of humans who 
collaborate to build (hopefully) awesome software. We try to write software which is easy to use so
we ask that you follow a few guidelines when contributing – these are laid out in this document.

Note that this document is about *contributing documentation* not contributing code and only 
applies to the contents of the `docs` folder – not any other parts of the project.

## Can I just submit a PR or do I need to create an issue first?

PRs not attached to previously created issues are welcome *if* they're a small change. This could
be something like fixing a small grammar mistake or a typo.

If your PR is something bigger create an issue beforehand. This will save everyone time and effort:

1. Multiple people don't end up submitting duplicate PRs doing the same thing.
2. People have a chance to submit feedback on an idea *before* someone does a lot of work on it.
3. It's easy to track progress on the development of the documentation.

## Spelling and grammar

We recognise that not everyone who contributes to Yew will have "perfect" grammar or be a native
English speaker. We're all human; everyone makes mistakes and nobody will look down on you if you 
make typos or grammar mistakes (we'll just fix them, merge PRs and move on with life).

To help catch spelling mistakes, we use a spellchecking script which originally came from the Rust
Book. If it picks up a spelling "mistake" which isn't actually a mistake, then please add it to the
list in `ci/dictionary.txt` (in alphabetically sorted order).

If in doubt about spelling, grammar or style you might find it useful to consult the 
[Microsoft Style Guide](https://docs.microsoft.com/style-guide/) which we sometimes use as a handy
reference.

## Line wrap
Having really long lines makes it hard to review code and see differences between versions. To 
solve this problem all lines should be wrapped at 100 characters.

If you're editing a line which is more than 100 characters long, please feel free to shorten it!