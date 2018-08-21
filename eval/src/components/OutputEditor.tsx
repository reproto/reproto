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
        name="output-editor"
        showGutter={true}
        mode={this.props.mode}
        theme="github"
        width="100%"
        maxLines={Infinity}
        value={this.props.value}
        setOptions={{autoScrollEditorIntoView: true}}
        editorProps={{$blockScrolling: false}}
        />
    );
  }
}
