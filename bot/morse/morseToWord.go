package morse

import (
	"strings"
)

func Decode(word string) []byte {
	line := strings.TrimSuffix(word, "\n")
	morseCode := strings.Split(line, " ")
	
	var result string
	var i int
	var space bool // edode : used to add a space if there is two spaces in between words
	
	for i = 0; i < len(morseCode); i++ {
		for key, value := range AllSigns {
			if morseCode[i] == "" {
				if space {
					result += " "
					break;
				}
				space = true
				continue;
			}
			if (value == morseCode[i]) {
				result += key
				space = false
				break;
			}
		}
	}
	return []byte(result)
}
