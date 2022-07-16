package scan

type PortState uint8

const (
	PortUnknown PortState = iota
	PortOpen
	PortClosed
	PortFiltered
)

var Ports = []int{}

func init() {
	for i := 1; i <= 1024; i++ {
		Ports = append(Ports, i)
	}
}
