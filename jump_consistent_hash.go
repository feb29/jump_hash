package jump_hash

func Slot(k uint64, n int) uint32 {
	if n <= 0 {
		n = 1
	}
	var b, j int64
	for j < int64(n) {
		b = j
		k = k*2862933555777941757 + 1
		j = int64(float64(b+1) * ((1 << 31) / float64((k>>33)+1)))
	}
	return uint32(b)
}
