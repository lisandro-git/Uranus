package morse

import (
	"strings"
)

func Decode(word string) string {
	line := strings.TrimSuffix(word, "\n")
	morse_code := strings.Split(line, " ")
	
	var result string
	var i int
	var space bool // edode : used to add a space if there is two spaces in between words
	
	for i = 0; i < len(morse_code); i++ {
		for key, value := range All_signs {
			if morse_code[i] == "" {
				if space {
					result += " "
					break;
				}
				space = true
				continue;
			}
			if (value == morse_code[i]) {
				result += key
				space = false
				break;
			}
		}
	}
	return result;
}
