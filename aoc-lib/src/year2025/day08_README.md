
---

# Advent of Code Day 08 – Playground: Solved using Kruskal + DSU

## Problem Summary

Given N junction boxes in 3D space, connect pairs with the shortest straight-line distances.

* **Part 1**:
  Consider the **1000 closest pairs** of junction boxes.
  After attempting those connections, compute the sizes of all resulting circuits and return the product of the three largest sizes.

* **Part 2**:
  Continue connecting junction boxes in order of increasing distance until **all boxes are in a single circuit**.
  Return the product of the **X coordinates** of the last pair that caused the final merge.

Distances are Euclidean; only relative ordering matters.

---

## Key Observations

* Squared distance preserves ordering and avoids `sqrt`.
* This is a classic **minimum-spanning-forest / Kruskal-style** problem.
* A **Disjoint Set Union (DSU / Union-Find)** structure efficiently tracks connected components.
* Part 1 does **not** require sorting all edges; only the K smallest matter.
* Part 2 **does** require a full ordered scan until the graph becomes connected.

---

## Core Data Structures

* **Edge**: `(weight, i, j)` where `i < j`
* **DSU / Union-Find**:

  * `find(x)` → component representative
  * `union(a, b)` → merge components if different
  * Tracks current component count

---

## Algorithm Overview

### Shared Setup

1. Parse all input points.
2. Build all undirected edges `(i, j)` with weight = squared distance.

---

### Part 1 – K Closest Connections (Efficient)

Goal: Use only the **1000 closest edges**.

Steps:

1. Build all edges.
2. Use **selection (`select_nth_unstable`)** to keep only the K smallest edges.
3. Sort just those K edges.
4. Run Kruskal unions over those edges.
5. Count final component sizes and compute the top-3 product.

Why:

* Avoids sorting ~500k edges when only 1000 are needed.
* DSU automatically ignores redundant connections.

---

### Part 2 – Full Connectivity

Goal: Find the **last edge** that connects the final two components.

Steps:

1. Sort all edges by distance.
2. Run Kruskal unions until only one component remains.
3. Track the last successful union.
4. Multiply the X coordinates of that edge’s endpoints.

Why:

* Full ordering is required to know which edge completes connectivity.
* Last successful union defines the answer.

---

## Pseudocode

```text
parse points[]

edges = []
for each i < j:
    edges.push( (w = dist2(points[i], points[j]), i, j) )

# Part 1
K = 1000
partition edges so first K are smallest by w
edges_k = first K edges
sort edges_k by w

uf = DSU(n)
for each (w, i, j) in edges_k:
    uf.union(i, j)

sizes = uf.component_sizes()
answer1 = product_of_top3(sizes)

# Part 2
sort edges by w

uf = DSU(n)
last = none
for each (w, i, j) in edges:
    if uf.union(i, j):
        last = (i, j)
        if uf.groups == 1:
            break

answer2 = points[last.i].x * points[last.j].x
```

---

## Why This Approach

* Avoids unnecessary work in Part 1.
* Reuses a single Kruskal/DSU core for both parts.
* Clear separation between **problem logic** and **implementation details**.
* Fast enough for AoC constraints while remaining readable.
* This is AoC grade code. For production code, I'd use spatial indexing or a different algorithmic strategy

---
