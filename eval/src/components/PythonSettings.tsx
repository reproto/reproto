import * as React from 'react';
import AceEditor from 'react-ace';

export interface PythonSettings {
  requests: boolean;
}

export interface PythonSettingsFormProps {
  settings: PythonSettings;
  onRequests: (update: boolean) => void;
}

export class PythonSettingsForm extends React.Component<PythonSettingsFormProps, {}> {
  render() {
    let { settings } = this.props;

    return (
      <form>
      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.requests}
          onChange={e => this.props.onRequests(e.target.checked)}
          id="python-requests" />

        <label htmlFor="python-requests" className="lb-sm">Requests Support</label>

        <small id="python-requests-help" className="form-text form-text-sm text-muted">
          Support for <a href="http://docs.python-requests.org">Requests</a>
        </small>
      </div>
      </form>
    );
  }
}
