import { testTextContains, testTextNotContained, testClassContains, testElementLocatedByXpath, testElementNotLocatedByXPath, testElementLocatedById, clickElementById, clickElementByXPath, getTextByXPath } from './webdriverAccess'
import { Builder, WebDriver, promise, logging } from 'selenium-webdriver'
import { config, FrameworkData } from './common'

export enum BenchmarkType { CPU, MEM, STARTUP };

const SHORT_TIMEOUT = 20 * 1000;

export interface BenchmarkInfo {
    id: string;
    type: BenchmarkType;
    label: string;
    description: string;
    throttleCPU?: number
}

export abstract class Benchmark {
    id: string;
    type: BenchmarkType;
    label: string;
    description: string;
    throttleCPU?: number;

    constructor(public benchmarkInfo: BenchmarkInfo) {
        this.id = benchmarkInfo.id;
        this.type = benchmarkInfo.type;
        this.label = benchmarkInfo.label;
        this.description = benchmarkInfo.description;
        this.throttleCPU = benchmarkInfo.throttleCPU;
    }
    abstract init(driver: WebDriver, framework: FrameworkData): Promise<any>;
    abstract run(driver: WebDriver, framework: FrameworkData): Promise<any>;
    after(driver: WebDriver, framework: FrameworkData): Promise<any> { return null; }
    // Good fit for a single result creating Benchmark
    resultKinds(): Array<BenchmarkInfo> { return [this.benchmarkInfo]; }
    extractResult(results: any[], resultKind: BenchmarkInfo): number[] { return results; };
}

export interface LighthouseData {
    TimeToConsistentlyInteractive: number;
    ScriptBootUpTtime: number;
    MainThreadWorkCost: number;
    TotalKiloByteWeight: number;
    [propName: string]: number;
}

export interface StartupBenchmarkResult extends BenchmarkInfo {
    property: keyof LighthouseData;
}

const benchRun = new class extends Benchmark {
    constructor() {
        super({
            id: "01_run1k",
            label: "create rows",
            description: "creating 1,000 rows",
            type: BenchmarkType.CPU
        })
    }
    async init(driver: WebDriver) { await testElementLocatedById(driver, "add", SHORT_TIMEOUT); }
    async run(driver: WebDriver) {
        await clickElementById(driver, "add");
        await testElementLocatedByXpath(driver, "//tbody/tr[1000]/td[2]/a");
    }
}

const benchReplaceAll = new class extends Benchmark {
    constructor() {
        super({
            id: "02_replace1k",
            label: "replace all rows",
            description: "updating all 1,000 rows (" + config.WARMUP_COUNT + " warmup runs).",
            type: BenchmarkType.CPU
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, 'run', SHORT_TIMEOUT);
        for (let i = 0; i < config.WARMUP_COUNT; i++) {
            await clickElementById(driver, 'run');
            await testTextContains(driver, '//tbody/tr[1]/td[1]', (i*1000+1).toFixed());
        }
    }
    async run(driver: WebDriver) {
        await clickElementById(driver, 'run');
        await testTextContains(driver, '//tbody/tr[1]/td[1]', '5001');
    }
}

const benchUpdate = new class extends Benchmark {
    constructor() {
        super({
            id: "03_update10th1k_x16",
            label: "partial update",
            description: "updating every 10th row for 1,000 rows (3 warmup runs). 16x CPU slowdown.",
            type: BenchmarkType.CPU,
            throttleCPU: 16
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "run", SHORT_TIMEOUT);
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1000]/td[2]/a");
        for (let i = 0; i < 3; i++) {
            await clickElementById(driver, 'update');
            await testTextContains(driver, '//tbody/tr[991]/td[2]/a', ' !!!'.repeat(i + 1));
        }
    }
    async run(driver: WebDriver) {
        await clickElementById(driver, 'update');
        await testTextContains(driver, '//tbody/tr[991]/td[2]/a', ' !!!'.repeat(3 + 1));
    }
}

const benchSelect = new class extends Benchmark {
    constructor() {
        super({
            id: "04_select1k",
            label: "select row",
            description: "highlighting a selected row. (" + config.WARMUP_COUNT + " warmup runs). 16x CPU slowdown.",
            type: BenchmarkType.CPU,
            throttleCPU: 16
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "run", SHORT_TIMEOUT);
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1]/td[2]/a");
        for (let i = 0; i <= config.WARMUP_COUNT; i++) {
            await clickElementByXPath(driver, `//tbody/tr[${i + 1}]/td[2]/a`);
        }
    }
    async run(driver: WebDriver) {
        await clickElementByXPath(driver, "//tbody/tr[2]/td[2]/a");
        await testClassContains(driver, "//tbody/tr[2]", "danger");
    }
}

const benchSwapRows = new class extends Benchmark {
    constructor() {
        super({
            id: "05_swap1k",
            label: "swap rows",
            description: "swap 2 rows for table with 1,000 rows. (" + config.WARMUP_COUNT + " warmup runs). 4x CPU slowdown.",
            type: BenchmarkType.CPU,
            throttleCPU: 4
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "run", SHORT_TIMEOUT);
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1]/td[2]/a");
        for (let i = 0; i <= config.WARMUP_COUNT; i++) {
            let text = await getTextByXPath(driver, "//tbody/tr[2]/td[2]/a")
            await clickElementById(driver, 'swaprows');
            await testTextContains(driver, "//tbody/tr[999]/td[2]/a", text);
        }
    }
    async run(driver: WebDriver) {
        let text = await getTextByXPath(driver, "//tbody/tr[2]/td[2]/a");
        await clickElementById(driver, 'swaprows');
        await testTextContains(driver, "//tbody/tr[999]/td[2]/a", text);
    }
}

const benchRemove = new class extends Benchmark {
    constructor() {
        super({
            id: "06_remove-one-1k",
            label: "remove row",
            description: "removing one row. (" + config.WARMUP_COUNT + " warmup runs).",
            type: BenchmarkType.CPU
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "run", SHORT_TIMEOUT);
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1]/td[2]/a");
        for (let i = 0; i < config.WARMUP_COUNT; i++) {
            await testTextContains(driver, `//tbody/tr[${config.WARMUP_COUNT - i + 4}]/td[1]`, (config.WARMUP_COUNT - i + 4).toString());
            await clickElementByXPath(driver, `//tbody/tr[${config.WARMUP_COUNT - i + 4}]/td[3]/a/span[1]`);
            await testTextContains(driver, `//tbody/tr[${config.WARMUP_COUNT - i + 4}]/td[1]`, '10');
        }
        await testTextContains(driver, '//tbody/tr[5]/td[1]', '10');
        await testTextContains(driver, '//tbody/tr[4]/td[1]', '4');

        // Click on a row the second time
        await testTextContains(driver, `//tbody/tr[6]/td[1]`, '11');
        await clickElementByXPath(driver, `//tbody/tr[6]/td[3]/a/span[1]`);
        await testTextContains(driver, `//tbody/tr[6]/td[1]`, '12');

    }
    async run(driver: WebDriver) {
        await clickElementByXPath(driver, "//tbody/tr[4]/td[3]/a/span[1]");
        await testTextContains(driver, '//tbody/tr[4]/td[1]', '10');
    }
}

const benchRunBig = new class extends Benchmark {
    constructor() {
        super({
            id: "07_create10k",
            label: "create many rows",
            description: "creating 10,000 rows",
            type: BenchmarkType.CPU
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "runlots", SHORT_TIMEOUT);
    }
    async run(driver: WebDriver) {
        await clickElementById(driver, 'runlots');
        await testElementLocatedByXpath(driver, "//tbody/tr[10000]/td[2]/a");
    }
}

const benchAppendToManyRows = new class extends Benchmark {
    constructor() {
        super({
            id: "08_create1k-after1k_x2",
            label: "append rows to large table",
            description: "appending 1,000 to a table of 10,000 rows. 2x CPU slowdown",
            type: BenchmarkType.CPU,
            throttleCPU: 2
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "run", SHORT_TIMEOUT);
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1000]/td[2]/a");
    }
    async run(driver: WebDriver) {
        await clickElementById(driver, 'add');
        await testElementLocatedByXpath(driver, "//tbody/tr[1100]/td[2]/a");
    }
}

const benchClear = new class extends Benchmark {
    constructor() {
        super({
            id: "09_clear1k_x8",
            label: "clear rows",
            description: "clearing a table with 1,000 rows. 8x CPU slowdown",
            type: BenchmarkType.CPU,
            throttleCPU: 8
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "run", SHORT_TIMEOUT);
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1000]/td[2]/a");
    }
    async run(driver: WebDriver) {
        await clickElementById(driver, 'clear');
        await testElementNotLocatedByXPath(driver, "//tbody/tr[1]");
    }
}

const benchReadyMemory = new class extends Benchmark {
    constructor() {
        super({
            id: "21_ready-memory",
            label: "ready memory",
            description: "Memory usage after page load.",
            type: BenchmarkType.MEM,
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "add", SHORT_TIMEOUT);
    }
    async run(driver: WebDriver) {
        await testElementNotLocatedByXPath(driver, "//tbody/tr[1]");
    }
    async after(driver: WebDriver, framework: FrameworkData) {
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1]/td[2]/a");
    }
}

const benchRunMemory = new class extends Benchmark {
    constructor() {
        super({
            id: "22_run-memory",
            label: "run memory",
            description: "Memory usage after adding 1000 rows.",
            type: BenchmarkType.MEM,
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "add", SHORT_TIMEOUT);
    }
    async run(driver: WebDriver) {
        await clickElementById(driver, 'run');
        await testElementLocatedByXpath(driver, "//tbody/tr[1]/td[2]/a");
    }
}

const benchUpdate5Memory = new class extends Benchmark {
    constructor() {
        super({
            id: "23_update5-memory",
            label: "update eatch 10th row for 1k rows (5 cycles)",
            description: "Memory usage after clicking update every 10th row 5 times",
            type: BenchmarkType.MEM,
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "add", SHORT_TIMEOUT);
    }
    async run(driver: WebDriver) {
        await clickElementById(driver, 'run');
        for (let i = 0; i < 5; i++) {
            await clickElementById(driver, 'update');
            await testTextContains(driver, '//tbody/tr[1]/td[2]/a', ' !!!'.repeat(i));
        }
    }
}

const benchReplace5Memory = new class extends Benchmark {
    constructor() {
        super({
            id: "24_run5-memory",
            label: "replace 1k rows (5 cycles)",
            description: "Memory usage after clicking create 1000 rows 5 times",
            type: BenchmarkType.MEM,
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "add", SHORT_TIMEOUT);
    }
    async run(driver: WebDriver) {
        for (let i = 0; i < 5; i++) {
            await clickElementById(driver, 'run');
            await testTextContains(driver, "//tbody/tr[1000]/td[1]", (1000 * (i + 1)).toFixed());
        }
    }
}

const benchCreateClear5Memory = new class extends Benchmark {
    constructor() {
        super({
            id: "25_run-clear-memory",
            label: "creating/clearing 1k rows (5 cycles)",
            description: "Memory usage after creating and clearing 1000 rows 5 times",
            type: BenchmarkType.MEM,
        })
    }
    async init(driver: WebDriver) {
        await testElementLocatedById(driver, "add", SHORT_TIMEOUT);
    }
    async run(driver: WebDriver) {
        for (let i = 0; i < 5; i++) {
            await clickElementById(driver, 'run');
            await testTextContains(driver, "//tbody/tr[1000]/td[1]", (1000 * (i + 1)).toFixed());
            await clickElementById(driver, 'clear');
            await testElementNotLocatedByXPath(driver, "//tbody/tr[1000]/td[1]");
        }
    }
}

const benchStartupConsistentlyInteractive: StartupBenchmarkResult = {
    id: "31_startup-ci",
    label: "consistently interactive",
    description: "a pessimistic TTI - when the CPU and network are both definitely very idle. (no more CPU tasks over 50ms)",
    type: BenchmarkType.STARTUP,
    property: "TimeToConsistentlyInteractive"
}

const benchStartupBootup: StartupBenchmarkResult = {
    id: "32_startup-bt",
    label: "script bootup time",
    description: "the total ms required to parse/compile/evaluate all the page's scripts",
    type: BenchmarkType.STARTUP,
    property: "ScriptBootUpTtime"
}

const benchStartupMainThreadWorkCost: StartupBenchmarkResult = {
    id: "33_startup-mainthreadcost",
    label: "main thread work cost",
    description: "total amount of time spent doing work on the main thread. includes style/layout/etc.",
    type: BenchmarkType.STARTUP,
    property: "MainThreadWorkCost"
}

const benchStartupTotalBytes: StartupBenchmarkResult = {
    id: "34_startup-totalbytes",
    label: "total kilobyte weight",
    description: "network transfer cost (post-compression) of all the resources loaded into the page.",
    type: BenchmarkType.STARTUP,
    property: "TotalKiloByteWeight"
}

class BenchStartup extends Benchmark {
    constructor() {
        super({
            id: "30_startup",
            label: "startup time",
            description: "Time for loading, parsing and starting up",
            type: BenchmarkType.STARTUP,
        })
    }
    async init(driver: WebDriver) { // not used with lighthouse
    }
    async run(driver: WebDriver, framework: FrameworkData) {
        // not used with lighthouse
    }
    extractResult(results: LighthouseData[], resultKind: BenchmarkInfo): number[] {
        return results.reduce((a, v) => { a.push(v[(resultKind as StartupBenchmarkResult).property]); return a; }, new Array<number>());
    }
    resultKinds() {
        return [
            benchStartupConsistentlyInteractive,
            benchStartupBootup,
            // benchStartupMainThreadWorkCost,
            benchStartupTotalBytes,
        ];
    }
}
const benchStartup = new BenchStartup();

export let benchmarks : Array<Benchmark> = [
    benchRun,
    benchReplaceAll,
    benchUpdate,
    benchSelect,
    benchSwapRows,
    benchRemove,
    benchRunBig,
    benchAppendToManyRows,
    benchClear,
    benchReadyMemory,
    benchRunMemory,
    benchUpdate5Memory,
    benchReplace5Memory,
    benchCreateClear5Memory,
    benchStartup,
];

export function fileName(framework: FrameworkData, benchmark: BenchmarkInfo) {
    return `${framework.fullNameWithKeyedAndVersion}_${benchmark.id}.json`;
}