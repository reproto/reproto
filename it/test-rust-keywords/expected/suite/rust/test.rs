#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "as")]
  _as: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "break")]
  _break: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "const")]
  _const: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "continue")]
  _continue: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "crate")]
  _crate: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "else")]
  _else: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "enum")]
  _enum: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "extern")]
  _extern: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "false")]
  _false: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "fn")]
  _fn: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "for")]
  _for: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "if")]
  _if: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "impl")]
  _impl: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "in")]
  _in: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "let")]
  _let: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "loop")]
  _loop: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "match")]
  _match: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "mod")]
  _mod: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "move")]
  _move: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "mut")]
  _mut: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "pub")]
  _pub: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "ref")]
  _ref: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "return")]
  _return: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "self")]
  _self: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "static")]
  _static: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "struct")]
  _struct: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "super")]
  _super: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "trait")]
  _trait: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "true")]
  _true: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "unsafe")]
  _unsafe: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "use")]
  _use: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "where")]
  _where: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "while")]
  _while: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "abstract")]
  _abstract: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "alignof")]
  _alignof: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "become")]
  _become: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "box")]
  _box: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "do")]
  _do: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "final")]
  _final: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "macro")]
  _macro: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "offsetof")]
  _offsetof: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "override")]
  _override: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "priv")]
  _priv: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "proc")]
  _proc: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "pure")]
  _pure: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "sizeof")]
  _sizeof: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "typeof")]
  _typeof: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "unsized")]
  _unsized: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "virtual")]
  _virtual: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "yield")]
  _yield: Option<String>,
}
