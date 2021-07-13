// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package bob should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package bob

import (
	"log"
	"regexp"
	"strings"
)

const lettersLower = "abcdefghijklmnopqrstuvwxyz"

func allUpper(s string) bool {
	for _, char := range lettersLower {
		if strings.Contains(s, string(char)) {
			return false
		}
	}
	return true
}

func isQuestion(s string) bool {
	// Check if s is not empty and last char of string is a question mark.
	s0 := strings.ReplaceAll(s, " ", "")
	if s0 == "" {
		return false
	}
	return string(s0[len(s0)-1:]) == "?"
}

func filterLetters(sIn string) string {
	// Use a regular expression to filter out all letters.
	reg, err := regexp.Compile("[^a-zA-Z]+")
	if err != nil {
		log.Fatal(err)
	}
	sOut := reg.ReplaceAllString(sIn, "")
	return sOut
}

func replaceSpaces(s string) string {
	// Build a replaces that takes pairs of chars (toReplace, withWhat) and then
	// apply on string and assigne it to a new vaiable.
	replacer := strings.NewReplacer(" ", "", "\t", "", "\n", "", "\r", "")
	output := replacer.Replace(s)
	return output
}

// Hey should have a comment documenting it.
func Hey(remark string) string {

	r := replaceSpaces(remark)
	s := filterLetters(r)
	isQuestion := isQuestion(r) // "?" anywhere
	isScreaming := allUpper(s)

	// "?" && allUpper?
	if isQuestion && isScreaming && s != "" {
		return "Calm down, I know what I'm doing!"

	}

	// "?"
	if isQuestion {
		return "Sure."
	}

	// All upper
	if isScreaming && s != "" {
		return "Whoa, chill out!"

	}

	// empty string
	if r == "" {
		return "Fine. Be that way!"
	}

	// else
	return "Whatever."
}
