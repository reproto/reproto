package test

import "encoding/json"
import "errors"

type Entry struct {
  A *Entry_A
  B *Entry_B
  Bar *Entry_Bar
  Baz *Entry_Baz
}

type Entry_A struct {
  Shared string `json:"shared"`
}

type Entry_B struct {
  Shared string `json:"shared"`
}

type Entry_Bar struct {
  Shared string `json:"shared"`
}

type Entry_Baz struct {
  Shared string `json:"shared"`
}

func (this *Entry) UnmarshalJSON(b []byte) error {
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
    sub := Entry_A{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.A = &sub
    return nil
  case "b":
    sub := Entry_B{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.B = &sub
    return nil
  case "Bar":
    sub := Entry_Bar{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Bar = &sub
    return nil
  case "Baz":
    sub := Entry_Baz{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Baz = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this Entry) MarshalJSON() ([]byte, error) {
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
