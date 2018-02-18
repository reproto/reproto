import * as React from 'react';
import AceEditor from 'react-ace';
import {Row, Col} from 'react-bootstrap';

export interface OutputProps {
}

export class Output extends React.Component<OutputProps, {}> {
    render() {
        return [
          <div className="row">
            <div className="col">
              <h1>Output</h1>

              <AceEditor />
            </div>
          </div>
        ];
    }
}
