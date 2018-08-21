import * as React from 'react';

export interface OpenApiSettings {
  json: boolean;
}

export interface OpenApiSettingsFormProps {
  settings: OpenApiSettings;
  onJson: (update: boolean) => void;
}

export class OpenApiSettingsForm extends React.Component<OpenApiSettingsFormProps, {}> {
  render() {
    let { settings } = this.props;

    return (
      <form>
      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.json}
          onChange={e => this.props.onJson(e.target.checked)}
          id="rust-json" />

        <label htmlFor="rust-json">JSON</label>

        <small id="rust-json-help" className="form-text form-text-sm text-muted">
          Output as JSON
        </small>
      </div>
      </form>
    );
  }
}
