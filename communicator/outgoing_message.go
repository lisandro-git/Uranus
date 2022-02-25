package communicator

import (
	"bytes"
	"encoding/gob"
	"fmt"
	"github.com/vmihailenco/msgpack/v5"
)

type Outgoing_Message struct {
	Username []byte
	Data     []byte
}

func (OM Outgoing_Message) Marshal() []byte {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	err := enc.Encode(OM)
	if err != nil {
		fmt.Println("Error encoding:", err)
	}
	return buf.Bytes()
}

func (OM Outgoing_Message) ExampleMarshal() []byte {
	
	b, err := msgpack.Marshal(OM)
	if err != nil {
		panic(err)
	}
	fmt.Println(b)
	var item Outgoing_Message
	err = msgpack.Unmarshal(b, &item)
	if err != nil {
		panic(err)
	}
	fmt.Println(OM)
	// Output: bar
	return []byte{}
}
