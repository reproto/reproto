import * as React from 'react';
import {Input} from './Input';
import {Output} from './Output';

export interface MainProps {
}

export class Main extends React.Component<MainProps, {}> {
    render() {
        return (
          <div className="container-fluid">
            <div className="row">
              <div className="col-4 input">
                <Input />
              </div>
              <div className="col output">
                <Output />
              </div>
            </div>
          </div>
        );
    }
}
