import './App.css';
import * as React from 'react';
import {ResultTableData, SORT_BY_NAME, SORT_BY_GEOMMEAN_CPU, TableResultValueEntry, Benchmark, TableResultGeommeanEntry, T_SORT_BY_GEOMMEAN, SORT_BY_GEOMMEAN_STARTUP, SORT_BY_GEOMMEAN_MEM, DisplayMode, IDisplayMode, Framework, ResultLookup, DisplayModeCompare} from './Common';
let Plotly = require('plotly');

export interface Props {
  separateKeyedAndNonKeyed: boolean;
  data: Array<ResultTableData>;
  sortBy: (name:string, tableIdx: number) => void;
  currentSortKey: string;
  displayMode: IDisplayMode;
}


const RenderRows = ({results, geomMean, benchmarks, currentSortKey, sortBy, sortbyGeommeanEnum} : {results: Array<Array<TableResultValueEntry|null>>, geomMean: Array<TableResultGeommeanEntry|null>, benchmarks: Array<Benchmark>, currentSortKey: string,
    sortBy: (name:string) => void , sortbyGeommeanEnum: T_SORT_BY_GEOMMEAN}) => {
    return <>{
        results.map((resultsForBenchmark, benchIdx) =>
    (<tr key={benchmarks[benchIdx].id}>
        <th className='benchname'><a href='#' className={currentSortKey==benchmarks[benchIdx].id ? 'sortKey' : ''} onClick={(event) => {event.preventDefault(); sortBy(benchmarks[benchIdx].id)}}>{benchmarks[benchIdx].label}</a>
          <div className="rowCount">{benchmarks[benchIdx].description}</div>
        </th>
        {resultsForBenchmark.map((result,idx) => result == null ? <td key={idx}></td> : result.render())}
    </tr>))}
    <tr>
        <th><a href='#' className={currentSortKey==sortbyGeommeanEnum ? 'sortKey' : ''} onClick={(event) => {event.preventDefault(); sortBy(sortbyGeommeanEnum)}}>slowdown geometric mean</a></th>
        {geomMean.map(result => result == null ? <td></td> : result.render())}
    </tr>

    </>
}

interface IBoxPlotData {
    framework: string;
    values: number[];
}

class BoxPlot extends React.Component<{traces: IBoxPlotData[]}, {}>
{
    private elemRef: React.RefObject<HTMLDivElement>;
    constructor(props: any) {
        super(props);
        this.elemRef = React.createRef();
        this.repaint = this.repaint.bind(this);
    }
    repaint() {
        let traces = this.props.traces.map(t => ({
            type: 'box',
            y: t.values,
            boxpoints: false,
            jitter: 0.5,
            name: t.framework,
            boxmean: 'sd'
        }));

        let layout = {
            showlegend: false,
            margin: {
                l: 40,
                r: 0,
                b: 120,
                t: 0,
                pad: 0
              },
        };
        Plotly.newPlot(this.elemRef.current, traces, layout, {staticPlot: true, editable: false});
    }
    componentDidMount() {
        this.repaint();
    }
    componentDidUpdate() {
        this.repaint();
    }
    render() {
        return <div ref={this.elemRef} style={{height: '100%', width: '100%'}}></div>
    }
}

const RenderBoxPlotsRows = ({frameworks, benchmarks, results, currentSortKey, sortBy}
    : {frameworks: Array<Framework>, benchmarks: Array<Benchmark>, results: ResultLookup, currentSortKey: string, sortBy: (name:string) => void}) => {
    return <>{benchmarks.map((benchmark) =>
    (<tr key={benchmark.id} style={{height: 400}}>
        <th className='benchname'><a href='#' className={currentSortKey==benchmark.id ? 'sortKey' : ''}
        onClick={(event) => {event.preventDefault(); sortBy(benchmark.id)}}>{benchmark.label}</a>
          <div className="rowCount">{benchmark.description}</div>
        </th>
        <td>
            <BoxPlot traces={frameworks.map(f => ({framework: f.name, values: results(benchmark, f) ? results(benchmark, f)!.values : [] })) as IBoxPlotData[]}/>
        </td>
    </tr>))}</>
}
// {data.frameworks.map(f => <th key={f.name}>{f.name}</th>)}

const BoxPlotTable = ({frameworks, benchmarks, results, currentSortKey, sortBy} :
    {frameworks: Array<Framework>, benchmarks: Array<Benchmark>, results: ResultLookup, currentSortKey: string, sortBy: (name:string) => void}) => {
    return benchmarks.length==0 ? null :
        (<div>
          <h3>Duration in milliseconds</h3>
          <table className='results'>
            <thead>
              <tr>
                <th className='benchname'><a href='#' className={currentSortKey==SORT_BY_NAME ? 'sortKey' : ''} onClick={(event) => {event.preventDefault(); sortBy(SORT_BY_NAME)}}>Name</a></th>
                 <th style={{width: frameworks.length*70+100}}></th>
              </tr>
            </thead>
            <tbody>
                <RenderBoxPlotsRows results={results} frameworks={frameworks} benchmarks={benchmarks} currentSortKey={currentSortKey} sortBy={sortBy}/>
            </tbody>
          </table>
        </div>);
};
const CpuResultsTable = ({data, currentSortKey, sortBy, displayMode} : {data: ResultTableData, currentSortKey: string, sortBy: (name:string) => void, displayMode: IDisplayMode}) => {
  return data.resultsCPU.length==0 ? null :
        (<div>
          <h3>Duration in milliseconds ± {displayMode.type === DisplayMode.HighlightVariance ? 'standard deviation' : '95% confidence interval'} (Slowdown = Duration / Fastest)</h3>
          <table className='results'>
            <thead>
              <tr>
                <th className='benchname'><a href='#' className={currentSortKey==SORT_BY_NAME ? 'sortKey' : ''} onClick={(event) => {event.preventDefault(); sortBy(SORT_BY_NAME)}}>Name</a><br/>Duration for...</th>
                {data.frameworks.map(f => <th className={f.issues ? 'with-issues' : ''} key={f.name}>{f.name}</th>)}
              </tr>
            </thead>
            <thead>
              <tr>
                  <th className='openIssues'>Issues for the implementation</th>
                  {data.frameworks.map(f => <th key={f.name}>{
                    f.issues && f.issues.map(i => <a href={'#'+i.toFixed()}>{i.toFixed()}</a> )
                  }</th>)}
              </tr>
            </thead>
            <tbody>
                <RenderRows results={data.resultsCPU} benchmarks={data.benchmarksCPU} currentSortKey={currentSortKey} sortBy={sortBy} geomMean={data.geomMeanCPU} sortbyGeommeanEnum={SORT_BY_GEOMMEAN_CPU}/>
            </tbody>
          </table>
        </div>);
};

const StartupResultsTable = ({data, currentSortKey, sortBy} : {data: ResultTableData, currentSortKey: string, sortBy: (name:string) => void}) => {
  return data.resultsStartup.length==0 ? null :
        (<div>
          <h3>Startup metrics (lighthouse with mobile simulation)</h3>
          <table className='results'>
            <thead>
              <tr>
                <th className='benchname'><a href='#' className={currentSortKey==SORT_BY_NAME ? 'sortKey' : ''} onClick={(event) => {event.preventDefault(); sortBy(SORT_BY_NAME)}}>Name</a></th>
                {data.frameworks.map(f => <th key={f.name}>{f.name}</th>)}
              </tr>
            </thead>
            <tbody>
            <RenderRows results={data.resultsStartup} benchmarks={data.benchmarksStartup} currentSortKey={currentSortKey} sortBy={sortBy} geomMean={data.geomMeanStartup} sortbyGeommeanEnum={SORT_BY_GEOMMEAN_STARTUP}/>
            </tbody>
          </table>
        </div>);
};

const MEM_TABLE_TEMP_DISABLED = false;

const MemResultsTable = ({data, currentSortKey, sortBy, displayMode} : {data: ResultTableData, currentSortKey: string, sortBy: (name:string) => void, displayMode: IDisplayMode}) => {
  return (MEM_TABLE_TEMP_DISABLED ? (<div><h3>Memory measuremts currently disabled (see issue <a href='https://github.com/krausest/js-framework-benchmark/issues/538'>#538</a>)</h3></div>) :
        (data.resultsMEM.length==0 ? null :
        (<div>
          <h3>Memory allocation in MBs ± {displayMode.type === DisplayMode.HighlightVariance ? 'standard deviation' : '95% confidence interval'}</h3>
          <table className='results'>
            <thead>
              <tr>
                <th className='benchname'><a href='#' className={currentSortKey==SORT_BY_NAME ? 'sortKey' : ''} onClick={(event) => {event.preventDefault(); sortBy(SORT_BY_NAME)}}>Name</a></th>
                {data.frameworks.map(f => <th key={f.name}>{f.name}</th>)}
              </tr>
            </thead>
            <tbody>
                <RenderRows results={data.resultsMEM} benchmarks={data.benchmarksMEM} currentSortKey={currentSortKey} sortBy={sortBy}  geomMean={data.geomMeanMEM} sortbyGeommeanEnum={SORT_BY_GEOMMEAN_MEM}/>
            </tbody>
          </table>
        </div>)));
};

interface Texts {
  nonKeyed: boolean|undefined;
  label: string;
  description: string;
}

export class ResultTable extends React.Component<Props, {}> {
  constructor(props: Props) {
    super(props);
  }
  render() {
      let texts : Array<Texts> = this.props.separateKeyedAndNonKeyed ?
                [{nonKeyed: false, label: 'Keyed results', description: 'Keyed implementations create an association between the domain data and a dom element by assigning a \'key\'. If data changes the dom element with that key will be updated. In consequence inserting or deleting an element in the data array causes a corresponding change to the dom.'}, {nonKeyed: true, label: 'Non keyed results', description: 'Non keyed implementations are allowed to reuse existing dom elements. In consequence inserting or deleting an element in the data array might append after or delete the last table row and update the contents of all elements after the inserting or deletion index. This can perform better, but can cause problems if dom state is modified externally.'}]
                : [{nonKeyed: undefined, label: 'Mixed keyed and non-keyed', description: 'This is an apple to oranges comparison. Use it to find out how much a non-keyed version can be faster (if that doesn\'t introduce any problems e.g. with transitions).'}];

        if (this.props.displayMode.type === DisplayMode.CompareAgainst && !(this.props.displayMode as DisplayModeCompare).compareAgainst) {
            return (<h3>Please choose a framework for the comparison.</h3>);
        }
        return (
        <div className="mt-3">
          { this.props.data.map((data, idx) => {
            return ( data.frameworks.length===0 || data.benchmarksCPU.length==0 && data.benchmarksStartup.length==0 && data.benchmarksMEM.length==0 ? null :
              <div key={texts[idx].label}>
                <h1>{texts[idx].label}</h1>
                <p>{texts[idx].description}</p>
                {
            this.props.displayMode.type === DisplayMode.BoxPlot ?
                    (<>
                <BoxPlotTable results={data.results} frameworks={data.frameworks} benchmarks={data.benchmarksCPU} currentSortKey={this.props.currentSortKey} sortBy={(sortKey) => this.props.sortBy(sortKey, idx)}/>
                    </>)
            :
                (<>
                <CpuResultsTable currentSortKey={this.props.currentSortKey} sortBy={(sortKey) => this.props.sortBy(sortKey, idx)} data={data} displayMode={this.props.displayMode}/>
                <StartupResultsTable currentSortKey={this.props.currentSortKey} sortBy={(sortKey) => this.props.sortBy(sortKey, idx)} data={data}/>
                <MemResultsTable currentSortKey={this.props.currentSortKey} sortBy={(sortKey) => this.props.sortBy(sortKey, idx)} data={data}  displayMode={this.props.displayMode}/>
                </>)}
              </div>
            )})}
        </div>
      );
    }
}