package message

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"errors"
	"fmt"
	"io/ioutil"
	"log"
)

// importPublicKey Parse public key from file
func importPublicKey() (*rsa.PublicKey, error) {
	publicKey, err := ioutil.ReadFile("../public.key")
	if err != nil {
		return nil, err
	}
	
	block, _ := pem.Decode(publicKey)
	if block == nil {
		return nil, errors.New("failed to parse PEM block containing the key")
	}
	
	pub, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		return nil, err
	}
	
	switch pub := pub.(type) {
	case *rsa.PublicKey:
		return pub, nil
	default:
		break // fall through
	}
	return nil, errors.New("Key type is not RSA")
}

// EncryptData Encrypt message after it has been serialized
func EncryptData(data []byte) ([]byte) {
	publicKey, err := importPublicKey()
	if err != nil {
		fmt.Println("Error: ", err)
		return nil;
	}
	encryptedBytes, err := rsa.EncryptPKCS1v15(rand.Reader, publicKey, data)
	
	if err != nil {
		panic(err)
	}
	return encryptedBytes;
}

// importPrivateKey Imports private key from file
func importPrivateKey() (*rsa.PrivateKey, error) {
	privPem, err := ioutil.ReadFile("../private.key")
	if err != nil {
		log.Fatal(err)
	}
	
	block, _ := pem.Decode(privPem)
	if block == nil {
		return nil, errors.New("failed to parse PEM block containing the key")
	}
	
	priv, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		return nil, err
	}
	
	return priv, nil
}

// DecryptData Decrypt message after it has been received and de-obfuscated
func DecryptData(data []byte) []byte {
	privateKey, err := importPrivateKey()
	if err != nil {
		log.Fatal(err)
	}
	
	decryptedBytes, err := rsa.DecryptPKCS1v15(rand.Reader, privateKey, data)
	
	if err != nil {
		panic(err)
	}
	return decryptedBytes
}
