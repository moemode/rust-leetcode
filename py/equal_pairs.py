from collections import defaultdict
from typing import List



def equalPairs(grid: List[List[int]]) -> int:
    sequences = defaultdict(set)
    for (row_idx, row) in enumerate(grid):
        sequences[tuple(row)].add(row_idx)
    matches = 0
    for col_idx in range(len(grid)):
        col = tuple(grid[i][col_idx] for i in range(len(grid)))
        matches += len(sequences[col])
    return matches

def equalPairsMatches(grid: List[List[int]]) -> int:
    sequences = defaultdict(set)
    for (row_idx, row) in enumerate(grid):
        sequences[tuple(row)].add(row_idx)
    matches = set()
    for col_idx in range(len(grid)):
        col = tuple(grid[i][col_idx] for i in range(len(grid)))
        for row_idx in sequences[col]:
            matches.add((row_idx, col_idx))
    return matches

grid = [[3,2,1],[1,7,6],[2,7,7]]
grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]

print(equalPairs(grid))

