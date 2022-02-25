package communicator

import (
	"bytes"
	"encoding/gob"
	"fmt"
	"github.com/vmihailenco/msgpack/v5"
)

type Incoming_Message struct {
	Username []byte
	Data     []byte
}

func (IM Incoming_Message) Deserialize() []byte {
	var buffer bytes.Buffer
	encoder := gob.NewEncoder(&buffer)
	err := encoder.Encode(IM)
	if err != nil {
		fmt.Println("Error while encoding the message")
	}
	return buffer.Bytes()
}

func (IM Incoming_Message) ExampleMarshal() {
	
	b, err := msgpack.Marshal(IM)
	if err != nil {
		panic(err)
	}
	
	var item Incoming_Message
	err = msgpack.Unmarshal(b, &item)
	if err != nil {
		panic(err)
	}
	fmt.Println(IM)
	// Output: bar
}
