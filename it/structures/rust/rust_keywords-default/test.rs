use _trait as t;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "abstract")]
  pub _abstract: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "alignof")]
  pub _alignof: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "as")]
  pub _as: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "become")]
  pub _become: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "box")]
  pub _box: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "break")]
  pub _break: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "const")]
  pub _const: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "continue")]
  pub _continue: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "crate")]
  pub _crate: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "do")]
  pub _do: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "else")]
  pub _else: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "enum")]
  pub _enum: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "extern")]
  pub _extern: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "false")]
  pub _false: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "final")]
  pub _final: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "fn")]
  pub _fn: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "for")]
  pub _for: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "if")]
  pub _if: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "impl")]
  pub _impl: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub imported: Option<t::Empty>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "in")]
  pub _in: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "let")]
  pub _let: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "loop")]
  pub _loop: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "macro")]
  pub _macro: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "match")]
  pub _match: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "mod")]
  pub _mod: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "move")]
  pub _move: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "mut")]
  pub _mut: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "offsetof")]
  pub _offsetof: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "override")]
  pub _override: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "priv")]
  pub _priv: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "proc")]
  pub _proc: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "pub")]
  pub _pub: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "pure")]
  pub _pure: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "ref")]
  pub _ref: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "return")]
  pub _return: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "self")]
  pub _self: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "sizeof")]
  pub _sizeof: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "static")]
  pub _static: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "struct")]
  pub _struct: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "super")]
  pub _super: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "trait")]
  pub _trait: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "true")]
  pub _true: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "type")]
  pub _type: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "typeof")]
  pub _typeof: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "unsafe")]
  pub _unsafe: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "unsized")]
  pub _unsized: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "use")]
  pub _use: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "virtual")]
  pub _virtual: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "where")]
  pub _where: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "while")]
  pub _while: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "yield")]
  pub _yield: Option<String>,
}
