package main

import (
	"fmt"
)

type User struct {
	username      string
	email         string
	sign_in_count int
	active        bool
}

func changeName(name *string, newName string) {
	*name = newName
}

func insideGoRoutines(channel chan User) {
	user1 := User{"someone", "someone@example.com", 1, true}
	aName := &user1.username
	channel <- user1
	changeName(aName, "notme") //should not be allowed concurrent access
}

func main() {

	channel := make(chan User)

	go insideGoRoutines(channel)
	received := <-channel
	changeName(&received.username, "Yes, you")
	fmt.Println(received)
}
