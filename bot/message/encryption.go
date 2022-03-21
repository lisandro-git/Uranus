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

// import_public_key Parse public key from file
func import_public_key() (*rsa.PublicKey, error) {
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

// Encrypt_data Encrypt message after it has been serialized
func Encrypt_data(data []byte) ([]byte) {
	public_key, err := import_public_key()
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

// import_private_key Imports private key from file
func import_private_key() (*rsa.PrivateKey, error) {
	priv_pem, err := ioutil.ReadFile("../private.key")
	if err != nil {
		log.Fatal(err)
	}
	
	block, _ := pem.Decode([]byte(priv_pem))
	if block == nil {
		return nil, errors.New("failed to parse PEM block containing the key")
	}
	
	priv, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		return nil, err
	}
	
	return priv, nil
}

// Decrypt_data Decrypt message after it has been received and de-obfuscated
func Decrypt_data(data []byte) []byte {
	private_key, err := import_private_key()
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
