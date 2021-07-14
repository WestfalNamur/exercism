// This is a "stub" file. It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package acronym should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package acronym

import (
	"fmt"
	"log"
	"regexp"
	"strings"
)

func filterLetters(s string) string {
	// Use a regular expression to filter out all letters.
	reg, err := regexp.Compile("[^a-zA-Z ']+")
	if err != nil {
		log.Fatal(err)
	}
	return reg.ReplaceAllString(s, " ")
}

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) string {
	// Convert a phrase to its acronym.
	// Techies love their TLA (Three Letter Acronyms)!

	// Filter string to only contain alphabetic letters
	sAbc := filterLetters(s)

	// Split into list of words
	words := strings.Fields(sAbc)

	// Create a slice with the first letters of each word
	fmt.Println()
	abbr := make([]string, len(words))
	for i, w := range words {
		abbr[i] = string(w[0])
	}

	// Join to remove the " "
	abbrJoined := strings.Join(abbr, "")

	// to upper
	abbrUp := strings.ToUpper(abbrJoined)
	fmt.Println(words, "Becomes: ", abbrUp)

	return abbrUp
}
