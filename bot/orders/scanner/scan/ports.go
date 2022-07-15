package scan

type PortState uint8

const (
	PortUnknown PortState = iota
	PortOpen
	PortClosed
	PortFiltered
)
