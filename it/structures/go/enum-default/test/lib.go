package test

import "encoding/json"
import "errors"

type Entry struct {
  Explicit *EnumExplicit `json:"explicit,omitempty"`

  Implicit *EnumImplicit `json:"implicit,omitempty"`

  EnumU32 *EnumU32 `json:"enum_u32,omitempty"`

  EnumU64 *EnumU64 `json:"enum_u64,omitempty"`

  EnumI32 *EnumI32 `json:"enum_i32,omitempty"`

  EnumI64 *EnumI64 `json:"enum_i64,omitempty"`
}

// Explicitly assigned strings
type EnumExplicit int

const (
  EnumExplicit_A EnumExplicit = iota
  EnumExplicit_B
)

func (this *EnumExplicit) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "foo":
    *this = EnumExplicit_A
  case "bar":
    *this = EnumExplicit_B
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this EnumExplicit) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case EnumExplicit_A:
    s = "foo"
  case EnumExplicit_B:
    s = "bar"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

// Implicit naming depending on the variant
type EnumImplicit int

const (
  EnumImplicit_A EnumImplicit = iota
  EnumImplicit_B
)

func (this *EnumImplicit) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "A":
    *this = EnumImplicit_A
  case "B":
    *this = EnumImplicit_B
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this EnumImplicit) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case EnumImplicit_A:
    s = "A"
  case EnumImplicit_B:
    s = "B"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

// Variants with long names.
type EnumLongNames int

const (
  EnumLongNames_FooBar EnumLongNames = iota
  EnumLongNames_Baz
)

func (this *EnumLongNames) UnmarshalJSON(b []byte) error {
  var s string

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case "FooBar":
    *this = EnumLongNames_FooBar
  case "Baz":
    *this = EnumLongNames_Baz
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this EnumLongNames) MarshalJSON() ([]byte, error) {
  var s string

  switch this {
  case EnumLongNames_FooBar:
    s = "FooBar"
  case EnumLongNames_Baz:
    s = "Baz"
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type EnumU32 int

const (
  EnumU32_Min EnumU32 = iota
  EnumU32_Max
)

func (this *EnumU32) UnmarshalJSON(b []byte) error {
  var s uint32

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case 0:
    *this = EnumU32_Min
  case 2147483647:
    *this = EnumU32_Max
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this EnumU32) MarshalJSON() ([]byte, error) {
  var s uint32

  switch this {
  case EnumU32_Min:
    s = 0
  case EnumU32_Max:
    s = 2147483647
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type EnumU64 int

const (
  EnumU64_Min EnumU64 = iota
  EnumU64_Max
)

func (this *EnumU64) UnmarshalJSON(b []byte) error {
  var s uint64

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case 0:
    *this = EnumU64_Min
  case 9007199254740991:
    *this = EnumU64_Max
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this EnumU64) MarshalJSON() ([]byte, error) {
  var s uint64

  switch this {
  case EnumU64_Min:
    s = 0
  case EnumU64_Max:
    s = 9007199254740991
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type EnumI32 int

const (
  EnumI32_Min EnumI32 = iota
  EnumI32_NegativeOne
  EnumI32_Zero
  EnumI32_Max
)

func (this *EnumI32) UnmarshalJSON(b []byte) error {
  var s int32

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case -2147483648:
    *this = EnumI32_Min
  case -1:
    *this = EnumI32_NegativeOne
  case 0:
    *this = EnumI32_Zero
  case 2147483647:
    *this = EnumI32_Max
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this EnumI32) MarshalJSON() ([]byte, error) {
  var s int32

  switch this {
  case EnumI32_Min:
    s = -2147483648
  case EnumI32_NegativeOne:
    s = -1
  case EnumI32_Zero:
    s = 0
  case EnumI32_Max:
    s = 2147483647
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}

type EnumI64 int

const (
  EnumI64_Min EnumI64 = iota
  EnumI64_NegativeOne
  EnumI64_Zero
  EnumI64_Max
)

func (this *EnumI64) UnmarshalJSON(b []byte) error {
  var s int64

  if err := json.Unmarshal(b, &s); err != nil {
    return err
  }

  switch s {
  case -9007199254740991:
    *this = EnumI64_Min
  case -1:
    *this = EnumI64_NegativeOne
  case 0:
    *this = EnumI64_Zero
  case 9007199254740991:
    *this = EnumI64_Max
  default:
    return errors.New("bad value")
  }

  return nil
}

func (this EnumI64) MarshalJSON() ([]byte, error) {
  var s int64

  switch this {
  case EnumI64_Min:
    s = -9007199254740991
  case EnumI64_NegativeOne:
    s = -1
  case EnumI64_Zero:
    s = 0
  case EnumI64_Max:
    s = 9007199254740991
  default:
    return nil, errors.New("bad value")
  }

  return json.Marshal(s)
}
