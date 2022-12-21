package main

import (
	"fmt"
	"example.com/greetings"
	//"rsc.io/quote"
	"log"
)

func main() {
	// Set properties of the predefined Logger, including
    // the log entry prefix and a flag to disable printing
    // the time, source file, and line number.
    log.SetPrefix("greetings: ")
    log.SetFlags(0)

    //fmt.Println("Hello, World!")
	//fmt.Println(quote.Go())

	message, err := greetings.Hello("Gladys")
    // If an error was returned, print it to the console and
    // exit the program.
    if err != nil {
        log.Fatal(err)
    }

    // If no error was returned, print the returned message
    // to the console.
    fmt.Println(message)


	// A slice of names.
    names := []string{"Gladys", "Samantha", "Darrin"}

	// Request greeting messages for the names.
    messages, err := greetings.Hellos(names)

	// If an error was returned, print it to the console and
    // exit the program.
    if err != nil {
        log.Fatal(err)
    }
	// If no error was returned, print the returned map of
    // messages to the console.
    fmt.Println(messages)
}