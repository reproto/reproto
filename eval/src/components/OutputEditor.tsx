import * as React from 'react';
import AceEditor from 'react-ace';

export interface OutputEditorProps {
  format: string;
  value: string;
}

export class OutputEditor extends React.Component<OutputEditorProps, {}> {
  render() {
    return (
      <AceEditor
        showGutter={false}
        mode={this.props.format}
        readOnly={true}
        theme="github"
        width="100%"
        height="100%"
        value={this.props.value}
        editorProps={{$blockScrolling: false}}
        />
    );
  }
}
