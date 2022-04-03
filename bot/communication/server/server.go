package server

import (
	msg "bot/message"
	"bufio"
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
	
	reader := bufio.NewReader(conn)
	var buffer bytes.Buffer
	for {
		ba, isPrefix, err := reader.ReadLine()
		if err != nil {
			// if the error is an End Of File this is still good
			if err == io.EOF {
				break;
			}
			return msg.Bot{}, err
		}
		buffer.Write(ba)
		if !isPrefix {
			break;
		}
	}
	return msg.Bot{}, nil;
}
