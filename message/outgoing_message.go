package message

import (
	"bytes"
	"fmt"
	"github.com/vmihailenco/msgpack"
)

type Outgoing_Message struct {
	Username []byte
	Data     []byte
	Command  []byte
}

func (OM Outgoing_Message) Marshal() ([]byte) {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf).UseCompactEncoding(true)
	err := enc.Encode(&OM)
	if err != nil {
		panic(err)
	}
	fmt.Println("Outgoing_Message marshalled " + buf.String())
	return buf.Bytes()
}
