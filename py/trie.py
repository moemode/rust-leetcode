# https://leetcode.com/problems/implement-trie-prefix-tree
from typing import Optional


class Node:
    def __init__(self):
        self.terminal = False
        self.next = [None] * 26  # 26 entries for lowercase English letters



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
            idx = ord(c) - ord('a')
            if curr.next[idx] is None:
                if create_mode:
                    curr.next[idx] = Node()
                else:
                    return None
            curr = curr.next[idx]
        return curr
        

    def startsWith(self, prefix: str) -> bool:
        return self.lookup(prefix, False) is not None


# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.search(word)
# param_3 = obj.startsWith(prefix)