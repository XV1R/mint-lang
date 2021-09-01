package main

import (
	"fmt"
	"mint/repl"
	"os"
	"os/user"
)

func main() {
	user, err := user.Current()
	if err != nil {
		panic(err)
	}
	fmt.Printf("MINT LANGUAGE\nWELCOME, %s\n", user.Username)
	repl.Start(os.Stdin, os.Stdout)
}
