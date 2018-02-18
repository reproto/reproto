import * as React from "react";
import {Input} from "./Input";
import {OutputEditor} from "./OutputEditor";
import * as Rust from "rust";
import {JavaSettings, JavaSettingsForm} from "./JavaSettings";
import {RustSettings, RustSettingsForm} from "./RustSettings";

const deepEqual = require("deep-equal");

const languages = [
  "yaml",
  "json",
  "java",
  "rust",
  "python",
  "javascript",
]

const themes = [
  "monokai",
  "github",
]

languages.forEach((lang) => {
  require(`brace/mode/${lang}`)
  require(`brace/snippets/${lang}`)
})

themes.forEach((theme) => {
  require(`brace/theme/${theme}`)
})

const DEFAULT_JSON = require("raw-loader!../static/default.json");
const DEFAULT_YAML = require("raw-loader!../static/default.yaml");
const logo = require("../static/logo.256.png");

interface Compiled {
  compiledContent: string;
  compiledRootName: string;
  compiledPackagePrefix: string;
  compiledFormat: Format;
  compiledOutput: Output;
  compiledSettings: Settings;
  result: DeriveResult;
}

interface Derive {
  content: string;
  root_name: string;
  format: string;
  output: string;
  package_prefix: string;
}

interface DeriveResult {
  result?: string;
  error?: string;
}

enum Format {
  Json = "json",
  Yaml = "yaml",
}

enum Output {
  Reproto = "reproto",
  Java = "java",
  Rust = "rust",
  Python = "python",
  JavaScript = "js",
  Json = "json",
}

export interface MainProps {
}

interface ContentSet {
  [key: string]: string;
}

interface Settings {
  java: JavaSettings;
  rust: RustSettings;
}

export interface MainState {
  contentSet: ContentSet;
  settings: Settings;
  format: Format;
  output: Output;
  rootName: string;
  packagePrefix: string;
  compiled?: Compiled;
  error?: string;
  derive?: (value: Derive) => DeriveResult;
}

export class Main extends React.Component<MainProps, MainState> {
  constructor(props: MainProps) {
    super(props);

    this.state = {
      contentSet: {
        json: DEFAULT_JSON,
        yaml: DEFAULT_YAML,
      },
      settings: {
        java: {
          jackson: true,
          lombok: true,
        },
        rust: {
          chrono: true,
        }
      },
      rootName: "Generated",
      packagePrefix: "io.github.reproto",
      format: Format.Json,
      output: Output.Rust,
    };
  }

  componentDidMount() {
    Rust.reproto_wasm.then((mod: any) => {
      this.setState({derive: mod.derive}, () => this.recompile());
    });
  }

  recompile() {
    let {
      derive,
      compiled,
      contentSet,
      format,
      output,
      rootName,
      packagePrefix,
      settings,
    } = this.state;

    let content = contentSet[format];

    if (!this.state.derive) {
      return;
    }

    let compile = true;
    let oldResult = null;

    if (compiled) {
      let {
        compiledContent,
        compiledRootName,
        compiledPackagePrefix,
        compiledFormat,
        compiledOutput,
        compiledSettings,
        result,
      } = compiled;

      oldResult = result;

      compile = compiledContent != content 
             || compiledRootName != rootName
             || compiledPackagePrefix != packagePrefix
             || compiledFormat != format
             || compiledOutput != output
             || !deepEqual(compiledSettings, settings);
    }

    if (compile) {
      const request = {
        content: content,
        root_name: rootName,
        package_prefix: packagePrefix,
        format: format,
        output: output,
        settings: settings,
      };

      const result = derive(request);

      // Don't hide old result.
      if (!result.result && oldResult) {
        result.result = oldResult.result;
      }

      const compiled: Compiled = {
        compiledContent: content,
        compiledRootName: rootName,
        compiledPackagePrefix: packagePrefix,
        compiledFormat: format,
        compiledOutput: output,
        compiledSettings: settings,
        result: result
      };

      this.setState({compiled: compiled});
    }
  }

  setContent(format: Format, value: string) {
    this.setState((state: MainState, props: MainProps) => {
      let contentSet: ContentSet = {...state.contentSet};
      contentSet[format] = value;
      return {contentSet: contentSet};
    }, () => this.recompile());
  }

  setFormat(value: string) {
    let format;

    switch (value) {
      case "yaml":
        format = "yaml" as Format;
        break;
      case "json":
        format = "json" as Format;
        break;
      default:
        return;
    }

    this.setState({
      format: format
    }, () => this.recompile());
  }

  setOutput(value: string) {
    let output;

    switch (value) {
      case "reproto":
        output = "reproto" as Output;
        break;
      case "java":
        output = "java" as Output;
        break;
      case "python":
        output = "python" as Output;
        break;
      case "rust":
        output = "rust" as Output;
        break;
      case "js":
        output = "js" as Output;
        break;
      case "json":
        output = "json" as Output;
        break;
      default:
        return;
    }

    this.setState({
      output: output
    }, () => this.recompile());
  }

  setRootName(rootName: string) {
    this.setState({
      rootName: rootName
    }, () => this.recompile());
  }

  setPackagePrefix(packagePrefix: string) {
    this.setState({
      packagePrefix: packagePrefix
    }, () => this.recompile());
  }

  updateJava(cb: (settings: JavaSettings) => void) {
    this.setState((state: MainState, props: MainProps) => {
      let settings = {...state.settings};
      settings.java = {...settings.java};
      cb(settings.java);
      return {settings: settings};
    }, () => this.recompile());
  }

  updateRust(cb: (settings: RustSettings) => void) {
    this.setState((state: MainState, props: MainProps) => {
      let settings = {...state.settings};
      settings.rust = {...settings.rust};
      cb(settings.rust);
      return {settings: settings};
    }, () => this.recompile());
  }

  render() {
    let {
      contentSet,
      compiled,
      format,
      output,
      rootName,
      packagePrefix,
      settings,
      derive,
    } = this.state;

    let content = contentSet[format];

    let errorMessage = null;
    let compiledResult = undefined;

    var wasmLoading = null;
    var settingsForm = null;

    if (!derive) {
      wasmLoading = (
        <div className="modal" role="dialog" style={{display: "block"}}>
          <div className="modal-dialog" role="document">
            <div className="modal-content">
              <div className="modal-body">
                <div style={{textAlign: "center"}}>
                  <div className="col-auto">
                    <i className="fa fa-spinner fa-spin" style={{fontSize: "24px"}} /><br />
                  </div>
                  <br />
                  <p>Loading WASM component...</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      );
    }

    if (format) {
      switch (output) {
        case "java":
          settingsForm = <JavaSettingsForm settings={settings.java}
            onJackson={update => this.updateJava(java => java.jackson = update)}
            onLombok={update => this.updateJava(java => java.lombok = update)}
            />;
          break;
        case "rust":
          settingsForm = <RustSettingsForm settings={settings.rust}
            onChrono={update => this.updateRust(rust => rust.chrono = update)}
            />;
          break;
        default:
          break;
      }
    }

    if (compiled) {
      let { error, result } = compiled.result;

      if (result) {
        compiledResult = result;
      }

      if (error) {
        errorMessage = (
          <div className="error row mt-2">
            <div className="col">
              <div className="alert alert-danger">{error}</div>
            </div>
          </div>
        );
      }
    }

    return (
      <div className="box">
        {wasmLoading}

        <div className="box-row header">
          <nav className="navbar navbar-expand-lg navbar-light bg-light">
            <a className="navbar-brand" href="https://github.com/reproto">
              <img src={logo} width={48} height={48} title="reproto" />
            </a>
            <a className="navbar-brand" href="#">reproto eval</a>
          </nav>

          <div className="container-fluid">
            <div className="row mb-2 mt-2">
              <div className="col">
                <form>
                  <div className="form-row">
                    <div className="col-auto">
                      <label htmlFor="format">Format:</label>

                      <select
                        id="format"
                        className="custom-select"
                        onChange={e => this.setFormat(e.target.value)}>
                        <option value="json">JSON</option>
                        <option value="yaml">YAML</option>
                      </select>
                    </div>
                    <div className="col-auto">
                      <label htmlFor="rootName">Generated Name:</label>

                      <input
                        id="rootName"
                        type="text"
                        className="form-control"
                        value={rootName}
                        onChange={e => this.setRootName(e.target.value)} />
                    </div>
                    <div className="col-auto">
                      <label htmlFor="packagePrefix">Package Prefix:</label>

                      <input
                        id="packagePrefix"
                        type="text"
                        className="form-control"
                        value={packagePrefix}
                        onChange={e => this.setPackagePrefix(e.target.value)} />
                    </div>

                    <div className="col-auto">
                      <label htmlFor="output">Output:</label>

                      <select
                        id="output"
                        className="custom-select"
                        value={output}
                        onChange={e => this.setOutput(e.target.value)}>
                        <option value="reproto">reproto</option>
                        <option value="java">Java</option>
                        <option value="python">Python</option>
                        <option value="js">JavaScript</option>
                        <option value="rust">Rust</option>
                        <option value="json">JSON</option>
                      </select>
                    </div>
                  </div>
                </form>
              </div>
            </div>

            <div className="row mb-2 mt-2">
              <div className="col">{settingsForm}</div>
            </div>
          </div>
        </div>

        <div className="box-row content">
          <div className="row editors">
            <div className="col-6 col-xl-4 input">
              <Input
                format={format as string}
                value={content}
                onChange={value => this.setContent(format, value)} />
            </div>

            <div className="col">
              {errorMessage}

              <OutputEditor
                format={output as string}
                value={compiledResult} />
            </div>
          </div>
        </div>
      </div>
    );
  }
}
