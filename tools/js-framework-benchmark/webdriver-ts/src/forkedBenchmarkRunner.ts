import {WebDriver, logging} from 'selenium-webdriver'
import {BenchmarkType, Benchmark, benchmarks, fileName, LighthouseData} from './benchmarks'
import {setUseShadowRoot, buildDriver} from './webdriverAccess'

const lighthouse = require('lighthouse');
const chromeLauncher = require('chrome-launcher');

import * as fs from 'fs';
import * as path from 'path';
import {TConfig, config as defaultConfig, JSONResult, FrameworkData, ErrorAndWarning, BenchmarkOptions, BenchmarkDriverOptions, TBenchmarkStatus} from './common'
import * as R from 'ramda';

let config:TConfig = defaultConfig;

// necessary to launch without specifiying a path
var jStat:any = require('jstat').jStat;

interface Timingresult {
    type: string;
    ts: number;
    dur?: number;
    end?: number;
    mem?: number;
    evt?: any;
}

function extractRelevantEvents(entries: logging.Entry[]) {
    let filteredEvents: Timingresult[] = [];
    let protocolEvents: any[] = [];
    entries.forEach(x => {
        let e = JSON.parse(x.message).message;
        if (config.LOG_DETAILS) console.log(JSON.stringify(e));
        if (e.method === 'Tracing.dataCollected') {
            protocolEvents.push(e)
        }
        if (e.method && (e.method.startsWith('Page') || e.method.startsWith('Network'))) {
            protocolEvents.push(e)
        } else if (e.params.name==='EventDispatch') {
            if (e.params.args.data.type==="click") {
                if (config.LOG_TIMELINE) console.log("CLICK ",JSON.stringify(e));
                filteredEvents.push({type:'click', ts: +e.params.ts, dur: +e.params.dur, end: +e.params.ts+e.params.dur});
            }
        } else if (e.params.name==='TimeStamp' &&
            (e.params.args.data.message==='afterBenchmark' || e.params.args.data.message==='finishedBenchmark' || e.params.args.data.message==='runBenchmark' || e.params.args.data.message==='initBenchmark')) {
            filteredEvents.push({type: e.params.args.data.message, ts: +e.params.ts, dur: 0, end: +e.params.ts});
            if (config.LOG_TIMELINE) console.log("TIMESTAMP ",JSON.stringify(e));
        } else if (e.params.name==='navigationStart') {
            filteredEvents.push({type:'navigationStart', ts: +e.params.ts, dur: 0, end: +e.params.ts});
            if (config.LOG_TIMELINE) console.log("NAVIGATION START ",JSON.stringify(e));
        } else if (e.params.name==='Paint') {
            if (config.LOG_TIMELINE) console.log("PAINT ",JSON.stringify(e));
            filteredEvents.push({type:'paint', ts: +e.params.ts, dur: +e.params.dur, end: +e.params.ts+e.params.dur, evt: JSON.stringify(e)});
        // } else if (e.params.name==='Rasterize') {
        //     console.log("RASTERIZE ",JSON.stringify(e));
        //     filteredEvents.push({type:'paint', ts: +e.params.ts, dur: +e.params.dur, end: +e.params.ts+e.params.dur, evt: JSON.stringify(e)});
        // } else if (e.params.name==='CompositeLayers') {
        //     console.log("COMPOSITE ",JSON.stringify(e));
        //     filteredEvents.push({type:'paint', ts: +e.params.ts, dur: +e.params.dur, end: +e.params.ts, evt: JSON.stringify(e)});
        // } else if (e.params.name==='Layout') {
        //     console.log("LAYOUT ",JSON.stringify(e));
        //     filteredEvents.push({type:'paint', ts: +e.params.ts, dur: +e.params.dur, end: e.params.ts, evt: JSON.stringify(e)});
        // } else if (e.params.name==='UpdateLayerTree') {
        //     console.log("UPDATELAYER ",JSON.stringify(e));
        //     filteredEvents.push({type:'paint', ts: +e.params.ts, dur: +e.params.dur, end: +e.params.ts+e.params.dur, evt: JSON.stringify(e)});
        } else if (e.params.name==='MajorGC' && e.params.args.usedHeapSizeAfter) {
            filteredEvents.push({type:'gc', ts: +e.params.ts, end:+e.params.ts, mem: Number(e.params.args.usedHeapSizeAfter)/1024/1024});
            if (config.LOG_TIMELINE) console.log("GC ",JSON.stringify(e));
        }
    });
    return {filteredEvents, protocolEvents};
}

async function fetchEventsFromPerformanceLog(driver: WebDriver): Promise<{timingResults: Timingresult[], protocolResults: any[]}> {
    let timingResults : Timingresult[] = [];
    let protocolResults : any[] = [];
    let entries = [];
    do {
        entries = await driver.manage().logs().get(logging.Type.PERFORMANCE);
        const {filteredEvents, protocolEvents} = extractRelevantEvents(entries);
        timingResults = timingResults.concat(filteredEvents);
        protocolResults = protocolResults.concat(protocolEvents);
    } while (entries.length > 0);
    return {timingResults, protocolResults};
}

function type_eq(requiredType: string) {
    return (e: Timingresult) => e.type=== requiredType;
}
function type_neq(requiredType: string) {
    return (e: Timingresult) => e.type !== requiredType;
}

function asString(res: Timingresult[]): string {
    return res.reduce((old, cur) => old + "\n" + JSON.stringify(cur), "");
}

function extractRawValue(results: any, id: string) {
    let audits = results.audits;
    if (!audits) return null;
    let audit_with_id = audits[id];
    if (typeof audit_with_id === 'undefined') return null;
    if (typeof audit_with_id.numericValue === 'undefined') return null;
    return audit_with_id.numericValue;
}

 function rmDir(dirPath: string) {
    try { var files = fs.readdirSync(dirPath); }
    catch(e) { console.log("error in rmDir "+dirPath, e); return; }
    if (files.length > 0)
      for (var i = 0; i < files.length; i++) {
        var filePath = path.join(dirPath, files[i]);
        if (fs.statSync(filePath).isFile())
          fs.unlinkSync(filePath);
        else
          rmDir(filePath);
      }
    fs.rmdirSync(dirPath);
  };

  async function runLighthouse(framework: FrameworkData, benchmarkOptions: BenchmarkOptions): Promise<LighthouseData> {
    const opts: any = {
        chromeFlags:
        [
            "--headless",
            "--no-sandbox",
            "--no-first-run",
            "--enable-automation",
            "--disable-infobars",
            "--disable-background-networking",
            "--disable-background-timer-throttling",
            "--disable-cache",
            "--disable-translate",
            "--disable-sync",
            "--disable-extensions",
            "--disable-default-apps",
            "--window-size=1200,800",
            "--remote-debugging-port=" + (benchmarkOptions.remoteDebuggingPort).toFixed()
        ],
        onlyCategories: ['performance'],
        port: (benchmarkOptions.remoteDebuggingPort).toFixed(),
        logLevel: "info"
    };

    try {
        if (benchmarkOptions.chromeBinaryPath) opts.chromePath = benchmarkOptions.chromeBinaryPath;
        let chrome = await chromeLauncher.launch(opts);
        let results = null;
        try {
            results = await lighthouse(`http://localhost:${benchmarkOptions.port}/${framework.uri}/index.html`, opts, null);
            await chrome.kill();
        } catch (error) {
            console.log("error running lighthouse", error);
            await chrome.kill();
            throw error;
        }
        //console.log("lh result", results);

        let LighthouseData: LighthouseData = {
            TimeToConsistentlyInteractive: extractRawValue(results.lhr, 'interactive'),
            ScriptBootUpTtime: Math.max(16, extractRawValue(results.lhr, 'bootup-time')),
            MainThreadWorkCost: extractRawValue(results.lhr, 'mainthread-work-breakdown'),
            TotalKiloByteWeight: extractRawValue(results.lhr, 'total-byte-weight')/1024.0
        };
        return LighthouseData;
    } catch (error) {
        console.log("error running lighthouse", error);
        throw error;
    }
}

async function computeResultsCPU(driver: WebDriver, benchmarkOptions: BenchmarkOptions, framework: FrameworkData, benchmark: Benchmark, warnings: String[]): Promise<number[]> {
    let entriesBrowser = await driver.manage().logs().get(logging.Type.BROWSER);
    if (config.LOG_DEBUG) console.log("browser entries", entriesBrowser);
    const perfLogEvents = (await fetchEventsFromPerformanceLog(driver));
    let filteredEvents = perfLogEvents.timingResults;

    // if (config.LOG_DEBUG) console.log("filteredEvents ", asString(filteredEvents));

    let remaining  = R.dropWhile(type_eq('initBenchmark'))(filteredEvents);
    let results = [];

    while (remaining.length >0) {
        let evts = R.splitWhen(type_eq('finishedBenchmark'))(remaining);
        if (R.find(type_neq('runBenchmark'))(evts[0]) && evts[1].length>0) {
            let eventsDuringBenchmark = R.dropWhile(type_neq('runBenchmark'))(evts[0]);

            if (config.LOG_DEBUG) console.log("eventsDuringBenchmark ", eventsDuringBenchmark);

            let clicks = R.filter(type_eq('click'))(eventsDuringBenchmark)
            if (clicks.length !== 1) {
                console.log("exactly one click event is expected", eventsDuringBenchmark);
                throw "exactly one click event is expected";
            }

            let eventsAfterClick = (R.dropWhile(type_neq('click'))(eventsDuringBenchmark));

            if (config.LOG_DEBUG) console.log("eventsAfterClick", eventsAfterClick);

            let paints = R.filter(type_eq('paint'))(eventsAfterClick);
            if (paints.length == 0) {
                console.log("at least one paint event is expected after the click event", eventsAfterClick);
                throw "at least one paint event is expected after the click event";
            }

            console.log("# of paint events ",paints.length);
            if (paints.length>2) {
                warnings.push(`For framework ${framework.name} and benchmark ${benchmark.id} the number of paint calls is higher than expected. There were ${paints.length} paints though at most 2 are expected. Please consider re-running and check the results`);
                console.log(`For framework ${framework.name} and benchmark ${benchmark.id} the number of paint calls is higher than expected. There were ${paints.length} paints though at most 2 are expected. Please consider re-running and check the results`);
            }
            paints.forEach(p => {
                console.log("duration to paint ",((p.end - clicks[0].ts)/1000.0));
            })
            let lastPaint = R.reduce((max, elem) => max.end > elem.end ? max : elem, {end: 0} as Timingresult, paints);

            let upperBoundForSoundnessCheck = (R.last(eventsDuringBenchmark).end - eventsDuringBenchmark[0].ts)/1000.0;
            let duration = (lastPaint.end - clicks[0].ts)/1000.0;

            console.log("*** duration", duration, "upper bound ", upperBoundForSoundnessCheck);
            if (duration<0) {
                console.log("soundness check failed. reported duration is less 0", asString(eventsDuringBenchmark));
                throw "soundness check failed. reported duration is less 0";
            }

            if (duration > upperBoundForSoundnessCheck) {
                console.log("soundness check failed. reported duration is bigger than whole benchmark duration", asString(eventsDuringBenchmark));
                throw "soundness check failed. reported duration is bigger than whole benchmark duration";
            }
            results.push(duration);
        }
        remaining = R.drop(1, evts[1]);
    }
    if (results.length !== benchmarkOptions.numIterationsForCPUBenchmarks) {
        console.log(`soundness check failed. number or results isn't ${benchmarkOptions.numIterationsForCPUBenchmarks}`, results, asString(filteredEvents));
        throw `soundness check failed. number or results isn't ${benchmarkOptions.numIterationsForCPUBenchmarks}`;
    }
    return results;
}

async function computeResultsMEM(driver: WebDriver, benchmarkOptions: BenchmarkOptions, framework: FrameworkData, benchmark: Benchmark, warnings: String[]): Promise<number> {
    let entriesBrowser = await driver.manage().logs().get(logging.Type.BROWSER);
    if (config.LOG_DEBUG) console.log("browser entries", entriesBrowser);
    let filteredEvents = (await fetchEventsFromPerformanceLog(driver)).timingResults;

    if (config.LOG_DEBUG) console.log("filteredEvents ", filteredEvents);

    let remaining  = R.dropWhile(type_eq('initBenchmark'))(filteredEvents);
    let results = [];

    while (remaining.length >0) {
        let evts = R.splitWhen(type_eq('finishedBenchmark'))(remaining);
        if (R.find(type_neq('runBenchmark'))(evts[0]) && evts[1].length>0) {
            let eventsDuringBenchmark = R.dropWhile(type_neq('runBenchmark'))(evts[0]);

            if (config.LOG_DEBUG) console.log("eventsDuringBenchmark ", eventsDuringBenchmark);

            let gcs = R.filter(type_eq('gc'))(eventsDuringBenchmark);

            let mem = R.last(gcs).mem;
            results.push(mem);
        }
        remaining = R.drop(1, evts[1]);
    }
    // if (results.length !== benchmarkOptions.numIterationsForMemBenchmarks) {
    if (results.length !== 1) { //benchmarkOptions.numIterationsForAllBenchmarks) {
        console.log(`soundness check failed. number or results isn't 1*`, results, asString(filteredEvents));
        throw `soundness check failed. number or results isn't 1`;
    }
    return results[0];
}

async function forceGC(framework: FrameworkData, driver: WebDriver): Promise<any> {
    if (framework.name.startsWith("angular-v4")) {
        // workaround for window.gc for angular 4 - closure rewrites windows.gc");
        await driver.executeScript("window.Angular4PreservedGC();");
    } else {
        for (let i=0;i<5;i++) {
            await driver.executeScript("window.gc();");
        }
    }
}

async function snapMemorySize(driver: WebDriver): Promise<number> {
    // currently needed due to https://github.com/krausest/js-framework-benchmark/issues/538
    let heapSnapshot: any = await driver.executeScript(":takeHeapSnapshot");
    if (typeof(heapSnapshot) === 'string') {
        console.log("INFO: heapSnapshot was a JSON string");
        heapSnapshot = JSON.parse(heapSnapshot);
    }
    console.log("****** heapSnapshot.snapshot.meta", typeof(heapSnapshot));
    let node_fields: any = heapSnapshot.snapshot.meta.node_fields;
    let nodes: any = heapSnapshot.nodes;

    let k = node_fields.indexOf("self_size");

    let self_size = 0;
    for(let l = nodes.length, d = node_fields.length; k < l; k += d) {
        self_size += nodes[k];
    }

    let memory = self_size / 1024.0 / 1024.0;
    return memory;
}

async function runBenchmark(driver: WebDriver, benchmark: Benchmark, framework: FrameworkData) : Promise<any> {
    await benchmark.run(driver, framework);
    if (config.LOG_PROGRESS) console.log("after run ",benchmark.id, benchmark.type, framework.name);
    if (benchmark.type === BenchmarkType.MEM) {
        await forceGC(framework, driver);
    }
}

async function afterBenchmark(driver: WebDriver, benchmark: Benchmark, framework: FrameworkData) : Promise<any> {
    if (benchmark.after) {
        await benchmark.after(driver, framework);
        if (config.LOG_PROGRESS) console.log("after benchmark ",benchmark.id, benchmark.type, framework.name);
    }
}

async function initBenchmark(driver: WebDriver, benchmark: Benchmark, framework: FrameworkData): Promise<any> {
    await benchmark.init(driver, framework)
    if (config.LOG_PROGRESS) console.log("after initialized ",benchmark.id, benchmark.type, framework.name);
    if (benchmark.type === BenchmarkType.MEM) {
        await forceGC(framework, driver);
    }
}

interface Result<T> {
    framework: FrameworkData;
    results: T[];
    benchmark: Benchmark
}


function writeResult<T>(res: Result<T>) {
    if (!config.WRITE_RESULTS) return;
    let benchmark = res.benchmark;
    let framework = res.framework.name;
    let keyed = res.framework.keyed;
    let type = null;

    switch (benchmark.type) {
        case BenchmarkType.CPU: type = "cpu"; break;
        case BenchmarkType.MEM: type = "memory"; break;
        case BenchmarkType.STARTUP: type = "startup"; break;
    }

    for (let resultKind of benchmark.resultKinds()) {
        let data = benchmark.extractResult(res.results, resultKind);
        let s = jStat(data);
        console.log(`result ${fileName(res.framework, resultKind)} min ${s.min()} max ${s.max()} mean ${s.mean()} median ${s.median()} stddev ${s.stdev(true)}`);
        let result: JSONResult = {
            "framework": res.framework.fullNameWithKeyedAndVersion,
            "keyed": keyed,
            "benchmark": resultKind.id,
            "type": type,
            "min": s.min(),
            "max": s.max(),
            "mean": s.mean(),
            "median": s.median(),
            "geometricMean": s.geomean(),
            "standardDeviation": s.stdev(true),
            "values": data
        }
        fs.writeFileSync(`${config.RESULTS_DIRECTORY}/${fileName(res.framework, resultKind)}`, JSON.stringify(result), {encoding: "utf8"});
    }
}

// async function registerError(driver: WebDriver, framework: FrameworkData, benchmark: Benchmark, error: string): Promise<BenchmarkError> {
//     // let fileName = 'error-' + framework.name + '-' + benchmark.id + '.png';
//     console.error("Benchmark failed",error);
//     // let image = await driver.takeScreenshot();
//     // console.error(`Writing screenshot ${fileName}`);
//     // fs.writeFileSync(fileName, image, {encoding: 'base64'});
//     return {imageFile: /*fileName*/ "no img", exception: JSON.stringify(error)};
// }

const wait = (delay = 1000) => new Promise(res => setTimeout(res, delay));

function convertError(error:any): string {
    console.log("ERROR in run Benchmark: |", error, "| type:", typeof error, " instance of Error", error instanceof Error, " Message: ", error.message);
    if (typeof error === 'string') {
        console.log("Error is string");
        return error;
    } 
    else if (error instanceof Error) {
        console.log("Error is instanceof Error");
        return error.message;
    } else {
        console.log("Error is unknown type");
        return error.toString();
    }
}

async function runCPUBenchmark(framework: FrameworkData, benchmark: Benchmark, benchmarkOptions: BenchmarkOptions): Promise<ErrorAndWarning>
{
    let error: String = undefined;
    let warnings: String[] = [];

    console.log("benchmarking ", framework, benchmark.id);
    let driver : WebDriver = null;
    console.timeLog("chromedriver", "runCPU started");
    try {
        driver = buildDriver(benchmarkOptions);
        for (let i = 0; i <benchmarkOptions.numIterationsForCPUBenchmarks; i++) {
            console.timeLog("chromedriver", "before setUseShadowRoot");
            setUseShadowRoot(framework.useShadowRoot);
            console.timeLog("chromedriver", "before get");
            await driver.get(`http://localhost:${benchmarkOptions.port}/${framework.uri}/index.html`);
            console.timeLog("chromedriver", "after get");

            // await (driver as any).sendDevToolsCommand('Network.enable');
            // await (driver as any).sendDevToolsCommand('Network.emulateNetworkConditions', {
                //     offline: false,
                //     latency: 200, // ms
                //     downloadThroughput: 780 * 1024 / 8, // 780 kb/s
                //     uploadThroughput: 330 * 1024 / 8, // 330 kb/s
                // });
                console.log("driver timerstamp *")
            await driver.executeScript("console.timeStamp('initBenchmark')");

            await initBenchmark(driver, benchmark, framework);
            if (benchmark.throttleCPU) {
                console.log("CPU slowdown", benchmark.throttleCPU);
                await (driver as any).sendDevToolsCommand('Emulation.setCPUThrottlingRate', {rate: benchmark.throttleCPU});
            }
            await driver.executeScript("console.timeStamp('runBenchmark')");
            await runBenchmark(driver, benchmark, framework);
            if (benchmark.throttleCPU) {
                console.log("resetting CPU slowdown");
                await (driver as any).sendDevToolsCommand('Emulation.setCPUThrottlingRate', {rate: 1});
            }
            await driver.executeScript("console.timeStamp('finishedBenchmark')");
            await afterBenchmark(driver, benchmark, framework);
            await driver.executeScript("console.timeStamp('afterBenchmark')"); 
        }
        let results = await computeResultsCPU(driver, benchmarkOptions, framework, benchmark, warnings);
        await writeResult({ framework: framework, results: results, benchmark: benchmark });
        console.log("QUIT");
        await driver.close();
        await driver.quit();
    } catch (e) {
        error = convertError(e);
        try {
            if (driver) {
                await driver.close();
                await driver.quit();
            }
        } catch (err) {
            console.log("ERROR cleaning up driver", err);
        }
    }
    return {error, warnings};
}

async function runMemBenchmark(framework: FrameworkData, benchmark: Benchmark, benchmarkOptions: BenchmarkOptions): Promise<ErrorAndWarning>
{
    let error: String = undefined;
    let warnings: String[] = [];
    let allResults: number[] = [];

    console.log("benchmarking ", framework, benchmark.id);
    let driver : WebDriver = null;
    try {
        for (let i = 0; i <benchmarkOptions.numIterationsForMemBenchmarks; i++) {
            driver = buildDriver(benchmarkOptions);
            setUseShadowRoot(framework.useShadowRoot);
            await driver.get(`http://localhost:${benchmarkOptions.port}/${framework.uri}/index.html`);

            await driver.executeScript("console.timeStamp('initBenchmark')");

            await initBenchmark(driver, benchmark, framework);
            if (benchmark.throttleCPU) {
                console.log("CPU slowdown", benchmark.throttleCPU);
                await (driver as any).sendDevToolsCommand('Emulation.setCPUThrottlingRate', {rate: benchmark.throttleCPU});
            }
            await driver.executeScript("console.timeStamp('runBenchmark')");
            await runBenchmark(driver, benchmark, framework);
            if (benchmark.throttleCPU) {
                console.log("resetting CPU slowdown");
                await (driver as any).sendDevToolsCommand('Emulation.setCPUThrottlingRate', {rate: 1});
            }
            let snapshotSize = await snapMemorySize(driver);
            await driver.executeScript("console.timeStamp('finishedBenchmark')");
            await afterBenchmark(driver, benchmark, framework);
            await driver.executeScript("console.timeStamp('afterBenchmark')");
            let result = await computeResultsMEM(driver, benchmarkOptions, framework, benchmark, warnings);
            if (config.LOG_DETAILS) console.log("comparison of memory usage. GC log:", result,  " :takeHeapSnapshot", snapshotSize);
            allResults.push(result);
            await driver.close();
            await driver.quit();
        }
        await writeResult({ framework: framework, results: allResults, benchmark: benchmark });
    } catch (e) {
        error= convertError(e);
        try {
            if (driver) {
                await driver.close();
                await driver.quit();
            }
        } catch (err) {
            console.log("ERROR cleaning up driver", err);
        }
    }
    return {error, warnings};
}

async function runStartupBenchmark(framework: FrameworkData, benchmark: Benchmark, benchmarkOptions: BenchmarkOptions ): Promise<ErrorAndWarning>
{
    console.log("benchmarking startup", framework, benchmark.id);

    let error: String = undefined;
    let results: LighthouseData[] = [];
    try {
        for (let i = 0; i <benchmarkOptions.numIterationsForStartupBenchmark; i++) {
            results.push(await runLighthouse(framework, benchmarkOptions));
        }
    } catch (e) {
        error = convertError(e);
    }
    await writeResult({framework: framework, results: results, benchmark: benchmark});
    return {error, warnings: []};
}

export async function executeBenchmark(frameworks: FrameworkData[], keyed: boolean, frameworkName: string, benchmarkName: string, benchmarkOptions: BenchmarkOptions): Promise<ErrorAndWarning> {
    let runFrameworks = frameworks.filter(f => f.keyed === keyed).filter(f => frameworkName === f.name);
    let runBenchmarks = benchmarks.filter(b => benchmarkName === b.id);
    if (runFrameworks.length!=1) throw `Framework name ${frameworkName} is not unique`;
    if (runBenchmarks.length!=1) throw `Benchmark name ${benchmarkName} is not unique`;

    let framework = runFrameworks[0];
    let benchmark = runBenchmarks[0];

    let errorAndWarnings : ErrorAndWarning;
    if (benchmark.type == BenchmarkType.STARTUP) {
        errorAndWarnings = await runStartupBenchmark(framework, benchmark, benchmarkOptions);
    } else if (benchmark.type == BenchmarkType.CPU) {
        errorAndWarnings = await runCPUBenchmark(framework, benchmark, benchmarkOptions);
    } else {
        errorAndWarnings = await runMemBenchmark(framework, benchmark, benchmarkOptions);
    }

    return errorAndWarnings;
}

export async function performBenchmark(frameworks: FrameworkData[], keyed: boolean, frameworkName: string, benchmarkName: string, benchmarkOptions: BenchmarkOptions): Promise<ErrorAndWarning> {
    let errorAndWarnings = await executeBenchmark(frameworks, keyed, frameworkName, benchmarkName, benchmarkOptions);
    if (config.LOG_DEBUG) console.log("benchmark finished - got errors promise", errorAndWarnings);
    return errorAndWarnings;
}

process.on('message', (msg) => {
    config = msg.config;
    console.log("START BENCHMARK. Write results? ", config.WRITE_RESULTS);
    // if (config.LOG_DEBUG) console.log("child process got message", msg);

    let {frameworks, keyed, frameworkName, benchmarkName, benchmarkOptions} : {frameworks: FrameworkData[], keyed: boolean, frameworkName: string, benchmarkName: string, benchmarkOptions: BenchmarkOptions} = msg;
    if (!benchmarkOptions.port) benchmarkOptions.port = config.PORT.toFixed();
        performBenchmark(frameworks, keyed, frameworkName, benchmarkName, benchmarkOptions).then(result => {
            process.send(result);
            process.exit(0);        
        }).catch((err) => {
            console.log("CATCH: Error in forkedBenchmarkRunner");
            process.send({failure: convertError(err)});
            process.exit(0);
    });
});
