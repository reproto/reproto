package test

import "encoding/json"
import "errors"

type Entry struct {
  Tagged *Tagged `json:"tagged,omitempty"`

  Untagged *Untagged `json:"untagged,omitempty"`
}

type Tagged struct {
  A *Tagged_A
  B *Tagged_B
  Bar *Tagged_Bar
  Baz *Tagged_Baz
}

type Tagged_A struct {
  Shared string `json:"shared"`
}

type Tagged_B struct {
  Shared string `json:"shared"`
}

type Tagged_Bar struct {
  Shared string `json:"shared"`
}

type Tagged_Baz struct {
  Shared string `json:"shared"`
}

func (this *Tagged) UnmarshalJSON(b []byte) error {
  var err error
  var ok bool
  env := make(map[string]json.RawMessage)

  if err := json.Unmarshal(b, &env); err != nil {
    return err
  }

  var raw_tag json.RawMessage

  if raw_tag, ok = env["@type"]; !ok {
    return errors.New("missing tag")
  }

  var tag string

  if err = json.Unmarshal(raw_tag, &tag); err != nil {
    return err
  }

  switch (tag) {
  case "foo":
    sub := Tagged_A{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.A = &sub
    return nil
  case "b":
    sub := Tagged_B{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.B = &sub
    return nil
  case "Bar":
    sub := Tagged_Bar{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Bar = &sub
    return nil
  case "Baz":
    sub := Tagged_Baz{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Baz = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this Tagged) MarshalJSON() ([]byte, error) {
  var b []byte
  var err error
  env := make(map[string]json.RawMessage)

  if this.A != nil {
    if b, err = json.Marshal(&this.A); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["@type"], err = json.Marshal("foo"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  }

  if this.B != nil {
    if b, err = json.Marshal(&this.B); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["@type"], err = json.Marshal("b"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  }

  if this.Bar != nil {
    if b, err = json.Marshal(&this.Bar); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["@type"], err = json.Marshal("Bar"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  }

  if this.Baz != nil {
    if b, err = json.Marshal(&this.Baz); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["@type"], err = json.Marshal("Baz"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  }

  return nil, errors.New("no sub-type set")
}

type Untagged struct {
  A *Untagged_A
  B *Untagged_B
  C *Untagged_C
}

// Special case: fields shared with other sub-types.
// NOTE: due to rust support through untagged, the types are matched in-order.
type Untagged_A struct {
  Shared string `json:"shared"`

  SharedIgnore *string `json:"shared_ignore,omitempty"`

  A string `json:"a"`

  B string `json:"b"`

  Ignore *string `json:"ignore,omitempty"`
}

type Untagged_B struct {
  Shared string `json:"shared"`

  SharedIgnore *string `json:"shared_ignore,omitempty"`

  A string `json:"a"`

  Ignore *string `json:"ignore,omitempty"`
}

type Untagged_C struct {
  Shared string `json:"shared"`

  SharedIgnore *string `json:"shared_ignore,omitempty"`

  B string `json:"b"`

  Ignore *string `json:"ignore,omitempty"`
}

func (this *Untagged) UnmarshalJSON(b []byte) error {
  var err error
  env := make(map[string]json.RawMessage)

  if err := json.Unmarshal(b, &env); err != nil {
    return err
  }

  keys := make(map[string]bool)

  for k := range env {
    keys[k] = true
  }

  var all bool

  all = true
  for _, k := range([]string{"shared", "a", "b"}) {
    if _, all = keys[k]; !all {
      break
    }
  }

  if all {
    sub := Untagged_A{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.A = &sub
    return nil
  }

  all = true
  for _, k := range([]string{"shared", "a"}) {
    if _, all = keys[k]; !all {
      break
    }
  }

  if all {
    sub := Untagged_B{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.B = &sub
    return nil
  }

  all = true
  for _, k := range([]string{"shared", "b"}) {
    if _, all = keys[k]; !all {
      break
    }
  }

  if all {
    sub := Untagged_C{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.C = &sub
    return nil
  }

  return errors.New("no combination of fields found")
}

func (this Untagged) MarshalJSON() ([]byte, error) {
  if this.A != nil {
    return json.Marshal(&this.A)
  }

  if this.B != nil {
    return json.Marshal(&this.B)
  }

  if this.C != nil {
    return json.Marshal(&this.C)
  }

  return nil, errors.New("no sub-type set")
}
