import * as React from 'react';
import './App.css';
import { DropDown } from './DropDown'
import { DropDownContents } from './DropDownContents'
import { Framework, Benchmark, DropdownCallback, DisplayMode, IDisplayMode, DisplayModeCompare } from './Common';
import { SelectBarFrameworks } from './SelectBarFrameworks';
import { SelectBarBenchmarks } from './SelectBarBenchmarks';

export interface Props {
    frameworkSelectKeyed: DropdownCallback<Framework>;
    frameworkSelectNonKeyed: DropdownCallback<Framework>;
    benchSelectCpu: DropdownCallback<Benchmark>;
    benchSelectStartup: DropdownCallback<Benchmark>;
    benchSelectMem: DropdownCallback<Benchmark>;
    selectFramework: (framework: Framework, value: boolean) => void;
    selectBenchmark: (benchmark: Benchmark, value: boolean) => void;
    selectSeparateKeyedAndNonKeyed: (value: boolean) => void;
    separateKeyedAndNonKeyed: boolean;
    frameworksKeyed: Array<Framework>;
    frameworksNonKeyed: Array<Framework>;
    benchmarksCPU: Array<Benchmark>;
    benchmarksStartup: Array<Benchmark>;
    benchmarksMEM: Array<Benchmark>;
    selectComparison: (framework: string) => void;
    selectDisplayMode: (mode: string) => void;
    displayMode: IDisplayMode;
}

const SelectCategory = ({ benchmarks, select, benchSelect, label }:
    { benchmarks: Array<Benchmark>, select: (benchmark: Benchmark, value: boolean) => void, benchSelect: DropdownCallback<Benchmark>, label: string }) => {
    return (<DropDownContents {...benchSelect}>
        <h3>{label}</h3>
        <div>
            <SelectBarBenchmarks isSelected={benchSelect.isSelected} select={select} benchmarks={benchmarks} />
        </div>
    </DropDownContents>);
}



export class SelectBar extends React.Component<Props, {}> {
    render() {
        let { frameworkSelectKeyed,
            frameworkSelectNonKeyed,
            benchSelectCpu,
            benchSelectStartup,
            benchSelectMem,
            selectFramework,
            selectBenchmark,
            selectSeparateKeyedAndNonKeyed,
            separateKeyedAndNonKeyed,
            frameworksKeyed,
            frameworksNonKeyed,
            benchmarksCPU,
            benchmarksStartup,
            benchmarksMEM,
            selectComparison,
            selectDisplayMode,
            displayMode
        } = this.props;
        return (
            <div className="selectBar">
                <div className="header-row">
                    <DropDown label="Which frameworks?" width='1024px'>
                        <DropDownContents {...frameworkSelectKeyed}>
                            <h3>Keyed frameworks:</h3>
                            <SelectBarFrameworks isSelected={frameworkSelectKeyed.isSelected} select={selectFramework} frameworks={frameworksKeyed} />
                        </DropDownContents>
                        <DropDownContents {...frameworkSelectNonKeyed}>
                            <h3>Non-keyed frameworks:</h3>
                            <SelectBarFrameworks isSelected={frameworkSelectNonKeyed.isSelected} select={selectFramework} frameworks={frameworksNonKeyed} />
                        </DropDownContents>
                    </DropDown>
                    <div className="hspan" />
                    <DropDown label="Which benchmarks?" width='300px'>
                        <SelectCategory benchmarks={benchmarksCPU} select={selectBenchmark} benchSelect={benchSelectCpu} label="Duration" />
                        <SelectCategory benchmarks={benchmarksStartup} select={selectBenchmark} benchSelect={benchSelectStartup} label="Startup" />
                        <SelectCategory benchmarks={benchmarksMEM} select={selectBenchmark} benchSelect={benchSelectMem} label="Memory" />
                    </DropDown>
                </div>
                <div className="header-row">
                    <div>
                        <label htmlFor="displayMode">Display mode</label>
                        <div className="hspan" />
                        <select id="displayMode" className="custom-select" value={displayMode.type} onChange={(evt) => selectDisplayMode(evt.target.value)}>
                            <option value={DisplayMode.DisplayMean}>Display results (mean results)</option>
                            <option value={DisplayMode.DisplayMedian}>Display results (median results)</option>
                            <option value={DisplayMode.CompareAgainst}>Compare results against one framework</option>
                            <option value={DisplayMode.HighlightVariance}>Colorize high variance</option>
                            <option value={DisplayMode.BoxPlot}>Display as box plots</option>
                        </select>
                        <div className="hspan" />
                        <input id='chb_nonKeyed' type="checkbox" onChange={(evt) => selectSeparateKeyedAndNonKeyed(evt.target.checked)} checked={separateKeyedAndNonKeyed} />
                        <label htmlFor='chb_nonKeyed'>
                            Separate keyed and non-keyed
                        </label>
                        {displayMode.type === DisplayMode.CompareAgainst &&
                            (<><div className="hspan"></div>
                                <select className="custom-select" value={(displayMode as DisplayModeCompare).compareAgainst ?
                                (displayMode as DisplayModeCompare).compareAgainst!.name : ''}
                            onChange={(evt) => selectComparison(evt.target.value)}>
                                    <option value=''>Compare with ...</option>
                                    <optgroup label="Keyed">
                                        {frameworksKeyed.map(f => <option key={f.name} value={f.name}>{f.name}</option>)}
                                    </optgroup>
                                    <optgroup label="Non-keyed">
                                        {frameworksNonKeyed.map(f => <option key={f.name} value={f.name}>{f.name}</option>)}
                                    </optgroup>
                                </select>
                            </>)}
                    </div>
                </div>
            </div>
        );
    }
}
