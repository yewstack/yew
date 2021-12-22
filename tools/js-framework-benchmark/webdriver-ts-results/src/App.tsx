import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './App.css';
import {benchmarks, frameworks, results as rawResults} from './results';
import {IDisplayMode, Result, RawResult, Framework, Benchmark, BenchmarkType, convertToMap, ResultTableData, SORT_BY_NAME, SORT_BY_GEOMMEAN_CPU, SORT_BY_GEOMMEAN_MEM, SORT_BY_GEOMMEAN_STARTUP, DisplayMode_Mean, DisplayMode, DisplayMode_Median, DisplayModeCompare, DisplayMode_HighlightVariance, DisplayMode_BoxPlot} from './Common';
import {SelectBar} from './SelectBar';
import {ResultTable} from './ResultTable';
var jStat:any = require('jStat').jStat;

interface State {
  benchmarks: Array<Benchmark>;
  benchmarksCPU: Array<Benchmark>;
  benchmarksStartup: Array<Benchmark>;
  benchmarksMEM: Array<Benchmark>;
  frameworks: Array<Framework>;
  frameworksKeyed: Array<Framework>;
  frameworksNonKeyed: Array<Framework>;
  selectedBenchmarks: Set<Benchmark>;
  selectedFrameworks: Set<Framework>;
  separateKeyedAndNonKeyed: boolean;
  resultTables: Array<ResultTableData>;
  sortKey: string;
  displayMode: IDisplayMode;
}

let knownIssues = [
  {issue: 634, text:"The HTML structure for the implementation is not fully correct.", link: "https://github.com/krausest/js-framework-benchmark/issues/634"},
  {issue: 694, text:"Keyed implementations must move the DOM nodes for swap rows ", link: "https://github.com/krausest/js-framework-benchmark/issues/694"},
];


let results : Result[] = (rawResults as RawResult[]).map(res => Object.assign(({framework: res.f, benchmark: res.b, values: res.v}),
    {mean: res.v ? jStat.mean(res.v) : Number.NaN,
    median: res.v ? jStat.median(res.v) : Number.NaN,
    standardDeviation: res.v ? jStat.stdev(res.v, true):  Number.NaN}));

let allBenchmarks = () => benchmarks.reduce((set, b) => set.add(b), new Set<Benchmark>() );
let allFrameworks = () => frameworks.reduce((set, f) => set.add(f), new Set<Framework>() );

let _allBenchmarks = allBenchmarks();
let _allFrameworks = allFrameworks();

let resultLookup = convertToMap(results);

class App extends React.Component<{}, State> {
    benchSelect = (benchmarkType: BenchmarkType) => ({
      selectAll: (event: React.SyntheticEvent<any>) => {
        event.preventDefault();
        let set = this.state.selectedBenchmarks;
        benchmarks.forEach(b => {if (b.type === benchmarkType) set.add(b);});
        this.nextState.selectedBenchmarks = set;
        this.setState({selectedBenchmarks: set, resultTables: this.updateResultTable()});
      },
      selectNone: (event: React.SyntheticEvent<any>) => {
        event.preventDefault();
        let set = this.state.selectedBenchmarks;
        benchmarks.forEach(b => {if (b.type === benchmarkType) set.delete(b);});
        this.nextState.selectedBenchmarks = set;
        this.nextState.sortKey = SORT_BY_NAME;
        this.setState({selectedBenchmarks: set, sortKey: SORT_BY_NAME, resultTables: this.updateResultTable()});
      },
      areAllSelected: () => benchmarks.filter(b => b.type === benchmarkType)
                              .every(b => this.state.selectedBenchmarks.has(b)),
      isNoneSelected: () => benchmarks.filter(b => b.type === benchmarkType)
                              .every(b => !this.state.selectedBenchmarks.has(b)),
      isSelected: (benchmark: Benchmark) => this.state.selectedBenchmarks.has(benchmark)
  })
  frameworkSelect = (keyed: boolean) => ({
      selectAll: (event: React.SyntheticEvent<any>) => {
        event.preventDefault();
        let set = this.state.selectedFrameworks;
        frameworks.forEach(framework => {if (framework.keyed === keyed && !set.has(framework)) set.add(framework);});
        this.nextState.selectedFrameworks = set;
        this.setState({selectedFrameworks: set, resultTables: this.updateResultTable()});
      },
      selectNone: (event: React.SyntheticEvent<any>) => {
        event.preventDefault();
        let set = this.state.selectedFrameworks;
        set.forEach(framework => {if (framework.keyed === keyed) set.delete(framework);});
        this.nextState.selectedFrameworks = set;
        this.setState({selectedFrameworks: set, resultTables: this.updateResultTable()});
      },
      areAllSelected: () => frameworks.filter(f => f.keyed===keyed).every(f => this.state.selectedFrameworks.has(f)),
      isNoneSelected: () => frameworks.filter(f => f.keyed===keyed).every(f => !this.state.selectedFrameworks.has(f)),
      isSelected: (framework: Framework) => this.state.selectedFrameworks.has(framework)
  });
  benchSelectCpu = this.benchSelect(BenchmarkType.CPU);
  benchSelectStartup = this.benchSelect(BenchmarkType.STARTUP);
  benchSelectMem = this.benchSelect(BenchmarkType.MEM);
  frameworkSelectKeyed = this.frameworkSelect(true);
  frameworkSelectNonKeyed = this.frameworkSelect(false);
  nextState: State;

  constructor(props: object) {
    super(props);
    this.nextState = {benchmarks,
                  benchmarksCPU: benchmarks.filter(b => b.type === BenchmarkType.CPU),
                  benchmarksStartup: benchmarks.filter(b => b.type === BenchmarkType.STARTUP),
                  benchmarksMEM: benchmarks.filter(b => b.type === BenchmarkType.MEM),
                  frameworks,
                  frameworksKeyed: frameworks.filter(f => f.keyed === true),
                  frameworksNonKeyed: frameworks.filter(f => f.keyed === false),
                  selectedBenchmarks: _allBenchmarks,
                  selectedFrameworks: _allFrameworks,
                  separateKeyedAndNonKeyed: false,
                  resultTables: [],
                  sortKey: SORT_BY_GEOMMEAN_CPU,
                  displayMode: DisplayMode_Mean,
                };
    this.nextState.resultTables = this.updateResultTable();
    this.state = this.nextState;
  }
  selectBenchmark = (benchmark: Benchmark, value: boolean) => {
    let set = new Set<Benchmark>();
    this.state.selectedBenchmarks.forEach(benchmark => set.add(benchmark));
    if (set.has(benchmark)) set.delete(benchmark);
    else set.add(benchmark);
    let sortKey = this.state.sortKey;
    let setIds = new Set();
    set.forEach(b => setIds.add(b.id))
    if ((sortKey!=SORT_BY_NAME && sortKey!=SORT_BY_GEOMMEAN_CPU && sortKey!=SORT_BY_GEOMMEAN_MEM && sortKey!=SORT_BY_GEOMMEAN_STARTUP) && !setIds.has(sortKey)) sortKey = SORT_BY_NAME;
    this.nextState.selectedBenchmarks = set;
    this.setState({selectedBenchmarks: set, sortKey, resultTables: this.updateResultTable()});
  }
  selectFramework = (framework: Framework, value: boolean): void => {
    let set = new Set<Framework>();
    this.state.selectedFrameworks.forEach(framework => set.add(framework));
    if (set.has(framework)) set.delete(framework);
    else set.add(framework);
    this.nextState.selectedFrameworks = set;
    this.setState({selectedFrameworks: set, resultTables: this.updateResultTable()});
  }
  selectSeparateKeyedAndNonKeyed = (value: boolean): void => {
    this.nextState.separateKeyedAndNonKeyed = value;
    this.setState({separateKeyedAndNonKeyed: value, resultTables: this.updateResultTable()});
  }
  selectDisplayMode = (value: any) => {
    let chooseDisplayMode = (displayMode: DisplayMode) => {
        switch (displayModeEnum) {
            case DisplayMode.DisplayMean:
                return DisplayMode_Mean;
            case DisplayMode.DisplayMedian:
                return DisplayMode_Median;
            case DisplayMode.CompareAgainst:
                return new DisplayModeCompare(undefined);
            case DisplayMode.HighlightVariance:
                return DisplayMode_HighlightVariance;
                case DisplayMode.BoxPlot:
                return DisplayMode_BoxPlot;
        }
    }

    let displayModeEnum = Number(value) as DisplayMode;
    this.nextState.displayMode = chooseDisplayMode(displayModeEnum);
    this.setState({displayMode: this.nextState.displayMode, resultTables: this.updateResultTable()});
  }
  updateResultTable() {
    if (this.nextState.separateKeyedAndNonKeyed) {
      return [new ResultTableData(frameworks, benchmarks, resultLookup, this.nextState.selectedFrameworks, this.nextState.selectedBenchmarks, false, this.nextState.sortKey, this.nextState.displayMode),
              new ResultTableData(frameworks, benchmarks, resultLookup, this.nextState.selectedFrameworks, this.nextState.selectedBenchmarks, true, this.nextState.sortKey, this.nextState.displayMode)]
    } else {
      return [new ResultTableData(frameworks, benchmarks, resultLookup, this.nextState.selectedFrameworks, this.nextState.selectedBenchmarks, undefined, this.nextState.sortKey, this.nextState.displayMode)]
    }
  }
  selectComparison = (framework: string): void => {
    let compareWith: Framework | undefined = undefined;
    compareWith = this.state.frameworksKeyed.find((f) => f.name === framework);
    if (!compareWith) {
      compareWith = this.state.frameworksNonKeyed.find((f) => f.name === framework);
    }
    this.nextState.displayMode = new DisplayModeCompare(compareWith);
    this.setState({displayMode: this.nextState.displayMode, resultTables: this.updateResultTable()});
  }

  sortBy = (sortkey: string, tableIdx: number): void => {
    this.state.resultTables[tableIdx].sortBy(sortkey);
    this.nextState.sortKey = sortkey;
    this.nextState.resultTables = this.updateResultTable();
    this.setState({sortKey:sortkey, resultTables: this.nextState.resultTables});
  }
  render() {
    let disclaimer = (false) ? (<div>
          <h2>Results for js web frameworks benchmark – round 8</h2>
          <p>Go here for the accompanying article <a href="http://www.stefankrause.net/wp/?p=504">http://www.stefankrause.net/wp/?p=504</a>. Source code can be found in the github <a href="https://github.com/krausest/js-framework-benchmark">repository</a>.</p>
        </div>) :
        (<p>Warning: These results are preliminary - use with caution (they may e.g. be from different browser versions).Official results are published on my <a href="http://www.stefankrause.net/">blog</a>.</p>);

    return (
      <div>
        {disclaimer}
        <p>The benchmark was run on a Razer Blade 15 Advanced (i7-8750H, 32 GB RAM, Ubuntu 20.04 (Linux 5.4.0-21, mitigations=off), Chrome  81.0.4044.113 (64-bit))</p>
        <SelectBar  benchmarksCPU={this.state.benchmarksCPU}
                    benchmarksStartup={this.state.benchmarksStartup}
                    benchmarksMEM={this.state.benchmarksMEM}
                    frameworksKeyed={this.state.frameworksKeyed}
                    frameworksNonKeyed={this.state.frameworksNonKeyed}
                    frameworkSelectKeyed={this.frameworkSelectKeyed}
                    frameworkSelectNonKeyed={this.frameworkSelectNonKeyed}
                    benchSelectCpu={this.benchSelectCpu}
                    benchSelectStartup={this.benchSelectStartup}
                    benchSelectMem={this.benchSelectMem}
                    selectBenchmark={this.selectBenchmark}
                    selectFramework={this.selectFramework}
                    selectSeparateKeyedAndNonKeyed={this.selectSeparateKeyedAndNonKeyed}
                    separateKeyedAndNonKeyed={this.state.separateKeyedAndNonKeyed}
                    selectComparison={this.selectComparison}
                    displayMode={this.state.displayMode}
                    selectDisplayMode={this.selectDisplayMode}
                    />
          {this.state.displayMode.type === DisplayMode.CompareAgainst &&
          (<p style={{marginTop:'10px'}}>In comparison mode white cells mean there's no statistically significant difference.
            Green cells are significantly faster than the comparison and red cells are slower.
            The test is performed as a one sided t-test. The significance level is 10%. The darker the color the lower the p-Value.</p>
          )}
          <ResultTable currentSortKey={this.state.sortKey} data={this.state.resultTables} separateKeyedAndNonKeyed={this.state.separateKeyedAndNonKeyed} sortBy={this.sortBy} displayMode={this.state.displayMode}/>

          <h3>Known Issues</h3>
          {knownIssues.map(issue =>
            <dl id={issue.issue.toFixed()}>
              <dt><a target="_blank" href={issue.link}>{issue.issue.toFixed()}</a></dt>
              <dd>{issue.text}</dd>
            </dl>
          )}
      </div>
    );
  }
}

export default App;

ReactDOM.render(
  <App />,
  document.getElementById('main') as HTMLElement
);