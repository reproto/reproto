import * as React from 'react';

export interface SwiftSettings {
  codable: boolean;
  simple: boolean;
}

export interface SwiftSettingsFormProps {
  settings: SwiftSettings;
  onCodable: (update: boolean) => void;
  onSimple: (update: boolean) => void;
}

export class SwiftSettingsForm extends React.Component<SwiftSettingsFormProps, {}> {
  render() {
    let { settings } = this.props;

    return (
      <form>
      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.codable}
          onChange={e => this.props.onCodable(e.target.checked)}
          id="swift-codable" />

        <label htmlFor="swift-codable" className="lb-sm">
          Codable Support
        </label>

        <small id="swift-codable-help" className="form-text form-text-sm text-muted">
          Support for Swift <a href="https://developer.apple.com/documentation/swift/codable">Codable</a>
        </small>
      </div>

      <div className="form-check mb-2">
        <input
          className="form-check-input"
          type="checkbox"
          checked={settings.simple}
          onChange={e => this.props.onSimple(e.target.checked)}
          id="swift-simple" />

        <label htmlFor="swift-simple" className="lb-sm">Simple</label>

        <small id="swift-codable-help" className="form-text form-text-sm text-muted">
          Simple and explicit Encoding/Decoding
        </small>
      </div>
      </form>
    );
  }
}
