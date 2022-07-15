package message

import (
	"crypto/cipher"
	"crypto/rand"
	ccp "golang.org/x/crypto/chacha20poly1305"
)

func init() {
	// Generate a random key
	if _, err := rand.Read(Key); err != nil {
		panic(err)
	}
	Aead = GenerateAead(Key)
	B.GenerateRandomUid()
}

var (
	Key                  = make([]byte, ccp.KeySize)
	FirstConnection bool = true
	Aead            cipher.AEAD
	B               Bot
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
