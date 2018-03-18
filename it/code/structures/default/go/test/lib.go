package test

import "encoding/json"
import "errors"

type Entry struct {
}

type Type struct {
}

type Interface struct {
  SubType *Interface_SubType
}

type Interface_SubType struct {
}

func (this *Interface) UnmarshalJSON(b []byte) error {
  var err error
  var ok bool
  env := make(map[string]json.RawMessage)

  if err := json.Unmarshal(b, &env); err != nil {
    return err
  }

  var raw_tag json.RawMessage

  if raw_tag, ok = env["type"]; !ok {
    return errors.New("missing tag")
  }

  var tag string

  if err = json.Unmarshal(raw_tag, &tag); err != nil {
    return err
  }

  switch (tag) {
  case "SubType":
    sub := Interface_SubType{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.SubType = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this Interface) MarshalJSON() ([]byte, error) {
  var b []byte
  var err error
  env := make(map[string]json.RawMessage)

  if this.SubType != nil {
    if b, err = json.Marshal(&this.SubType); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["type"], err = json.Marshal("SubType"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  }

  return nil, errors.New("no sub-type set")
}

type Enum int

const (
  Enum_Variant Enum = iota
)

func (this *Enum) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "Variant":
    *this = Enum_Variant
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this Enum) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case Enum_Variant:
    s = "Variant"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type Tuple struct {
}

func (this *Tuple) UnmarshalJSON(b []byte) error {
  var array []json.RawMessage

  if err := json.Unmarshal(b, &array); err != nil {
    return err
  }


  return nil
}

func (this Tuple) MarshalJSON() ([]byte, error) {
  var array []json.RawMessage


  return json.Marshal(array)
}
