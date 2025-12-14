# Advent of Code 2024 - Day 25: Code Chronicle

## The Problem

The Chief Historian's office door is locked with a fancy five-pin tumbler lock, and North Pole security has lost track of which keys go with which locks. They can only provide schematics of all locks and keys.

The schematics show locks (filled at top, empty at bottom) and keys (empty at top, filled at bottom) as 7-row grids. Each represents a set of 5 columns with different heights.

- **Task:**: Count how many unique lock/key pairs fit together without any column overlap.

---

## Solution Approach

### Part 1: Count Compatible Lock/Key Pairs

1. Parse the input to separate locks from keys
2. Convert each schematic to a list of column heights
3. Test every lock/key combination
4. Count pairs where lock_height + key_height ≤ 5 for all columns

This is a straightforward brute-force approach that works well because:
- The number of locks and keys is manageable
- Each compatibility check is O(5) = O(1)
- Total complexity: O(locks × keys) which is fast enough

---

## Understanding Locks and Keys

### Lock Representation

Locks have pins extending **downward** from the top:
```
#####  <- Top row always filled
.####
.####
.####
.#.#.
.#...
.....  <- Bottom row always empty
```

This converts to pin heights: `[0,5,3,4,3]`
- Column 0: No pins below top row = height 0
- Column 1: 5 rows of # below top = height 5
- Column 2: 3 rows of # below top = height 3
- Column 3: 4 rows of # below top = height 4
- Column 4: 3 rows of # below top = height 3

### Key Representation

Keys have teeth extending **upward** from the bottom:
```
.....  <- Top row always empty
#....
#....
#...#
#.#.#
#.###
#####  <- Bottom row always filled
```

This converts to tooth heights: `[5,0,2,1,3]`
- Column 0: 5 rows of # above bottom = height 5
- Column 1: No # above bottom row = height 0
- Column 2: 2 rows of # above bottom = height 2
- Column 3: 1 row of # above bottom = height 1
- Column 4: 3 rows of # above bottom = height 3

### Compatibility Check

A lock and key fit if the sum of heights in each column doesn't exceed 5:
- Available space per column: 5 rows (between top and bottom rows)
- Lock pins occupy some of this space from top
- Key teeth occupy some from bottom
- They fit if: `lock_height[i] + key_height[i] <= 5` for all columns

**Example:**
- Lock: `[0,5,3,4,3]`
- Key: `[5,0,2,1,3]`
- Sums: `[5,5,5,5,6]`
- Result: **Don't fit** (column 4 sums to 6, exceeds 5)

---

## Implementation Details

### Data Structures

**Schematic Type**: Each schematic is represented as `Vec<usize>` with 5 elements (one height per column)

**Parsing Strategy**:
1. Split input by double newlines to get individual schematics
2. For each schematic:
   - Check first row: all `#` means lock, all `.` means key
   - Count `#` symbols in each column (excluding first/last rows)
   - Store as height vector

### Algorithm

```
For each lock L:
    For each key K:
        compatible = true
        For each column i in 0..5:
            if L[i] + K[i] > 5:
                compatible = false
                break
        if compatible:
            count += 1
```

**Complexity**:
- Parsing: O(n × rows × cols) where n is number of schematics
- Matching: O(locks × keys × 5) = O(locks × keys)
- Total: O(n + locks × keys)

---

## Edge Cases
1. **Empty input**: Should return 0
2. **No locks or no keys**: Should return 0
3. **All incompatible**: Should return 0
4. **All compatible**: Should return locks × keys

---

## Why This Works

The problem is essentially a constraint satisfaction check:
- Each lock defines space requirements
- Each key defines space requirements
- They're compatible if total requirements don't exceed available space

The brute-force approach is optimal here because:
1. Every lock must be checked against every key
2. No way to prune the search space without checking
3. The number of checks is manageable

There's no need for optimization techniques like:
- Hashing or indexing (checking is already O(1))
- Early termination (we need the complete count)
- Parallel processing (problem is small enough)

---
