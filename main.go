package main

import (
	com "Uranus/message"
	"fmt"
	"net"
	"os"
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
	OM com.Outgoing_Message
	IM com.Incoming_Message
)

func message_input() ([]byte) {
	var input []byte
	for {
		fmt.Println("Enter a message: ")
		fmt.Scanln(&input)
		if len(input) > 0 {
			return input;
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
	OM.Data = message_input()
	var data []byte = OM.Marshal(OM)
	fmt.Println("Sending message: ", data, " len : ", len(data))
	_, err := server.Write(data)
	if err != nil {
		return ;
	}
}

func main()() {

	var server net.Conn = connect_to_server()
	defer server.Close()

	// declare variable from Messages.go
	OM.Username = message_input()

	wg.Add(1)
	go Client(server)
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
