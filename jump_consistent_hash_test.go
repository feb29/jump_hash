package jump_hash_test

import (
	"testing"

	"github.com/feb29/jump_hash"
)

func TestSlot(t *testing.T) {
	t.Log(jump_hash.Slot(10863919174838991, 11))
	t.Log(jump_hash.Slot(2016238256797177309, 11))
	t.Log(jump_hash.Slot(1673758223894951030, 11))
	t.Log(jump_hash.Slot(2, 100001))
	t.Log(jump_hash.Slot(2201, 100001))
	t.Log(jump_hash.Slot(2202, 100001))
}

func BenchmarkSlot(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = jump_hash.Slot(10863919174838991, 11)
	}
}
