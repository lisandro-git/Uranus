package message

import (
	"crypto/cipher"
	"crypto/rand"
	ccp "golang.org/x/crypto/chacha20poly1305"
)

func EncryptCCP(msg []byte) []byte {
	// Select a random nonce, and leave capacity for the ciphertext.
	nonce := make([]byte, Aead.NonceSize(), Aead.NonceSize()+len(msg)+Aead.Overhead())
	if _, err := rand.Read(nonce); err != nil {
		panic(err)
	}

	// Encrypt the message and append the ciphertext to the nonce.
	return Aead.Seal(nonce, nonce, msg, nil)
}

func DecryptCCP(encryptedMsg []byte) []byte {
	// Split nonce and ciphertext.
	nonce, ciphertext := encryptedMsg[:Aead.NonceSize()], encryptedMsg[Aead.NonceSize():]

	// Decrypt the message and check it wasn't tampered with.
	plaintext, err := Aead.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		panic(err)
	}
	return plaintext
}

func GenerateAead(key []byte) cipher.AEAD {
	aead, err := ccp.NewX(key)
	if err != nil {
		panic(err)
	}
	return aead
}
