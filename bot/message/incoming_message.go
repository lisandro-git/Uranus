package message

import (
	"bytes"
	"github.com/mitchellh/mapstructure"
	"github.com/vmihailenco/msgpack"
)

type Incoming_Message struct {
	Data     []byte
	Command  []byte
}

func (IM Incoming_Message) Deserialize() (Incoming_Message) {
	var buf bytes.Buffer
	dec := msgpack.NewDecoder(&buf)
	v, err := dec.DecodeInterface()
	if err != nil {
		panic(err)
	}
	
	err = mapstructure.Decode(v, &IM)
	if err != nil {
		panic(err)
	}
	return IM;
}
