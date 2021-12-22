import { BenchmarkType, Benchmark, benchmarks, fileName, LighthouseData } from './benchmarks'
import * as fs from 'fs';
import * as yargs from 'yargs';
import * as path from 'path'
import { JSONResult, config, FrameworkData, initializeFrameworks, ErrorAndWarning, BenchmarkOptions } from './common'
import * as R from 'ramda';
import { fork } from 'child_process';
import { executeBenchmark } from './forkedBenchmarkRunner';
import mapObjIndexed from 'ramda/es/mapObjIndexed';

function forkedRun(frameworks: FrameworkData[], frameworkName: string, keyed: boolean, benchmarkName: string, benchmarkOptions: BenchmarkOptions): Promise<ErrorAndWarning> {
    if (config.FORK_CHROMEDRIVER) {
        return new Promise(function (resolve, reject) {
            const forked = fork('dist/forkedBenchmarkRunner.js');
            if (config.LOG_DEBUG) console.log("forked child process");
            forked.send({ config, frameworks, keyed, frameworkName, benchmarkName, benchmarkOptions });
            forked.on('message', (msg: ErrorAndWarning) => {
                if (config.LOG_DEBUG) console.log("main process got message from child", msg);
                resolve(msg);
            });
            forked.on('close', (msg) => {
                if (config.LOG_DEBUG) console.log("child closed", msg);
            });
            forked.on('error', (msg) => {
                if (config.LOG_DEBUG) console.log("child error", msg);
            });
            forked.on('exit', (code, signal) => {
                if (config.LOG_DEBUG) console.log("child exit", code, signal);
            });
        });
    } else {
        return executeBenchmark(frameworks, keyed, frameworkName, benchmarkName, benchmarkOptions);
    }
}

async function performRetryableRun(runFrameworks: FrameworkData[], framework: FrameworkData, benchmark: Benchmark) {
    let errors: String[] = [];
    let warnings: String[] = [];
    let retry = 1;
    let attemptRetry = true;

    for (; retry<=5 && attemptRetry; retry++) {
        errors = [];
        warnings = [];
        console.log(`Executing benchmark ${framework.name} and benchmark ${benchmark.id} retry # ${retry}`);

        attemptRetry = false;
        let benchmarkOptions: BenchmarkOptions = {
            port: config.PORT.toFixed(),
            remoteDebuggingPort: config.REMOTE_DEBUGGING_PORT,
            chromePort: config.CHROME_PORT,
            headless: args.headless,
            chromeBinaryPath: args.chromeBinary,
            numIterationsForCPUBenchmarks: config.REPEAT_RUN,
            numIterationsForMemBenchmarks: config.REPEAT_RUN_MEM,
            numIterationsForStartupBenchmark: config.REPEAT_RUN_STARTUP
        }
        // Assumption: For all errors we can handle it won't throw but return a result
        let benchMsg: any = await forkedRun(runFrameworks, framework.name, framework.keyed, benchmark.id, benchmarkOptions);
        if (benchMsg.failure) {
            console.log(`Executing ${framework.uri} and benchmark ${benchmark.id} failed with a technical error: ${benchMsg.failure}`);
            errors.push(`Executing ${framework.uri} and benchmark ${benchmark.id} failed with a technical error: ${benchMsg.failure}`);
            if (config.EXIT_ON_ERROR) throw "Exiting because of an technical error and config.EXIT_ON_ERROR = true";
        } else {
            let errorsAndWarnings = benchMsg as ErrorAndWarning;
            if (errorsAndWarnings.error) errors.push(`Executing ${framework.uri} and benchmark ${benchmark.id} failed: ` + errorsAndWarnings.error);
            for (let warning of errorsAndWarnings.warnings) {
                if (errorsAndWarnings.error) warnings.push(`Executing ${framework.uri} and benchmark ${benchmark.id} failed: ` + errorsAndWarnings.error);
            }
            if (errorsAndWarnings.error && errorsAndWarnings.error.indexOf("Server terminated early with status 1")>-1) {
                console.log("******* STRANGE selenium error found - retry");
                attemptRetry = true;
            }
            if (errorsAndWarnings.error && config.EXIT_ON_ERROR) throw "Exiting because of an error and config.EXIT_ON_ERROR = true";
        }
    }
    return {errors: errors, warnings: warnings};
}

async function runBench(runFrameworks: FrameworkData[], benchmarkNames: string[]) {
    let errors: String[] = [];
    let warnings: String[] = [];

    let runBenchmarks = benchmarks.filter(b => benchmarkNames.some(name => b.id.toLowerCase().indexOf(name) > -1));

    let restart: string = undefined;
    let index = runFrameworks.findIndex(f => f.fullNameWithKeyedAndVersion===restart);
    if (index>-1) {
        runFrameworks = runFrameworks.slice(index);
    }

    console.log("Frameworks that will be benchmarked", runFrameworks.map(f => f.fullNameWithKeyedAndVersion));
    console.log("Benchmarks that will be run", runBenchmarks.map(b => b.id));

    let data: [[FrameworkData, Benchmark]] = <any>[];
    for (let i = 0; i < runFrameworks.length; i++) {
        for (let j = 0; j < runBenchmarks.length; j++) {
            data.push([runFrameworks[i], runBenchmarks[j]]);
        }
    }

    for (let i = 0; i < data.length; i++) {
        let framework = data[i][0];
        let benchmark = data[i][1];
        let result = await performRetryableRun(runFrameworks, framework, benchmark);
        errors = errors.concat(result.errors);
        warnings = warnings.concat(result.warnings);
    }

    if (warnings.length > 0) {
        console.log("================================");
        console.log("The following warnings were logged:");
        console.log("================================");

        warnings.forEach(e => {
            console.log(e);
        });
    }

    if (errors.length > 0) {
        console.log("================================");
        console.log("The following benchmarks failed:");
        console.log("================================");

        errors.forEach(e => {
            console.log(e);
        });
        throw "Benchmarking failed with errors";
    }
}

// FIXME: Clean up args.
// What works: npm run bench keyed/react, npm run bench -- keyed/react, npm run bench -- keyed/react --count 1 --benchmark 01_
// What doesn't work (keyed/react becomes an element of argument benchmark): npm run bench -- --count 1 --benchmark 01_ keyed/react

let args = yargs(process.argv)
    .usage("$0 [--framework Framework1 Framework2 ...] [--benchmark Benchmark1 Benchmark2 ...] [--count n] [--exitOnError] \n or: $0 [directory1] [directory2] .. [directory3] \n or: $0 installed")
    .help('help')
    .default('check', 'false')
    .default('fork', 'true')
    .boolean('noResults')
    .default('exitOnError', 'false')
    .default('count', Number.MAX_SAFE_INTEGER)
    .default('port', config.PORT)
    .string('chromeBinary')
    .string('chromeDriver')
    .boolean('headless')
    .boolean('installed')
    .array("framework").array("benchmark")
    .argv;

let allArgs = args._.length<=2 ? [] : args._.slice(2,args._.length);

let runBenchmarksFromDirectoryNamesArgs = !args.framework;

async function main() {

    let runBenchmarks = (args.benchmark && args.benchmark.length > 0 ? args.benchmark : [""]).map(v => v.toString());
    let runFrameworks: FrameworkData[];
    if(args.installed){
        console.log("MODE: Installed frameworks.");
        const hasPackageLock = (directoryName: string)=>
            !!fs.existsSync(path.join(path.resolve('..','frameworks'), directoryName, 'package-lock.json'))
        runFrameworks = await initializeFrameworks(hasPackageLock)
    } else if (runBenchmarksFromDirectoryNamesArgs) {
        console.log("MODE: Directory names. Using arguments as the directory names to be re-run: ", allArgs);
        let matchesDirectoryArg = (directoryName: string) => allArgs.length==0 || allArgs.some(arg => arg==directoryName)
        runFrameworks = await initializeFrameworks(matchesDirectoryArg);
    } else {
        console.log("MODE: Classic command line options");
        let frameworkNames = (args.framework && args.framework.length > 0 ? args.framework : [""]).map(v => v.toString());
        let frameworks = await initializeFrameworks();
        runFrameworks = frameworks.filter(f => frameworkNames.some(name => f.fullNameWithKeyedAndVersion.indexOf(name) > -1));
    }
    let count = Number(args.count);
    config.PORT = Number(args.port);
    if (count < Number.MAX_SAFE_INTEGER) config.REPEAT_RUN = count;
    config.REPEAT_RUN_MEM = Math.min(count, config.REPEAT_RUN_MEM);
    config.REPEAT_RUN_STARTUP = Math.min(count, config.REPEAT_RUN_STARTUP);
    config.FORK_CHROMEDRIVER = args.fork === 'true';
    config.WRITE_RESULTS = !args.noResults;

    console.log(args, "no-results", args.noResults, config.WRITE_RESULTS);

    let exitOnError = args.exitOnError === 'true'

    config.EXIT_ON_ERROR = exitOnError;

    console.log("fork chromedriver process?", config.FORK_CHROMEDRIVER);

    if (!fs.existsSync(config.RESULTS_DIRECTORY))
    fs.mkdirSync(config.RESULTS_DIRECTORY);

    if (args.help) {
        yargs.showHelp();
    } else {
        return runBench(runFrameworks, runBenchmarks);
    }
}

main().then(_ => {
    console.log("successful run");
    process.exit(0);
}).catch(error => {
    console.log("run was not completely sucessful", error);
    process.exit(1);
})