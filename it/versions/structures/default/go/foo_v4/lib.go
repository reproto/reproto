package foo_v4

import "../bar_v1"
import "../bar_v2_0"
import "../bar_v2_1"

type Thing struct {
  Name *string `json:"name,omitempty"`

  Other *bar_v1.Other `json:"other,omitempty"`

  Other2 *bar_v2_0.Other `json:"other2,omitempty"`

  Other21 *bar_v2_1.Other `json:"other21,omitempty"`
}
