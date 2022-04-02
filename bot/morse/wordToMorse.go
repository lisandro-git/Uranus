package morse

import (
	"strings"
)

func Encode(word string) []byte {
	word = strings.TrimSuffix(word, "\n")
	
	var result []byte
	var i int
	for i = 0; i < len(word); i++ {
		if v, found := AllSigns[string(word[i])]; found {
			result = append(result, v + " "...)
		}
	}
	return result;
}
