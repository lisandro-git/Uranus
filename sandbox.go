package main

import (
	"bytes"
	"fmt"
	"github.com/vmihailenco/msgpack"
)

type Human struct {
	Age  uint8
	Name string
}


func main() {
	
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf).UseCompactEncoding(true)//.StructAsArray(true).UseCompactEncoding(false)
	err := enc.Encode(&Human{Age: 69, Name: "Gopher"})
	if err != nil {
		panic(err)
	}
	fmt.Println(buf.Bytes())
	
	dec := msgpack.NewDecoder(&buf)
	v, err := dec.DecodeInterface()
	if err != nil {
		panic(err)
	}
	// put the result of v inside the Human struct
	
	fmt.Println(v)


}
