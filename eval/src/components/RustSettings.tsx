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
        <div className="form-row">
          <div className="col-auto">
            <div className="form-check">
              <input
                className="form-check-input"
                type="checkbox"
                checked={settings.chrono}
                onChange={e => this.props.onChrono(e.target.checked)}
                id="java-chrono" />

              <label htmlFor="java-chrono"><a href="https://docs.rs/chrono">Chrono</a> Support</label>
            </div>
          </div>
        </div>
      </form>
    );
  }
}
