package test

import "encoding/json"
import "errors"

type Entry struct {
}

type RootType struct {
}

type RootInterface struct {
  Value interface {
    IsRootInterface()
  }
}

type RootInterface_Foo struct {
}

func (this RootInterface_Foo) IsRootInterface() {
}

func (this *RootInterface) UnmarshalJSON(b []byte) error {
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
  case "Foo":
    sub := RootInterface_Foo{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Value = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this RootInterface) MarshalJSON() ([]byte, error) {
  var b []byte
  var err error
  env := make(map[string]json.RawMessage)

  switch v := this.Value.(type) {
  case *RootInterface_Foo:
    if b, err = json.Marshal(v); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["type"], err = json.Marshal("Foo"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  default:
    return nil, errors.New("RootInterface: no sub-type set")
  }
}

type RootEnum int

const (
  RootEnum_Foo RootEnum = iota
)

func (this *RootEnum) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "Foo":
    *this = RootEnum_Foo
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this RootEnum) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case RootEnum_Foo:
    s = "Foo"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type RootTuple struct {
}

func (this *RootTuple) UnmarshalJSON(b []byte) error {
  var array []json.RawMessage

  if err := json.Unmarshal(b, &array); err != nil {
    return err
  }


  return nil
}

func (this RootTuple) MarshalJSON() ([]byte, error) {
  var array []json.RawMessage


  return json.Marshal(array)
}

type RootType_NestedType struct {
}

type RootType_NestedInterface struct {
  Value interface {
    IsRootType_NestedInterface()
  }
}

type RootType_NestedInterface_Foo struct {
}

func (this RootType_NestedInterface_Foo) IsRootType_NestedInterface() {
}

func (this *RootType_NestedInterface) UnmarshalJSON(b []byte) error {
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
  case "Foo":
    sub := RootType_NestedInterface_Foo{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Value = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this RootType_NestedInterface) MarshalJSON() ([]byte, error) {
  var b []byte
  var err error
  env := make(map[string]json.RawMessage)

  switch v := this.Value.(type) {
  case *RootType_NestedInterface_Foo:
    if b, err = json.Marshal(v); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["type"], err = json.Marshal("Foo"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  default:
    return nil, errors.New("RootType_NestedInterface: no sub-type set")
  }
}

type RootType_NestedEnum int

const (
  RootType_NestedEnum_Foo RootType_NestedEnum = iota
)

func (this *RootType_NestedEnum) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "Foo":
    *this = RootType_NestedEnum_Foo
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this RootType_NestedEnum) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case RootType_NestedEnum_Foo:
    s = "Foo"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type RootType_NestedTuple struct {
}

func (this *RootType_NestedTuple) UnmarshalJSON(b []byte) error {
  var array []json.RawMessage

  if err := json.Unmarshal(b, &array); err != nil {
    return err
  }


  return nil
}

func (this RootType_NestedTuple) MarshalJSON() ([]byte, error) {
  var array []json.RawMessage


  return json.Marshal(array)
}

type RootInterface_Foo_NestedType struct {
}

type RootInterface_Foo_NestedInterface struct {
  Value interface {
    IsRootInterface_Foo_NestedInterface()
  }
}

type RootInterface_Foo_NestedInterface_NestedFoo struct {
}

func (this RootInterface_Foo_NestedInterface_NestedFoo) IsRootInterface_Foo_NestedInterface() {
}

func (this *RootInterface_Foo_NestedInterface) UnmarshalJSON(b []byte) error {
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
  case "NestedFoo":
    sub := RootInterface_Foo_NestedInterface_NestedFoo{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Value = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this RootInterface_Foo_NestedInterface) MarshalJSON() ([]byte, error) {
  var b []byte
  var err error
  env := make(map[string]json.RawMessage)

  switch v := this.Value.(type) {
  case *RootInterface_Foo_NestedInterface_NestedFoo:
    if b, err = json.Marshal(v); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["type"], err = json.Marshal("NestedFoo"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  default:
    return nil, errors.New("RootInterface_Foo_NestedInterface: no sub-type set")
  }
}

type RootInterface_Foo_NestedEnum int

const (
  RootInterface_Foo_NestedEnum_Foo RootInterface_Foo_NestedEnum = iota
)

func (this *RootInterface_Foo_NestedEnum) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "Foo":
    *this = RootInterface_Foo_NestedEnum_Foo
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this RootInterface_Foo_NestedEnum) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case RootInterface_Foo_NestedEnum_Foo:
    s = "Foo"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type RootInterface_Foo_NestedTuple struct {
}

func (this *RootInterface_Foo_NestedTuple) UnmarshalJSON(b []byte) error {
  var array []json.RawMessage

  if err := json.Unmarshal(b, &array); err != nil {
    return err
  }


  return nil
}

func (this RootInterface_Foo_NestedTuple) MarshalJSON() ([]byte, error) {
  var array []json.RawMessage


  return json.Marshal(array)
}

type RootTuple_NestedType struct {
}

type RootTuple_NestedInterface struct {
  Value interface {
    IsRootTuple_NestedInterface()
  }
}

type RootTuple_NestedInterface_Foo struct {
}

func (this RootTuple_NestedInterface_Foo) IsRootTuple_NestedInterface() {
}

func (this *RootTuple_NestedInterface) UnmarshalJSON(b []byte) error {
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
  case "Foo":
    sub := RootTuple_NestedInterface_Foo{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Value = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this RootTuple_NestedInterface) MarshalJSON() ([]byte, error) {
  var b []byte
  var err error
  env := make(map[string]json.RawMessage)

  switch v := this.Value.(type) {
  case *RootTuple_NestedInterface_Foo:
    if b, err = json.Marshal(v); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["type"], err = json.Marshal("Foo"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  default:
    return nil, errors.New("RootTuple_NestedInterface: no sub-type set")
  }
}

type RootTuple_NestedEnum int

const (
  RootTuple_NestedEnum_Foo RootTuple_NestedEnum = iota
)

func (this *RootTuple_NestedEnum) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "Foo":
    *this = RootTuple_NestedEnum_Foo
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this RootTuple_NestedEnum) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case RootTuple_NestedEnum_Foo:
    s = "Foo"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type RootTuple_NestedTuple struct {
}

func (this *RootTuple_NestedTuple) UnmarshalJSON(b []byte) error {
  var array []json.RawMessage

  if err := json.Unmarshal(b, &array); err != nil {
    return err
  }


  return nil
}

func (this RootTuple_NestedTuple) MarshalJSON() ([]byte, error) {
  var array []json.RawMessage


  return json.Marshal(array)
}

type RootService_NestedType struct {
}

type RootService_NestedInterface struct {
  Value interface {
    IsRootService_NestedInterface()
  }
}

type RootService_NestedInterface_Foo struct {
}

func (this RootService_NestedInterface_Foo) IsRootService_NestedInterface() {
}

func (this *RootService_NestedInterface) UnmarshalJSON(b []byte) error {
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
  case "Foo":
    sub := RootService_NestedInterface_Foo{}

    if err = json.Unmarshal(b, &sub); err != nil {
      return err
    }

    this.Value = &sub
    return nil
  default:
    return errors.New("bad tag")
  }
}

func (this RootService_NestedInterface) MarshalJSON() ([]byte, error) {
  var b []byte
  var err error
  env := make(map[string]json.RawMessage)

  switch v := this.Value.(type) {
  case *RootService_NestedInterface_Foo:
    if b, err = json.Marshal(v); err != nil {
      return nil, err
    }

    if err = json.Unmarshal(b, &env); err != nil {
      return nil, err
    }

    if env["type"], err = json.Marshal("Foo"); err != nil {
      return nil, err
    }

    return json.Marshal(env)
  default:
    return nil, errors.New("RootService_NestedInterface: no sub-type set")
  }
}

type RootService_NestedEnum int

const (
  RootService_NestedEnum_Foo RootService_NestedEnum = iota
)

func (this *RootService_NestedEnum) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "Foo":
    *this = RootService_NestedEnum_Foo
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this RootService_NestedEnum) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case RootService_NestedEnum_Foo:
    s = "Foo"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type RootService_NestedTuple struct {
}

func (this *RootService_NestedTuple) UnmarshalJSON(b []byte) error {
  var array []json.RawMessage

  if err := json.Unmarshal(b, &array); err != nil {
    return err
  }


  return nil
}

func (this RootService_NestedTuple) MarshalJSON() ([]byte, error) {
  var array []json.RawMessage


  return json.Marshal(array)
}

type RootType_NestedInterface_Foo_Nested struct {
}

type RootType_NestedTuple_Nested struct {
}

type RootType_NestedService_Nested struct {
}

type RootInterface_Foo_NestedInterface_NestedFoo_Nested struct {
}

type RootInterface_Foo_NestedTuple_Nested struct {
}

type RootInterface_Foo_NestedService_Nested struct {
}

type RootTuple_NestedInterface_Foo_Nested struct {
}

type RootTuple_NestedTuple_Nested struct {
}

type RootTuple_NestedService_Nested struct {
}

type RootService_NestedInterface_Foo_Nested struct {
}

type RootService_NestedTuple_Nested struct {
}

type RootService_NestedService_Nested struct {
}
