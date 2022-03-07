package main

import (
	"Uranus/message"
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
	OM message.Outgoing_Message
	IM message.Incoming_Message
)

func message_input(m string) ([]byte) {
	var input []byte
	for {
		fmt.Printf("%s -> ", m)
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
	OM.Data = message_input("data")
	
	var enc_data = message.Encrypt_data(OM.Marshal(OM))
	_, err := server.Write(enc_data)
	if err != nil {
		return;
	} else {
		fmt.Printf("Data sent : %d", enc_data)
	}
}

func main()() {

	var server net.Conn = connect_to_server()
	defer server.Close()

	// declare variable from Messages.go
	OM.Username = message_input("username")
	OM.Command  = message_input("command")

	wg.Add(1)
	go func() {
		for {
			Client(server)
		}
	}()
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
