from typing import List, Optional
from collections import deque

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def rightSideView(root: Optional[TreeNode]) -> List[int]:
    level = deque()
    rightSide = []
    if root:
        level.append(root)
    while level:
        rightSide.append(level[-1].val)
        buildNextLevel(level)
    return rightSide

            
def buildNextLevel(level: deque):
    level_len = len(level)
    for _ in range(level_len):
        n = level.popleft()
        if n.left:
            level.append(n.left)
        if n.right:
            level.append(n.right)

def test_case2():
    # Building tree from BFS array [1,2,3,4,null,null,null,5]
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)
    root.left.left = TreeNode(4)
    root.left.left.left = TreeNode(5)

    result = rightSideView(root)
    print("Input: [1,2,3,4,null,null,null,5], Output:", result)

test_case2()