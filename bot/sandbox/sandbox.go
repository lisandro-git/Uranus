package main

import (
	"crypto/cipher"
	"crypto/rand"
	"fmt"
	ccp "golang.org/x/crypto/chacha20poly1305"
)

var (
	key = make([]byte, ccp.KeySize)
)

func encryptCCP(aead cipher.AEAD, msg []byte) []byte {
	// Select a random nonce, and leave capacity for the ciphertext.
	nonce := make([]byte, aead.NonceSize(), aead.NonceSize()+len(msg)+aead.Overhead())
	if _, err := rand.Read(nonce); err != nil {
		panic(err)
	}

	// Encrypt the message and append the ciphertext to the nonce.
	return aead.Seal(nonce, nonce, msg, nil)
}

func decryptCCP(aead cipher.AEAD, encryptedMsg []byte) []byte {
	// Split nonce and ciphertext.
	nonce, ciphertext := encryptedMsg[:aead.NonceSize()], encryptedMsg[aead.NonceSize():]

	// Decrypt the message and check it wasn't tampered with.
	plaintext, err := aead.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		panic(err)
	}

	return plaintext
}

func main() {
	var msg = []byte("Hello, world!")

	if _, err := rand.Read(key); err != nil {
		panic(err)
	}

	aead, err := ccp.NewX(key)
	if err != nil {
		panic(err)
	}

	// Encryption.
	var encryptedMsg []byte = encryptCCP(aead, msg)

	fmt.Println(string(decryptCCP(aead, encryptedMsg)))
}
