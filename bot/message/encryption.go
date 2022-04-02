package message

import (
	"crypto"
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
	public_key, err := ioutil.ReadFile("../public.key")
	if err != nil {
		return nil, err
	}
	
	block, _ := pem.Decode(public_key)
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
	public_key, err := importPublicKey()
	if err != nil {
		fmt.Println("Error: ", err)
		return nil;
	}
	encryptedBytes, err := rsa.EncryptPKCS1v15(rand.Reader, public_key, data)
	/*encryptedBytes, err := rsa.EncryptOAEP(
		sha256.New(),
		rand.Reader,
		public_key,
		data,
		nil)*/
	
	if err != nil {
		panic(err)
	}
	return encryptedBytes;
}

// importPrivateKey Imports private key from file
func importPrivateKey() (*rsa.PrivateKey, error) {
	priv_pem, err := ioutil.ReadFile("../private.key")
	if err != nil {
		log.Fatal(err)
	}
	
	block, _ := pem.Decode(priv_pem)
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
	private_key, err := importPrivateKey()
	if err != nil {
		log.Fatal(err)
	}
	
	decrypted_bytes, err := private_key.Decrypt(
		nil,
		data,
		&rsa.OAEPOptions{Hash: crypto.SHA256})
	
	if err != nil {
		panic(err)
	}
	return decrypted_bytes
}
