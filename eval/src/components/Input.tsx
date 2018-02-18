import * as React from 'react';
import AceEditor from 'react-ace';
import {Row, Col} from 'react-bootstrap';

export interface InputProps {
}

export class Input extends React.Component<InputProps, {}> {
    render() {
        return [
          <div className="row">
            <div className="col">
              <h1>Input</h1>

              <AceEditor />
            </div>
          </div>
        ];
    }
}
