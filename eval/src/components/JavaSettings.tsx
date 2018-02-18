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
        <div className="form-row">
          <div className="col-auto">
            <div className="form-check">
              <input
                className="form-check-input"
                type="checkbox"
                checked={settings.jackson}
                onChange={e => this.props.onJackson(e.target.checked)}
                id="java-jackson" />

              <label htmlFor="java-jackson">Jackson Support</label>
            </div>
          </div>

          <div className="col-auto">
            <div className="form-check">
              <input
                className="form-check-input"
                type="checkbox"
                checked={settings.lombok}
                onChange={e => this.props.onLombok(e.target.checked)}
                id="java-lombok" />

              <label htmlFor="java-lombok">Lombok</label>
            </div>
          </div>
        </div>
      </form>
    );
  }
}
