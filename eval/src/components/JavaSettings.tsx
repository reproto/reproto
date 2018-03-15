import * as React from 'react';
import AceEditor from 'react-ace';

export interface JavaSettings {
  jackson: boolean;
  lombok: boolean;
}

export interface JavaSettingsFormProps {
  settings: JavaSettings;
  onJackson: (update: boolean) => void;
  onLombok: (update: boolean) => void;
}

export class JavaSettingsForm extends React.Component<JavaSettingsFormProps, {}> {
  render() {
    let { settings } = this.props;

    return (
      <form>
      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.jackson}
          onChange={e => this.props.onJackson(e.target.checked)}
          id="java-jackson" />

        <label htmlFor="java-jackson" className="lb-sm">Jackson Support</label>

        <small id="java-jackson-help" className="form-text form-text-sm text-muted">
          Support for <a href="https://github.com/FasterXML/jackson">Jackson</a>
        </small>
      </div>

      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.lombok}
          onChange={e => this.props.onLombok(e.target.checked)}
          id="java-lombok" />

        <label htmlFor="java-lombok" className="lb-sm">Lombok</label>

        <small id="java-lombok-help" className="form-text form-text-sm text-muted">
          Support for <a href="https://projectlombok.org/">Lombok Annotations</a>
        </small>
      </div>
      </form>
    );
  }
}
