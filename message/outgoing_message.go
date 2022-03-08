package message

import (
	"bytes"
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
	return buf.Bytes()
}
