import * as React from 'react';
import AceEditor from 'react-ace';
import {Annotation, Marker} from 'react-ace';

export interface InputProps {
  mode: string;
  value: string;
  annotations: Annotation[];
  markers: Marker[];
  onChange: (value: string) => void;
}

export class Input extends React.Component<InputProps, {}> {
  render() {
    return (
      <AceEditor
        tabSize={2}
        showGutter={true}
        mode={this.props.mode}
        theme="monokai"
        width="100%"
        height="100%"
        value={this.props.value}
        annotations={this.props.annotations}
        markers={this.props.markers}
        onChange={this.props.onChange.bind(this)}
        />
    );
  }
}
