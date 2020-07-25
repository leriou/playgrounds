package backtrack_test

import (
	"testing"

	bt "../backtrack"
)

func TestNqueen(t *testing.T) {

	c := bt.Nqueen()
	if c != 92 {
		t.Error("c -> ", c)
	}
}
