package client

import (
	"fmt"
	"net"
)

const (
	REMOTEHOST = "localhost"
	REMORTPORT = ":6000"
	TYPE       = "tcp4"
)

func ConnectToCommandingC2() (net.Conn) {
	server, err := net.Dial(TYPE, REMOTEHOST+REMORTPORT)
	if err != nil {
		fmt.Println("Error connecting to commanding C2 server :", err.Error())
	} else {
		fmt.Println("Connected to " + REMOTEHOST + REMORTPORT)
	}
	return server;
}
