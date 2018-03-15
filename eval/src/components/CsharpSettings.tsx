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
      <form>
      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.json_net}
          onChange={e => this.props.onJsonNet(e.target.checked)}
          id="csharp-json-net" />

        <label htmlFor="csharp-json-net" className="lb-sm">Json.NET Support</label>

        <small id="csharp-json-net-help" className="form-text form-text-sm text-muted">
          Support for <a href="https://www.newtonsoft.com/json">Newtonsoft.Json (Json.NET)</a>
        </small>
      </div>
      </form>
    );
  }
}
