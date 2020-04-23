# route! macro

## Terms
* matcher string - The string provided to the `route!` macro. This string has a special syntax that defines how it matches route strings.
* route string - The section of the URL containing some combination (not necessarily all) of path query and fragment.
* matcher - The struct produced by the `route!` macro.
* path - The part of the url containing characters separated by `/` characters.
* query - The part of the url started by a `?` character, containing sections in the form `this=that`, with additional sections taking the form `&this=that`.
* fragment - The part of the url started by a `#` and can contain unstructured text.
* **any** - A section delimited by `{}` and controls capturing or skipping characters.
  * **capture** - An any section that contains a alphabetical identifier within ( eg. `{capture}`). That identifier is used as the key when storing captured sections in the `Matches`.
  * `Matches` - An alias to `HashMap<&str, String>`. Captured sections of the route string are stored as values, with their keys being the names that appeared in the capture section.
* **optional** - Denotes a part of the route string that does not have to match.
* **literal** - A matching route string must these sections exactly. These are made up of text as well as special characters.
* special characters - ` /?&=#`, characters that are reserved for separating sections of the route string.
* flags - extra keywords you can specify after the matcher string that determine how it will the matcher will behave.

## Description

The `route!` macro is used to define a matcher for a `Route`.
It accepts a matcher string and a few optional flags that determine how the matcher behaves.
The matcher string has a specific syntax that the macro checks at compile time to make sure that you won't encounter an error at runtime when the `Router` fails to properly parse a malformed matcher string. 

You don't have to use the macro though. 
You can opt to use the parser at runtime instead, or construct a vector of `MatcherTokens` yourself, although this isn't recommended.


The parser tries to ensure that these extensions to the URL produce tokens that can be used to match route strings in a predictable manner, and wont parse "improperly" formatted URLs.

Examples of URLs that the parser attempts to avoid parsing successfully include:
* Instances of double slashes (`/route/to/a//thing`)
* Empty or incomplete queries (`/route?query=` or (`/route?query`)
* Missing queries (`/route?&query=yes`)
* Path sections not starting with a slash (`im/not/really/a/route`)

To do this, the parser is made up of rules dictating where you can place any and optional sections.

### Optional
The optional section, being relatively simple in its operation, is defined mostly by where you can and cannot place them.
* The router first attempts to parse a path, then the query, then the fragment.
Optional sections are not allowed to cross these boundaries.
* To avoid the parsing double slashes in the path section, optional sections have to start with a `/` and contain either a literal, any, or a nested _optional_ if they are in a path section, and can't come immediately after a `/`, nor can a `/` come after them.
In practice, this looks like: `/a/path/to(/match)`
* Optional sections within path sections can only appear at the end of path sections.
You can't have a literal part come after an optional part.
  * This means that `/a/path/(/to)(/match)` is valid.
  * So is `/a/path/(/to(/match))` is also valid.
  * But `/a/path(/to)/match` is not.
* Optional sections within a query can take a few forms:
  * `?(query=thing)`
  * `?(query=thing)(query=another_thing)`
  * `(?query=thing)`
  * `?query=thing(&query=another_thing)`
  * `?query=thing(&query=another_thing)(&query=another_thing)`
* Optional sections for fragments are generally pretty flexible
  * `(#)`
  * `#(anything_you_want_here_bud)`
  * `(#anything_you_want_here_bud)`
  
### Any
Parts delimited by `{}` can match multiple characters and will match up until the point where the parser can identify the next literal, or if one cannot be found, the end of the route string.

They can appear anywhere in paths, even between non-`/` characters like `/a/p{}ath`.
They can appear in the right hand part of queries: `/path/?query={}`.
And can be interspersed anywhere in a fragment: `#frag{}ment{}`.

* There are many types of `{}` sections.
  * `{}` - This will match anything, but will be terminated by a special character `/` TODO are there other characters that can stop this matching?
  * `{*}` - This will match anything and cannot be stopped by a `/`, only the next literal. This is useful for matching the whole route string. This and its named variant can appear at the beginning of the matching string.
  * `{4}` - A pair of brackets containing a number will consume that many path sections before being terminated by a `/`. `{1}` is equivalent to `{}`.
  * `{name}` -  The named capture variant will match up to the next `/` or literal, and will add the string it captured to the `Matches` `HashMap` with the key being the specified string ("name" in this case).
  * `{*:name}` - Will match anything up until the next specified literal and put the contents of its captures in the `Matches`.
  * `{3:name}` - Will consume the specified number of path sections and add those contents to the `Matches`.
  
### Flags

* `CaseInsensitive` - This will make the literals specified in the matcher string match both the lower case and upper case variants of characters as they appear in the route string.
* `Strict` - By default, as part of an optimization step, an optional `/` is appended to the end of the path if it doesn't already have one. Setting this flag turns that off.
* `Incomplete` - By default, a route will not match unless the entire route string is matched. Enabling this flag allows the matcher to succeed as soon as all segments in the matcher are satisfied, without having to consume all of the route string.


## Note
The exact semantics and allowed syntax of the matcher string aren't fully nailed down yet.
It is likely to shift slightly over time.
If you find any inconsistencies between this document and the implementation, opening an issue in the YewRouter project would be appreciated. 