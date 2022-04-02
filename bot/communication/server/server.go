package server

import (
	"fmt"
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

