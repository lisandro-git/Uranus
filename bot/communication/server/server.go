package server

import (
	msg "bot/message"
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

func ReadCommands(conn net.Conn, B *msg.Bot) (msg.Bot, error) {
	var buffer [4096]byte
	for {
		read, err := conn.Read(buffer[:])
		if err != nil {
			if err == io.EOF {
				return msg.Bot{}, nil;
			} else {
				return msg.Bot{}, err;
			}
		}
		B.DeobfuscateData(buffer[:read])
		fmt.Println("deobfuscated data : ", B)
	}
}
