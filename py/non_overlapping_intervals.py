from typing import List


def eraseOverlapIntervals(intervals: List[List[int]]) -> int:
    erase = 0
    if not intervals:
        return erase
    intervals.sort(key=lambda int: int[1], reverse=True)
    previous = intervals.pop()
    while intervals:
        current = intervals.pop()
        if overlap(previous, current):
            erase += 1
        else:
            previous = current
    return erase

def overlap(i1: List[int], i2: List[int]) -> bool:
    return i2[0] < i1[1]

intervals = [[1,2],[2,3],[3,4],[1,3]]
intervals = [[1,2],[1,2],[1,2]]
intervals = [[1,2],[2,3]]
print(eraseOverlapIntervals(intervals))