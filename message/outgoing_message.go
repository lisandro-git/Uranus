package message

import (
	"bytes"
	"github.com/vmihailenco/msgpack"
)

type Outgoing_Message struct {
	Username []byte
	Data []byte
}

func (OM Outgoing_Message) Marshal(message Outgoing_Message) ([]byte) {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf).UseCompactEncoding(true)
	err := enc.Encode(&message)
	if err != nil {
		panic(err)
	}
	return buf.Bytes()
}

