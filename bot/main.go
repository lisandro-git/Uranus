package main

import (
	"bot/message"
	"bufio"
	"fmt"
	r "math/rand"
	"net"
	"os"
	"strings"
	"sync"
	"time"
)

const (
	HOST      = "localhost"
	PORT      = "6000"
	TYPE      = "tcp4"
	Msgsize   = 4096
	UidLength = 16
	letterBytes = "0123456789abcdef"
	letterIdxBits = 6                    // 6 bits to represent a letter index
	letterIdxMask = 1<<letterIdxBits - 1 // All 1-bits, as many as letterIdxBits
	letterIdxMax  = 63 / letterIdxBits   // # of letter indices fitting in 63 bits

)

var (
	wg sync.WaitGroup
	O message.Bot
	src = r.NewSource(time.Now().UnixNano())
)

func messageInput(m string) ([]byte) {
	for {
		fmt.Printf("%s -> ", m)
		in := bufio.NewReader(os.Stdin)
		
		line, err := in.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading input")
			os.Exit(1)
		} else {
			return []byte(strings.TrimSuffix(line, "\n"))
		}
	};
}

func connectToServer() (net.Conn) {
	server, err := net.Dial(TYPE, HOST+":"+PORT)
	if err != nil {
		fmt.Println("Error connecting to : ", err.Error())
		os.Exit(1)
	}
	fmt.Println("Connected to " + HOST + ":" + PORT)
	return server;
}

func generateRandomUid() []byte {
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
	
	return sb.Bytes()
}

func client(server net.Conn) () {
	for {
		O.Com.Command = messageInput("Command")
		O.Com.Data    = messageInput("Data")
		
		x, err := server.Write(O.ObfuscateData())
		if err != nil {
			return;
		} else {
			fmt.Println("Sent ", x, " bytes")
		}
	}
}

func main()() {
	var server net.Conn = connectToServer()
	defer server.Close()
	
	wg.Add(1)
	O.Uid = generateRandomUid()
	go client(server)
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
