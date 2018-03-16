import * as React from 'react';
import AceEditor from 'react-ace';

export interface OutputEditorProps {
  mode: string;
  value: string;
}

export class OutputEditor extends React.Component<OutputEditorProps, {}> {
  render() {
    return (
      <AceEditor
        showGutter={true}
        mode={this.props.mode}
        readOnly={true}
        theme="github"
        width="100%"
        maxLines={Infinity}
        value={this.props.value}
        editorProps={{$blockScrolling: false}}
        />
    );
  }
}
