package message

import (
	"bot/morse"
	"bytes"
	"encoding/base32"
	"github.com/vmihailenco/msgpack"
)

type Bot struct {
	Uid  []uint8
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

func (b *Bot) DeobfuscateData() []byte {
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

func (b *Bot) ObfuscateData() []byte {
	var encrypted_data = EncryptData(b.marshal())
	var encoded_data = base32.StdEncoding.EncodeToString(encrypted_data)
	return morse.Encode(encoded_data)
}
