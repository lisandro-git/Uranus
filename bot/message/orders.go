package message

import (
	"bot/morse"
	"bytes"
	"encoding/base32"
	"github.com/vmihailenco/msgpack"
)

type Bot struct {
	Uid  [16]uint8
	Version [8]uint8
	Com    Commands
}

type Commands struct {
	Command  []byte
	Data []byte
}

func (b *Bot) unmarshal() []byte {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf)
	enc.Encode(b)
	return buf.Bytes()
}

func (b *Bot) Deobfuscate_data() []byte {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf)
	enc.Encode(b)
	return buf.Bytes()
}

func (b *Bot) marshal() []byte {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf).StructAsArray(true)
	err := enc.Encode(&b)
	if err != nil {
		panic(err)
	}
	return buf.Bytes()
}

func (b *Bot) Obfuscate_data() []byte {
	var encrypted_data = Encrypt_data(b.marshal())
	var encoded_data = base32.StdEncoding.EncodeToString(encrypted_data)
	return morse.Encode(encoded_data)
}
