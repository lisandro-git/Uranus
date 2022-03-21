package message

import (
	"bot/morse"
	"bytes"
	"encoding/base32"
	"github.com/vmihailenco/msgpack"
)

type Bot struct {
	Uid  [16]byte
	Data Outgoing_Message
}

type Outgoing_Message struct {
	Username []byte
	Data     []byte
	Command  []byte
}

func (OM Outgoing_Message) Marshal() []byte {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf).StructAsArray(true)
	err := enc.Encode(&OM)
	if err != nil {
		panic(err)
	}
	return buf.Bytes()
}

func (OM Outgoing_Message) Obfuscate_data() []byte {
	var encrypted_data = Encrypt_data(OM.Marshal())
	var encoded_data = base32.StdEncoding.EncodeToString(encrypted_data)
	return morse.Encode(encoded_data)
}
