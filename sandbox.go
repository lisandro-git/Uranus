package main

import (
	"bytes"
	"fmt"
	"github.com/vmihailenco/msgpack/v5"
)



func main() {
	type Human struct {
		_msgpack struct{} `msgpack:",as_array"`
		Name     []byte
	}
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf)
	err := enc.Encode(&Human{Name: []byte"John"})
	if err != nil {
		panic(err)
	}
	fmt.Println(buf.Bytes())
}




























