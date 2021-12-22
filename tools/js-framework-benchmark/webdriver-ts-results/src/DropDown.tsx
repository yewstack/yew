import * as React from 'react';

interface Props {
  label: string;
  children: JSX.Element | JSX.Element[];
  width: string;
}

export class DropDown extends React.Component<Props,{open: boolean}> {
    constructor(props: Props) {
        super(props);
        this.state = {open: false};
    }
    public toggle = (event: React.SyntheticEvent<HTMLElement>) => {
        event.stopPropagation();
        this.setState((state,props) => {
            return {open: !state.open}
        })
      }
    public render() {
        let {label, children, width} = this.props;
        return (<div className={(this.state.open ? 'open dropdown-container' : 'dropdown-container')}>
          <button type="button" onClick={this.toggle} className={(this.state.open ? 'open dropdown' : 'dropdown')}>
            {label} <span className="caret"></span>
          </button>
          <div className="shutter" onClick={this.toggle}></div>
          <div className={(this.state.open ? 'show ' : '') +'dropdown-menu'} style={{width: width}}>
            {children}
          </div>
        </div>);
    }
};
