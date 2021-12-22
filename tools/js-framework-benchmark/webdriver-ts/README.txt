Typescript Benchmark driver
===========================

This is the indentended replacement for the java-webdriver.

Compile with:
npm install
npm run compile

Run with:
npm run selenium
You can optionally pass a list of frameworks or benchmarks that should be run.
npm run selenium -- --framework ang,bobril --benchmark run
runs all frameworks that contain the string ang or bobril and all benchmarks whose name contains run

Create the result table:
npm run results
http://localhost:8080/webdriver-ts/table.html


TODOs:
* Convert makeTable.js to Typescript
* Extract benchmark names and description to common module