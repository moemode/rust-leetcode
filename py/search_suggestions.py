from typing import Iterator, List, Optional, Set


class Node:
    def __init__(self):
        self.terminal = False
        self.next = [None] * 26  # 26 entries for lowercase English letters

    def contained_suffixes(self) -> Iterator[str]:
        children = [(self, "")]
        while children:
            (node, suffix) = children.pop()
            if node.terminal:
                yield suffix
            # iterate over reverse of self.next
            for i in range(25, -1, -1):
                if node.next[i]:
                    children.append((node.next[i], suffix + chr(i + ord("a"))))


class Trie:

    def __init__(self):
        self.root = Node()

    def insert(self, word: str) -> None:
        n = self.lookup(word, True)
        n.terminal = True

    def search(self, word: str) -> bool:
        n = self.lookup(word, False)
        return n.terminal if n else False

    def lookup(self, prefix: str, create_mode: bool) -> Optional[Node]:
        curr = self.root
        for c in prefix:
            idx = ord(c) - ord("a")
            if curr.next[idx] is None:
                if create_mode:
                    curr.next[idx] = Node()
                else:
                    return None
            curr = curr.next[idx]
        return curr

    def startsWith(self, prefix: str) -> bool:
        return self.lookup(prefix, False) is not None


def suggestedProducts(self, products: List[str], searchWord: str) -> List[List[str]]:
    product_trie = Trie()
    for p in products:
        product_trie.insert(p)
    prefix = ""
    curr = product_trie.root
    suggestions = []
    for c in searchWord:
        prefix += c
        idx = ord(c) - ord("a")
        if curr.next[idx] is None:
            break
        curr = curr.next[idx]
        # get the first three elements
        suggestion = []
        for suffix in curr.contained_suffixes():
            suggestion.append(prefix + suffix)
            if len(suggestion) == 3:
                break
        suggestions.append(suggestion)
    if len(suggestions) < len(searchWord):
        suggestions.extend([] * (len(searchWord) - len(suggestions)))
    return suggestions


def test_case_mouse():
    products = ["mobile", "mouse", "moneypot", "monitor", "mousepad"]
    searchWord = "mouse"
    result = suggestedProducts(None, products, searchWord)
    print(
        "Input: products =", products, ", searchWord =", searchWord, ", Output:", result
    )


test_case_mouse()
