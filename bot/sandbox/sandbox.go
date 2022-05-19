package main

import (
	"crypto/cipher"
	"crypto/rand"
	"fmt"
	ccp "golang.org/x/crypto/chacha20poly1305"
)

func encryptCCP(aead cipher.AEAD, msg []byte) []byte {
	// Select a random nonce, and leave capacity for the ciphertext.
	//generating a random nonce
	nonce := make([]byte, aead.NonceSize())
	if _, err := rand.Read(nonce); err != nil {
		panic(err)
	}

	fmt.Println("nonce:", nonce)
	// Encrypt the message and append the ciphertext to the nonce.
	return aead.Seal(nonce, nonce, msg, []byte("Edode"))
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
	var msg = []byte("fuck off u fat georgian")
	// [89, 206, 188, 150, 16, 216, 10, 113, 18, 155, 133, 139, 250, 89, 173, 150, 121, 222, 95, 207, 105, 12, 160, 75, 224, 59, 223, 96, 171, 42, 138, 194]
	var key = []byte{89, 206, 188, 150, 16, 216, 10, 113, 18, 155, 133, 139, 250, 89, 173, 150, 121, 222, 95, 207, 105, 12, 160, 75, 224, 59, 223, 96, 171, 42, 138, 194}

	aead, err := ccp.NewX(key)
	if err != nil {
		panic(err)
	}

	fmt.Println("key:", key)
	fmt.Println("Aead Key : ", aead)

	// Encryption.
	var encryptedMsg []byte = encryptCCP(aead, msg)
	fmt.Println("Encrypted message:", encryptedMsg)

}

/*
key : [89, 206, 188, 150, 16, 216, 10, 113, 18, 155, 133, 139, 250, 89, 173, 150, 121, 222, 95, 207, 105, 12, 160, 75, 224, 59, 223, 96, 171, 42, 138, 194]
nonce : [60, 206, 7, 66, 97, 87, 164, 12, 27, 95, 178, 72, 27, 119, 94, 229, 254, 88, 168, 78, 131, 31, 191, 159]
ciphertext: [139, 31, 95, 93, 116, 14, 104, 25, 161, 226, 244, 196, 195, 30, 130, 69, 108, 253, 242, 232, 232, 142, 107, 246, 176, 40, 161, 145, 60]
plaintext: "Hello, world!"
*/
