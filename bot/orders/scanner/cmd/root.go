package cmd

import (
	"bot/orders/scanner/scan"
	"context"
	"fmt"
	"os"
	"os/signal"
	"time"
)

var (
	timeoutMS   int = 2000
	parallelism int = 500
)

func ExecuteScan(args []string) []scan.Result {
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
	var x = []scan.Result{}
	for _, target := range args {

		targetIterator := scan.NewTargetIterator(target)

		scanner := scan.NewConnectScanner(targetIterator, time.Millisecond*time.Duration(timeoutMS), parallelism)
		if err := scanner.Start(); err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		results, err := scanner.Scan(ctx, scan.Ports)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		x = append(x, results...)
	}
	fmt.Printf("Scan complete in %s.\n", time.Since(startTime).String())
	return x
}
