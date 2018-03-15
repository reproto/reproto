import * as React from 'react';
import AceEditor from 'react-ace';

export interface RustSettings {
  chrono: boolean;
}

export interface RustSettingsFormProps {
  settings: RustSettings;
  onChrono: (update: boolean) => void;
}

export class RustSettingsForm extends React.Component<RustSettingsFormProps, {}> {
  render() {
    let { settings } = this.props;

    return (
      <form>
      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.chrono}
          onChange={e => this.props.onChrono(e.target.checked)}
          id="java-chrono" />

        <label htmlFor="java-chrono">Chrono Support</label>

        <small id="java-chrono-help" className="form-text form-text-sm text-muted">
          Support for <a href="https://github.com/chronotope/chrono">Chrono</a> (Required for <code>datetime</code> fields)
        </small>
      </div>
      </form>
    );
  }
}
