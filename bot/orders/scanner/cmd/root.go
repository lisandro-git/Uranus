package cmd

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"strings"
	"time"

	"bot/orders/scanner/scan"
	log "github.com/sirupsen/logrus"
)

var debug bool
var timeoutMS int = 2000
var parallelism int = 500
var portSelection string
var scanType = "connect"
var hideUnavailableHosts bool
var versionRequested bool
var ports []int

func init() {
	for i := 1; i <= 1024; i++ {
		ports = append(ports, i)
	}
}

func createScanner(ti *scan.TargetIterator, scanTypeStr string, timeout time.Duration, routines int) (scan.Scanner, error) {
	switch strings.ToLower(scanTypeStr) {
	case "connect":
		return scan.NewConnectScanner(ti, timeout, routines), nil
	}
	return nil, fmt.Errorf("Unknown scan type '%s'", scanTypeStr)
}

func Execute(args []string) {
	ctx, cancel := context.WithCancel(context.Background())

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		<-c
		fmt.Println("Scan cancelled. Requesting stop...")
		cancel()
	}()

	startTime := time.Now()
	fmt.Printf("\nStarting scan at %s\n\n", startTime.String())

	for _, target := range args {

		targetIterator := scan.NewTargetIterator(target)

		// creating scanner
		scanner, err := createScanner(targetIterator, scanType, time.Millisecond*time.Duration(timeoutMS), parallelism)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		log.Debugf("Starting scanner...")
		if err := scanner.Start(); err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		log.Debugf("Scanning target %s...", target)

		results, err := scanner.Scan(ctx, ports)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		for _, result := range results {
			if !hideUnavailableHosts || result.IsHostUp() {
				scanner.OutputResult(result)
			}
		}

	}

	fmt.Printf("Scan complete in %s.\n", time.Since(startTime).String())

}
