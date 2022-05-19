package client

import (
	msg "bot/message"
	"bufio"
	"fmt"
	"net"
	"os"
	"strings"
)

const (
	REMOTEHOST = "localhost"
	REMORTPORT = ":6000"
	TYPE       = "tcp4"
)

func ConnectToCommandingC2() net.Conn {
	server, err := net.Dial(TYPE, REMOTEHOST+REMORTPORT)
	if err != nil {
		fmt.Println("Error connecting to commanding C2 server :", err.Error())
	} else {
		fmt.Println("Connected to " + REMOTEHOST + REMORTPORT)
	}
	return server
}

func WriteData(server net.Conn, B *msg.Bot) {
	for {
		if msg.FirstConnection {
			B.Com.Command = []byte("")
			B.Com.Data = msg.Key
		} else {
			B.Com.Data = messageInput("Data")
			B.Com.Command = []byte("")
		}

		x, err := server.Write(B.ObfuscateData())

		if err != nil {
			return
		} else {
			fmt.Println("	Bytes sent ", x, " bytes")
		}
	}

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
			return []byte(strings.TrimSuffix(line, "\n"))
		}
	}
}
