import * as React from 'react';
import {DropdownCallback} from './Common'

interface Props<T> extends DropdownCallback<T> {
  children: Array<JSX.Element>;
}

export function DropDownContents<T>(props: Props<T>) {
  let {selectNone, selectAll, isNoneSelected, areAllSelected, children} = props;
  return <div className="section">
            {children[0]}
            <div className="float-rt">
                {!isNoneSelected() ? <a href='#' onClick={selectNone}>None</a> : <span>None</span>}
                &nbsp;
                {!areAllSelected() ? <a href='#' onClick={selectAll}>All</a> : <span>All</span>}
            </div>
            <div className="grid">
                {children[1]}
            </div>
        </div>;
}
