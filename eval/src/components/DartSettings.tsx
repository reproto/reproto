import * as React from 'react';

export interface DartSettings {
}

export interface DartSettingsFormProps {
  settings: DartSettings;
}

export class DartSettingsForm extends React.Component<DartSettingsFormProps, {}> {
  render() {
    let { settings } = this.props;

    return (<form></form>);
  }
}
