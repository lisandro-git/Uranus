package cmd

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"strings"
	"time"

	"bot/orders/scanner/scan"
)

var timeoutMS int = 2000
var parallelism int = 500
var portSelection string
var scanType = "stealth"
var hideUnavailableHosts bool
var versionRequested bool

func createScanner(ti *scan.TargetIterator, scanTypeStr string, timeout time.Duration, routines int) (scan.Scanner, error) {
	switch strings.ToLower(scanTypeStr) {
	case "stealth":
		if os.Geteuid() > 0 {
			return nil, fmt.Errorf("Access Denied: You must be a priviliged user to run this type of scan.")
		}
		return scan.NewSynScanner(ti, timeout, routines), nil
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

		results, err := scanner.Scan(ctx, scan.DefaultPorts)
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
