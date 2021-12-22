import * as React from 'react';
import {Benchmark} from './Common';

interface Props {
  benchmarks: Array<Benchmark>;
  isSelected: (benchmark: Benchmark) => boolean;
  select: (benchmark: Benchmark, value: boolean) => void;
}

export const SelectBarBenchmarks = (props: Props) => {
    return (
      <div>
        {props.benchmarks.map(item =>
            <div key={item.id} className="col-md-12">
                <div className="form-check">
                    <input id={'inp-'+item.id} className="form-check-input" type="checkbox" onChange={(evt) => props.select(item, evt.target.checked)} checked={props.isSelected(item)} />
                    <label htmlFor={'inp-'+item.id} className="form-check-label">
                    {item.label}
                    </label>
                </div>
            </div>
        )}
      </div>);
};