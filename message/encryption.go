package message

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/pem"
	"errors"
	"fmt"
)

func parse_public_key() (*rsa.PublicKey, error) {
	var public_key string
	public_key =
		`-----BEGIN RSA PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDUk85fHLJoX+mskL088GAQHEJC
qcf6UK+h+l3ffiCB1Wc16U024FqRmg0Eu6f4IHA9gapXlWNdp/1cnPsobK90E+o1
DwZqn5ewreCkm7RavxcV7GTx75gzumGHfOhhz5yb74ICvdE79UcKifJNjM+AXc74
43WaXvuLTxsFE7nGGwIDAQAB
-----END RSA PUBLIC KEY-----
`
	block, _ := pem.Decode([]byte(public_key))
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

func Encrypt_data(data []byte) ([]byte) {
	public_key, err := parse_public_key()
	if err != nil {
		fmt.Println("Error: ", err)
		return nil;
	}
	encryptedBytes, err := rsa.EncryptOAEP(
		sha256.New(),
		rand.Reader,
		public_key,
		data,
		nil)
	
	if err != nil {
		panic(err)
	}
	return encryptedBytes;
}
