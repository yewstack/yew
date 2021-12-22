import * as React from 'react';
var jStat:any = require('jStat').jStat;

export enum DisplayMode { DisplayMean, DisplayMedian, CompareAgainst, HighlightVariance, BoxPlot };
export interface IDisplayMode {
    type: DisplayMode;
};

class DisplayModeSimple implements IDisplayMode {
    constructor(public type: DisplayMode) {}
};
export class DisplayModeCompare implements IDisplayMode {
    public type = DisplayMode.CompareAgainst;
    constructor(public compareAgainst: Framework|undefined) {}
};
export const DisplayMode_Mean = new DisplayModeSimple(DisplayMode.DisplayMean);
export const DisplayMode_Median = new DisplayModeSimple(DisplayMode.DisplayMedian);
export const DisplayMode_HighlightVariance = new DisplayModeSimple(DisplayMode.HighlightVariance);
export const DisplayMode_BoxPlot = new DisplayModeSimple(DisplayMode.BoxPlot);

export interface Framework {
    name: string;
    keyed: boolean;
    issues?: number[];
}

export enum BenchmarkType { CPU, MEM, STARTUP }

export interface Benchmark {
    id: string;
    type: BenchmarkType;
    label: string;
    description: string;
}

export interface RawResult {
    f: string;
    b: string;
    v: number[];
}

export interface Result {
    framework: string;
    benchmark: string;
    values: number[];
    mean: number;
    median: number;
    standardDeviation: number;
}

export interface DropdownCallback<T> {
  selectNone: (event: React.SyntheticEvent<any>) => void;
  selectAll: (event: React.SyntheticEvent<any>) => void;
  isNoneSelected: () => boolean;
  areAllSelected: () => boolean;
  isSelected: (item: T) => boolean;
}

export interface TableResultEntry {
    render() : JSX.Element;
}

export const SORT_BY_NAME = 'SORT_BY_NAME';
export const SORT_BY_GEOMMEAN_CPU = 'SORT_BY_GEOMMEAN_CPU';
export const SORT_BY_GEOMMEAN_MEM = 'SORT_BY_GEOMMEAN_MEM';
export const SORT_BY_GEOMMEAN_STARTUP = 'SORT_BY_GEOMMEAN_STARTUP';
export type T_SORT_BY_GEOMMEAN = typeof SORT_BY_GEOMMEAN_CPU | typeof SORT_BY_GEOMMEAN_MEM | typeof SORT_BY_GEOMMEAN_STARTUP;

let computeColor = function(factor: number): string {
    if (factor < 2.0) {
        let a = (factor - 1.0);
        let r = (1.0-a)* 99 + a * 255;
        let g = (1.0-a)* 191 + a * 236;
        let b = (1.0-a)* 124 + a * 132;
        return `rgb(${r.toFixed(0)}, ${g.toFixed(0)}, ${b.toFixed(0)})`
    } else  {
        let a = Math.min((factor - 2.0) / 2.0, 1.0);
        let r = (1.0-a)* 255 + a * 249;
        let g = (1.0-a)* 236 + a * 105;
        let b = (1.0-a)* 132 + a * 108;
        return `rgb(${r.toFixed(0)}, ${g.toFixed(0)}, ${b.toFixed(0)})`
    }
}

export class TableResultValueEntry implements TableResultEntry {
    constructor(public key:string, public value: number, public formattedValue: string, public deviation: string, public factor: number, public formattedFactor: string, public bgColor: string, public textColor: string, public statisticallySignificantFactor: string|number|undefined = undefined) {
    }
    render() {
        let col = this.bgColor;
        let textCol = this.textColor;
        return (<td key={this.key} style={{backgroundColor:col, color: textCol}}>
                    {/* <span className="mean">{}</span> */}
                    <span className="mean">{this.formattedValue}</span>
                    <span className="deviation">{this.deviation}</span>
                    <br />
                    <span className="factor">({this.formattedFactor})</span>
                    {this.statisticallySignificantFactor && <>
                    <br/>
                    <span className="factor">{this.statisticallySignificantFactor}</span>
                    </>
                    }
                </td>);
    }
}

export class TableResultGeommeanEntry implements TableResultEntry {
    constructor(public key:string, public mean: number, public color: string) {
    }
    render() {
        return (<th key={this.key} style={{backgroundColor:this.color}}>{this.mean.toFixed(2)}
                </th>);
    }
}

export interface ResultLookup {
    (benchmark: Benchmark, framework: Framework): Result|null;
}
export function convertToMap(results: Array<Result>): ResultLookup {
    let resultMap = new Map<String, Map<String, Result>>();
    results.forEach(r => {
        if (!resultMap.has(r.benchmark)) resultMap.set(r.benchmark, new Map<String,Result>());
        resultMap.get(r.benchmark)!.set(r.framework, r);
    });
    return (benchmark: Benchmark, framework: Framework) => {
        let m = resultMap.get(benchmark.id);
        if (!m) return null;
        let v = m.get(framework.name);
        if (!v) return null;
        return v;
    }
}

let statisticComputeColor = function(sign: number, pValue: number): [string, string] {
    if (pValue > 0.10) {
        return ['#fff','#000'];
    }
    if (sign < 0) {
        let a = (0.1 - pValue) * 10.0;
        let r = 0;
        let g = (1.0-a)* 255 + a * 160;
        let b = 0;
        return [`rgb(${r.toFixed(0)}, ${g.toFixed(0)}, ${b.toFixed(0)})`, '#fff'];
    } else  {
        let a = (0.1 - pValue) * 10.0;
        let r = (1.0-a)* 255 + a * 160;
        let g = 0;
        let b = 0;
        return [`rgb(${r.toFixed(0)}, ${g.toFixed(0)}, ${b.toFixed(0)})`, '#fff'];
    }
}

const formatEn = new Intl.NumberFormat('en-US', {minimumFractionDigits: 1, maximumFractionDigits: 1, useGrouping: true});

export class ResultTableData {
    // Rows
    benchmarksCPU: Array<Benchmark>;
    benchmarksStartup: Array<Benchmark>;
    benchmarksMEM: Array<Benchmark>;
    // Columns
    frameworks: Array<Framework>;
    // Cell data
    resultsCPU: Array<Array<TableResultValueEntry|null>>;   // [benchmark][framework]
    geomMeanCPU: Array<TableResultGeommeanEntry|null>;
    geomMeanStartup: Array<TableResultGeommeanEntry|null>;
    geomMeanMEM: Array<TableResultGeommeanEntry|null>;
    resultsStartup: Array<Array<TableResultValueEntry|null>>;
    resultsMEM: Array<Array<TableResultValueEntry|null>>;

    constructor(public allFrameworks: Array<Framework>, public allBenchmarks: Array<Benchmark>, public results: ResultLookup,
        public selectedFrameworks: Set<Framework>, public selectedBenchmarks: Set<Benchmark>, nonKeyed: boolean|undefined, sortKey: string,
        public displayMode: IDisplayMode) {
        this.frameworks = this.allFrameworks.filter(framework => (nonKeyed===undefined || framework.keyed !== nonKeyed) && selectedFrameworks.has(framework));
        this.update(sortKey);
    }
    private update(sortKey: string) {
        console.time("update");
        this.benchmarksCPU = this.allBenchmarks.filter(benchmark => benchmark.type === BenchmarkType.CPU && this.selectedBenchmarks.has(benchmark));
        this.benchmarksStartup = this.allBenchmarks.filter(benchmark => benchmark.type === BenchmarkType.STARTUP && this.selectedBenchmarks.has(benchmark));
        this.benchmarksMEM = this.allBenchmarks.filter(benchmark => benchmark.type === BenchmarkType.MEM && this.selectedBenchmarks.has(benchmark));

        // const prepare = (benchmark: Benchmark) => {
        //     this.frameworks.forEach(f => {
        //         let result = this.results(benchmark, f);
        //         if (result !== null) {
        //             let vals = result.values.slice(0);
        //             result.mean = jStat.mean(vals);
        //             result.median = jStat.median(vals);
        //             result.standardDeviation = jStat.stdev(vals, true);
        //         }
        //     });
        // }

        // this.benchmarksCPU.forEach(prepare);
        // this.benchmarksStartup.forEach(prepare);
        // this.benchmarksMEM.forEach(prepare);


        this.resultsCPU = this.benchmarksCPU.map(benchmark => this.computeFactors(benchmark, true));
        this.resultsStartup = this.benchmarksStartup.map(benchmark => this.computeFactors(benchmark, true));
        this.resultsMEM = this.benchmarksMEM.map(benchmark => this.computeFactors(benchmark, false));

        this.geomMeanCPU = this.frameworks.map((framework, idx) => {
            let resultsForFramework = this.resultsCPU.map(arr => arr[idx]);
            return this.computeGeometricMean(framework, this.benchmarksCPU, resultsForFramework);
        });
        this.geomMeanStartup = this.frameworks.map((framework, idx) => {
            let resultsForFramework = this.resultsStartup.map(arr => arr[idx]);
            return this.computeGeometricMean(framework, this.benchmarksStartup, resultsForFramework);
        });
        this.geomMeanMEM = this.frameworks.map((framework, idx) => {
            let resultsForFramework = this.resultsMEM.map(arr => arr[idx]);
            return this.computeGeometricMean(framework, this.benchmarksMEM, resultsForFramework);
        });
        this.sortBy(sortKey);
        console.timeEnd("update");
    }
    sortBy(sortKey: string) {
        let zipped = this.frameworks.map((f,frameworkIndex) => {
            let sortValue;
            if (sortKey === SORT_BY_NAME) sortValue = f.name;
            else if (sortKey === SORT_BY_GEOMMEAN_CPU) sortValue = this.geomMeanCPU[frameworkIndex]!.mean || Number.POSITIVE_INFINITY;
            else if (sortKey === SORT_BY_GEOMMEAN_MEM) sortValue = this.geomMeanMEM[frameworkIndex]!.mean || Number.POSITIVE_INFINITY;
            else if (sortKey === SORT_BY_GEOMMEAN_STARTUP) sortValue = this.geomMeanStartup[frameworkIndex]!.mean || Number.POSITIVE_INFINITY;
            else {
                let cpuIdx = this.benchmarksCPU.findIndex(b => b.id === sortKey);
                let startupIdx = this.benchmarksStartup.findIndex(b => b.id === sortKey);
                let memIdx = this.benchmarksMEM.findIndex(b => b.id === sortKey);
                if (cpuIdx>-1) sortValue = this.resultsCPU[cpuIdx][frameworkIndex]==null ? Number.POSITIVE_INFINITY : this.resultsCPU[cpuIdx][frameworkIndex]!.value;
                else if (startupIdx>-1) sortValue = this.resultsStartup[startupIdx][frameworkIndex]==null ? Number.POSITIVE_INFINITY : this.resultsStartup[startupIdx][frameworkIndex]!.value;
                else if (memIdx>-1) sortValue = this.resultsMEM[memIdx][frameworkIndex]==null ? Number.POSITIVE_INFINITY : this.resultsMEM[memIdx][frameworkIndex]!.value;
                else throw `sortKey ${sortKey} not found`;
            }
            return {
                framework: f,
                origIndex: frameworkIndex,
                sortValue: sortValue
            };
        });
        zipped.sort((a,b) => { if (a.sortValue! < b.sortValue!) return -1; else if (a.sortValue == b.sortValue) return 0; return 1;})
        let remappedIdx = zipped.map(z => z.origIndex);
        this.frameworks = this.remap(remappedIdx, this.frameworks);
        this.resultsCPU = this.resultsCPU.map(row => this.remap(remappedIdx, row));
        this.resultsStartup = this.resultsStartup.map(row => this.remap(remappedIdx, row));
        this.resultsMEM = this.resultsMEM.map(row => this.remap(remappedIdx, row));
        this.geomMeanCPU = this.remap(remappedIdx, this.geomMeanCPU);
        this.geomMeanMEM = this.remap(remappedIdx, this.geomMeanMEM);
        this.geomMeanStartup = this.remap(remappedIdx, this.geomMeanStartup);
    }
    remap<T>(remappedIdx: Array<number>, array: Array<T>): Array<T> {
        let copy = new Array<T>(array.length);
        remappedIdx.forEach((idx, i) => {
            copy[i] = array[idx];
        });
        return copy;
    }

    computeGeometricMean(framework: Framework, benchmarksCPU: Array<Benchmark>, resultsCPUForFramework: Array<TableResultValueEntry|null>) {
            let count = 0.0;
            let gMean = resultsCPUForFramework.reduce((gMean, r) => {
                if (r !== null)  {
                    count++;
                    gMean *= (r.factor as number);
                }
                return gMean;
            }, 1.0);
            let value = Math.pow(gMean, 1 / count);
            return new TableResultGeommeanEntry(framework.name, value, computeColor(value));
    }

    computeFactors(benchmark: Benchmark, clamp: boolean): Array<TableResultValueEntry|null> {
        let benchmarkResults = this.frameworks.map(f => this.results(benchmark, f));
        let selectFn = (result: Result|null) => {
            if (result===null) return 0;
            if (this.displayMode.type === DisplayMode.DisplayMean) {
                return result.mean;
            } else if (this.displayMode.type === DisplayMode.DisplayMedian) {
                return result.median;
            } else if (this.displayMode.type === DisplayMode.HighlightVariance) {
                return (result.standardDeviation || 0)/result.mean * 100.0;
            } else { // if (this.displayMode.type === DisplayMode.CompareAgainst) {
                return result.mean;
            }
        }
        let min = benchmarkResults.reduce((min, result) => result===null ? min : Math.min(min, selectFn(result)), Number.POSITIVE_INFINITY);
        return this.frameworks.map(f => {
            let result = this.results(benchmark, f);
            if (result === null) return null;

            let value = selectFn(result);
            let factor = value/min;
            if (this.displayMode.type === DisplayMode.DisplayMean) {
                let conficenceInterval = 1.959964 * (result.standardDeviation || 0) / Math.sqrt(result.values.length);
                let conficenceIntervalStr = conficenceInterval.toFixed(1);
                let formattedValue = formatEn.format(value);
                return new TableResultValueEntry(f.name, value, formattedValue, conficenceIntervalStr, factor, factor.toFixed(2), computeColor(factor), '#000');
            }
            else if (this.displayMode.type === DisplayMode.BoxPlot) {
                let conficenceInterval = 1.959964 * (result.standardDeviation || 0) / Math.sqrt(result.values.length);
                let conficenceIntervalStr = conficenceInterval.toFixed(1);
                let formattedValue = formatEn.format(value);
                return new TableResultValueEntry(f.name, value, formattedValue, conficenceIntervalStr, factor, factor.toFixed(2), computeColor(factor), '#000');
            }
            else if (this.displayMode.type === DisplayMode.DisplayMedian) {
                let conficenceInterval = 1.959964 * (result.standardDeviation || 0) / Math.sqrt(result.values.length);
                let conficenceIntervalStr = conficenceInterval.toFixed(1);
                let formattedValue = formatEn.format(value);
                return new TableResultValueEntry(f.name, value, formattedValue, conficenceIntervalStr, factor, factor.toFixed(2), computeColor(factor), '#000');
            }
            else if (this.displayMode.type === DisplayMode.HighlightVariance) {
                let formattedValue = formatEn.format(result.mean);
                let stdDev = result.standardDeviation || 0;
                let stdDevStr = stdDev.toFixed(1);
                let stdDevFactor = stdDev/result.mean * 100.0;
                console.log("variance ",f.name, benchmark.id, stdDev, value, stdDevFactor);
                return new TableResultValueEntry(f.name, value, formattedValue, stdDevStr, stdDevFactor, stdDevFactor.toFixed(2) + "%", computeColor(stdDevFactor/5.0 + 1.0), '#000');
            }
            else if (this.displayMode.type === DisplayMode.CompareAgainst && (this.displayMode as DisplayModeCompare).compareAgainst) {
                let compareWithResults = this.results(benchmark, (this.displayMode as DisplayModeCompare).compareAgainst!)!;
                let conficenceInterval = 1.959964 * (result.standardDeviation || 0) / Math.sqrt(result.values.length);
                let conficenceIntervalStr = conficenceInterval.toFixed(1);
                // let meanStr = 'x'; //mean.toLocaleString('en-US', {minimumFractionDigits: 1, maximumFractionDigits: 1, useGrouping: true});

                // X1,..,Xn: this Framework, Y1, ..., Ym: selected Framework
                // https://de.wikipedia.org/wiki/Zweistichproben-t-Test
                let formattedValue = formatEn.format(value);
                let statisticalResult = undefined;
                let statisticalCol = undefined;
                let compareWithMean = compareWithResults.mean;
                let stdDev = result.standardDeviation || 0;
                let compareWithResultsStdDev = compareWithResults.standardDeviation || 0;

                let x1 = value;
                let x2 = compareWithMean;
                let s1_2 = stdDev*stdDev;
                let s2_2 = compareWithResultsStdDev * compareWithResultsStdDev;
                let n1 = 10;
                let n2 = 10;
                let ny = Math.pow(s1_2/n1 + s2_2/n2, 2) /
                        (s1_2*s1_2 / (n1*n1*(n1-1)) + s2_2*s2_2/(n2*n2*(n2-1)));
                let t = (x1-x2)/Math.sqrt(s1_2/n1 + s2_2/n2);
                let p = (1.0-jStat.studentt.cdf( Math.abs(t), ny ))*2;
                statisticalCol = statisticComputeColor(t, p);
                statisticalResult = (p*100).toFixed(3)+"%";
                return new TableResultValueEntry(f.name, value, formattedValue, conficenceIntervalStr, factor, factor.toFixed(2), statisticalCol[0], statisticalCol[1], statisticalResult);
            } else {
                return null;
            }
        });
    }
    filterResults = function(bench: Benchmark, frameworks: Array<Framework>, results: Array<Result>) {
        return frameworks.reduce((array, framework) => {
            let res = results.filter(r => r.benchmark === bench.id && r.framework === framework.name);
            if (res.length===1) array.push(res[0]);
            else array.push(null);
            return array;
        }, new Array<Result|null>());
    }
}