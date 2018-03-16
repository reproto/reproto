package test

type Entry interface {
  isEntry()
}

type Entry_A struct {
  Shared string `json:"shared"`
}

func (this Entry_A) isEntry() {
}

type Entry_B struct {
  Shared string `json:"shared"`
}

func (this Entry_B) isEntry() {
}

type Entry_Bar struct {
  Shared string `json:"shared"`
}

func (this Entry_Bar) isEntry() {
}

type Entry_Baz struct {
  Shared string `json:"shared"`
}

func (this Entry_Baz) isEntry() {
}

func (this Entry) UnmarshalJSON(b []byte) error {
}

func (this Entry) MarshalJSON() ([]byte, error) {
}
