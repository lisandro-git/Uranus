package server

import (
	msg "bot/message"
	"bytes"
	"fmt"
	"io"
	"net"
	"os"
)

const (
	LOCALHOST = "localhost"
	LOCALPORT = ":6969"
	TYPE      = "tcp4"
)

func StartLocalServer() net.Listener {
	listener, err := net.Listen(TYPE, LOCALHOST+LOCALPORT)
	if err != nil {
		fmt.Println("Error listening: ", err.Error())
		os.Exit(1)
	}
	fmt.Println("Listening on " + LOCALHOST + LOCALPORT)
	return listener;
}

func ReadCommands(conn net.Conn) (msg.Bot, error) {
	defer conn.Close()
	
	var buffer [4096]byte
	for {
		read, err := conn.Read(buffer[:])
		if err != nil {
			return msg.Bot{}, err
		}
		fmt.Println("Deobfuscated data : ", msg.DeobfuscateData(bytes.Trim(buffer[:read], "\x00")))
		
		if err != nil {
			// if the error is an End Of File this is still good
			if err == io.EOF {
				break;
			}
			return msg.Bot{}, err
		}
	}
	return msg.Bot{}, nil;
}
