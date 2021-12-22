import * as React from 'react';
import {Framework} from './Common';

interface Props {
  frameworks: Array<Framework>;
  isSelected: (framework: Framework) => boolean;
  select: (framework: Framework, value: boolean) => void;
}

export const SelectBarFrameworks = (props: Props) => {
    return (
      <>
        {props.frameworks.map(item =>
            <div key={item.name} className="col-3">
                <input className="form-check-input" id={'inp-'+item.name+'-'+item.keyed}
                    type="checkbox"
                    onChange={(evt) => props.select(item, evt.target.checked)}
                    checked={props.isSelected(item)}
                />
                <label htmlFor={'inp-'+item.name+'-'+item.keyed} className="form-check-label">
                    {item.name}
                </label>
            </div>
        )}
      </>);
};