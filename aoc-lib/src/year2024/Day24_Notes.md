# Advent of Code 2024 - Day 24: Crossed Wires

Day 24 literally took me 24 hours to solve. No pun intended. 
Getting old and slow reasoning - maybe...I had to force-recall XOR logic lessons from my college days

Regardless, I am taking time to document what worked here so that Future Me can thank me :-)

## The Problem

We have a logic circuit made of AND, OR, and XOR gates. The circuit takes two binary numbers as input (X and Y via wires x00, x01, etc.) and computes their sum (Z via wires z00, z01, etc.).

The circuit SHOULD be a ripple-carry adder, but some output wires got swapped during assembly. We need to find which wires were swapped.

- **Part 1**: Simulate the circuit and determine what number appears on the Z wires.
- **Part 2**: Identify exactly which 8 wires (4 pairs) were swapped.
---

## Solution Approach

### Part 1: Simple Gate-Level Simulation

- Start with initial wire values
- Repeatedly evaluate gates whose inputs are ready
- Continue until circuit reaches steady state
- Read out the final Z value

### Part 2: Structural Validation of Ripple-Carry Adder

- Instead of trying to fix the circuit, we detect violations
- A correct ripple-carry adder has very specific structure
- Any gate that doesn't follow the rules = swapped wire
- We check 4 key rules and collect violators

---

## Ripple-Carry Adder Primer

Adding two N-bit numbers X + Y = Z requires:

For each bit position i:
```couchbasequery  
half_sum[i] = x[i] XOR y[i]                    // Add without carry
carry_gen[i] = x[i] AND y[i]                   // Generate carry?
sum[i] = half_sum[i] XOR carry_in[i]           // Final sum bit -> z[i]
carry_prop[i] = half_sum[i] AND carry_in[i]    // Propagate carry?
carry_out[i] = carry_gen[i] OR carry_prop[i]   // Next carry
```

**Special Cases:**
- **Bit 0**: No carry in, so `x0 XOR y0` goes directly to `z0`
- **Top bit**: It's just the final carry out

---

## Visual Structure - Each Bit Position

```azure
┌─────────────────────────────────────────────────────────────────────────┐
│ Each Bit Position i (except bit 0 and top bit):                        │
│                                                                         │
│    x[i] ──┐                                                             │
│           ├─ XOR ─→ half_sum[i] ──┐                                    │
│    y[i] ──┘                        │                                    │
│                                    ├─ XOR ──→ z[i]  (sum output)       │
│                    carry_in[i] ────┘                                    │
│                         │                                               │
│                         └─ AND ──→ carry_prop[i] ──┐                   │
│                            ↑                        │                   │
│                       half_sum[i]                   ├─ OR ──→ carry_out│
│                                                     │                   │
│    x[i] ──┐                                         │                   │
│           ├─ AND ──→ carry_gen[i] ─────────────────┘                   │
│    y[i] ──┘                                                             │
│                                                                         │
│ Key observations:                                                       │
│  • Two XOR gates: (x,y)→half_sum and (half_sum,carry)→z                │
│  • Two AND gates: (x,y)→carry_gen and (half_sum,carry)→carry_prop      │
│  • One OR gate: (carry_gen,carry_prop)→carry_out                       │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Part 2: The Four Structural Rules

A valid ripple-carry adder must satisfy these rules:

### Rule 1: Z-outputs come from XOR (except final carry bit)

**Why?** Each `z[i] = half_sum XOR carry`, which is an XOR operation.

**Exception:** The highest z-bit is just the final carry (no XOR).

### Rule 2: XOR gates with intermediate wires must output to Z

**Why?** There are only two types of XOR gates:
- Type A: `x[i] XOR y[i] → half_sum[i]` (inputs are x,y)
- Type B: `half_sum[i] XOR carry[i] → z[i]` (output is z)

If inputs aren't x/y, this must be Type B, so output must be z.

### Rule 3: XOR(x,y) must feed another XOR (except bit 0)

**Why?** `half_sum = x XOR y` must be used in `sum = half_sum XOR carry`.

**Exception:** Bit 0 has no carry-in, so `x0 XOR y0` goes directly to `z0`.

### Rule 4: AND gates must feed OR gates (except bit 0)

**Why?** `carry[i+1] = carry_gen OR carry_prop`, where both terms come from ANDs:
- `carry_gen = x[i] AND y[i]`
- `carry_prop = half_sum[i] AND carry[i]`

Both of these AND results must feed into the OR that produces `carry[i+1]`.

**Exception:** Bit 0 is special: `x0 AND y0` becomes `carry[1]` directly (no OR needed for first carry).

---

## Implementation Details

### Data Structures

**Op Enum**: Represents gate operation types (And, Or, Xor)

**Gate Struct**: Holds gate information
- `a`: First input wire name
- `b`: Second input wire name
- `out`: Output wire name
- `op`: Operation type

### Input Format

```
x00: 1
y00: 0

x00 AND y00 -> z00
```

First section: Initial wire values (wire_name: bit_value)
Second section: Gate definitions (input1 OP input2 -> output)

### Part 1 Algorithm: Fixed-Point Iteration

1. Clone initial wire values
2. Loop until no values change:
   - For each gate, if both inputs are ready, compute output
   - Update output wire if value changed
3. Extract z-wires and combine into final number

**Z-value extraction:**
- Collect all wires starting with 'z'
- Sort by bit position (z00, z01, z02, ...)
- Combine into 64-bit integer where z00 is LSB

### Part 2 Algorithm: Rule Validation

1. Find highest z-bit (the final carry)
2. Check each gate against the 4 structural rules
3. Collect all gates that violate rules
4. Return sorted, comma-separated list of swapped wires

**Key Insight:** We don't need to reconstruct the entire adder or figure out which pairs to swap. We just identify all wires that violate structural properties - those are the swapped ones!

---

## Why This Works

The elegance of this solution is that it leverages the **deterministic structure** of a ripple-carry adder. Unlike other circuits that could be implemented many ways, an N-bit ripple-carry adder has only one valid structure (per bit position). Any deviation from this structure must be a wiring error.

By checking local properties (what type of gate produces this output, what does this gate feed into), we can identify all incorrect wires without needing to:
- Simulate with test inputs
- Try different swap combinations
- Reconstruct the carry chain

The structural rules are both **necessary and sufficient** for correctness.