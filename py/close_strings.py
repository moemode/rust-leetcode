def closeStrings(self, word1: str, word2: str) -> bool:
    return charCounts(word1) == charCounts(word2) and set(word1) == set(word2)
    
"""
Given word return a list l with l[i] = c, if there
are c distinct characters in word which appear i + 1 times.
"""
def charCounts(word: str) -> list[int]:
    counts = [0] * len(word)
    for c in set(word):
        counts[word.count(c)-1] += 1
    return counts

def closeStringsOpt(self, word1: str, word2: str) -> bool:
    if len(word1) != len(word2):
        return False
    chars1 = set(word1)
    chars2 = set(word2)
    if chars1 != chars2:
        return False
    counts = [0] * len(word1)
    for c in chars1:
        counts[word1.count(c)-1] += 1
    for c in chars2:
        counts[word2.count(c)-1] -= 1
    return all(count == 0 for count in counts)


print(charCounts("cabbbba"))