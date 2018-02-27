import * as React from 'react';
import AceEditor from 'react-ace';

export interface CsharpSettings {
  json_net: boolean;
}

export interface CsharpSettingsFormProps {
  settings: CsharpSettings;
  onJsonNet: (update: boolean) => void;
}

export class CsharpSettingsForm extends React.Component<CsharpSettingsFormProps, {}> {
  render() {
    let { settings } = this.props;

    return (
      <div className="form-row">
        <div className="col-auto">
          <div className="form-check">
            <input
              className="form-check-input"
              type="checkbox"
              checked={settings.json_net}
              onChange={e => this.props.onJsonNet(e.target.checked)}
              id="java-json-net" />

            <label htmlFor="java-json-net" className="lb-sm">Json.NET Support</label>
          </div>
        </div>
      </div>
    );
  }
}
