package scan

import (
	"net"
	"time"
)

type Result struct {
	Host         net.IP
	Open         []int
	Closed       []int
	Filtered     []int
	Manufacturer string
	MAC          string
	Latency      time.Duration
	Name         string
}

func NewResult(host net.IP) Result {
	return Result{
		Host:     host,
		Open:     []int{},
		Closed:   []int{},
		Filtered: []int{},
		Latency:  -1,
	}
}

func (r Result) IsHostUp() bool {
	return r.Latency > -1
}
