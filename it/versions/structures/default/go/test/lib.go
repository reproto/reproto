package test

import "../foo_v4"

type Entry struct {
  Thing *foo_v4.Thing `json:"thing,omitempty"`
}
