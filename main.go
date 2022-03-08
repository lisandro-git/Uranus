package main

import (
	"Uranus/message"
	"bufio"
	"fmt"
	"net"
	"os"
	"strings"
	"sync"
)

const (
	HOST            = "localhost"
	PORT            = "6000"
	TYPE            = "tcp4"
	MSG_SIZE        = 4096
	USERNAME_LENGTH = 10
)

var (
	wg sync.WaitGroup
	OM message.Outgoing_Message
	IM message.Incoming_Message
)

func message_input(m string) ([]byte) {
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

func connect_to_server() (net.Conn) {
	server, err := net.Dial(TYPE, HOST+":"+PORT)
	if err != nil {
		fmt.Println("Error connecting to : ", err.Error())
		os.Exit(1)
	}
	fmt.Println("Connected to " + HOST + ":" + PORT)
	return server;
}

func Client (server net.Conn) () {
	for {
		OM.Command = message_input("Command")
		OM.Data    = message_input("Data")
		var enc_data = message.Encrypt_data(OM.Marshal())
		
		x, err := server.Write(enc_data)
		if err != nil {
			return;
		} else {
			fmt.Println("Sent ", x, " bytes")
		}
	}
}

func main()() {
	var server net.Conn = connect_to_server()
	defer server.Close()

	// declare variable from Messages.go
	OM.Username = message_input("username")
	OM.Command  = message_input("command")

	wg.Add(1)
	go Client(server)
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
