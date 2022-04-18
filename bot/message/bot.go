package message

import "crypto/cipher"

var (
	IsConnected bool
	Aead        cipher.AEAD
)

type Bot struct {
	Uid     []uint8
	Version [8]uint8
	Com     Commands
}

type Commands struct {
	Command []byte
	Data    []byte
}
