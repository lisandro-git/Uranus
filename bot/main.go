package main

import (
	"bot/message"
	"bufio"
	"bytes"
	"fmt"
	"io"
	r "math/rand"
	"net"
	"os"
	"strings"
	"sync"
	"time"
)

const (
	REMOTEHOST = "localhost"
	REMORTPORT = ":6000"
	
	LOCALHOST  = "localhost"
	LOCALPORT  = ":6969"
	
	TYPE       = "tcp4"
	Msgsize    = 4096
	UidLength  = 16
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

// generateRandomUid generates a random hexadecimal string
// of length 16 used to identify the bot
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
	
	return sb.Bytes();
}

func messageInput(m string) []byte {
	for {
		fmt.Printf("%s -> ", m)
		in := bufio.NewReader(os.Stdin)
		
		line, err := in.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading input")
			os.Exit(1)
		} else {
			return []byte(strings.TrimSuffix(line, "\n"));
		}
	};
}

func connectToCommandingC2() (net.Conn) {
	server, err := net.Dial(TYPE, REMOTEHOST + REMORTPORT)
	if err != nil {
		fmt.Println("Error connecting to commanding C2 server :", err.Error())
	} else {
		fmt.Println("Connected to " + REMOTEHOST + REMORTPORT)
	}
	return server;
}

func tryConnect()() {
	defer wg.Done()
	var remoteServer net.Conn = connectToCommandingC2()
	if remoteServer != nil {
		defer remoteServer.Close()
	} else {
		return;
	}
	write(remoteServer)
}

func startLocalServer() net.Listener {
	listener, err := net.Listen(TYPE, LOCALHOST + LOCALPORT)
	if err != nil {
		fmt.Println("Error listening: ", err.Error())
		os.Exit(1)
	}
	fmt.Println("Listening on " + LOCALHOST + LOCALPORT)
	return listener;
}

func read(conn net.Conn) (string, error) {
	defer conn.Close()
	
	reader := bufio.NewReader(conn)
	var buffer bytes.Buffer
	for {
		ba, isPrefix, err := reader.ReadLine()
		if err != nil {
			// if the error is an End Of File this is still good
			if err == io.EOF {
				break;
			}
			return "", err
		}
		buffer.Write(ba)
		if !isPrefix {
			break;
		}
	}
	return buffer.String(), nil;
}

func write(server net.Conn) () {
	for {
		O.Com.Data = messageInput("Data")
		O.Com.Command = []byte("");
		
		x, err := server.Write(O.ObfuscateData())
		if err != nil {
			return;
		} else {
			fmt.Println("Sent ", x, " bytes")
		}
	}
}

func main() () {
	O.Uid = generateRandomUid()
	
	wg.Add(1)
	go tryConnect()
	
	wg.Add(1)
	go func() {
		defer wg.Done()
		
		var listener net.Listener = startLocalServer()
		defer listener.Close()
		
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("Error accepting: ", err.Error())
			os.Exit(1)
		}
		fmt.Println("Accepted connection from ", conn.RemoteAddr())
		read(conn)
	}()
	
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
