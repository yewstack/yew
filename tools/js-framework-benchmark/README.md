# js-framework-benchmark

This is a simple benchmark for several javascript frameworks. The benchmarks creates a large table with randomized entries and measures the time for various operations including rendering duration.

## About the benchmarks

The following operations are benchmarked for each framework:

* create rows: Duration for creating 1,000 rows after the page loaded (no warmup).
* replace all rows: Duration for replacing all 1,000 rows of the table (with 5 warmup iterations).
* partial update: Time to update the text of every 10th row for a table with 10,000 rows (with 5 warmup iterations).
* select row: Duration to highlight a row in response to a click on the row. (with 5 warmup iterations).
* swap rows: Time to swap 2 rows on a table with 1,000 rows. (with 5 warmup iterations).
* remove row: Duration to remove a row for a table with 1,000 rows. (with 5 warmup iterations).
* create many rows: Duration to create 10,000 rows (no warmup)
* append rows to large table: Duration for adding 1,000 rows on a table of 10,000 rows (no warmup).
* clear rows: Duration to clear the table filled with 10,000 rows. (no warmup)
* ready memory: Memory usage after page load.
* run memory: Memory usage after adding 1,000 rows.
* update memory: Memory usage after clicking 5 times update for a table with 1,000 rows.
* replace memory: Memory usage after clicking 5 times create 1,000 rows.
* repeated clear memory: Memory usage after creating and clearing 1,000 rows for 5 times.
* update memory: Memory usage after clicking 5 times update for a table with 1,000 rows.
* startup time: Duration for loading and parsing the javascript code and rendering the page.
* consistently interactive: The lighthouse metric TimeToConsistentlyInteractive: A pessimistic TTI - when the CPU and network are both definitely very idle. (no more CPU tasks over 50ms)
* script bootup time: The lighthouse metric ScriptBootUpTtime: The total ms required to parse/compile/evaluate all the page's scripts
* main thread work cost: The lighthouse metric MainThreadWorkCost: Total amount of time spent doing work on the main thread. includes style/layout/etc.
* total byte weight: The lighthouse metric TotalByteWeight: Network transfer cost (post-compression) of all the resources loaded into the page.

For all benchmarks the duration is measured including rendering time. You can read some details on this [article](http://www.stefankrause.net/wp/?p=218).

## How to get started - building and running

There are 3 framework entries in this repository:

* `keyed/wasm-bindgen`
* `keyed/yew`
* `keyed/yew-baseline` _(Used to determine if recent changes improved performance)_

### Prerequisites

#### Install Node

Have *node.js (>=10.0)* installed. We recommend using `nvm` to manage node versions The benchmark has been tested with node v12.16.2.
Please make sure that the following command work before trying to build:
```
> npm
npm -version
6.14.15
> nvm current
v12.16.2
> node --version
v12.16.2
```

#### Install `chromedriver`

https://chromedriver.chromium.org/downloads


#### Install local server

```
cargo install https
```

Make sure port `8080` is not taken.

### Run benchmarks

```
./benchmark_change.sh $CHANGED_YEW_PROJECT
```
where `$CHANGED_YEW_PROJECT` is the path to the root of the yew project you have
your changes in.
