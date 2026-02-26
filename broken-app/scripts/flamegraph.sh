#!/usr/bin/env bash
set -euo pipefail

cargo install flamegraph 2>/dev/null || true

# === Flamegraph ДО оптимизации ===
cat > src/algo.rs << 'SLOW'
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            out.push(*v);
            out.sort_unstable();
        }
    }
    out
}

pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),
    }
}
SLOW

cargo flamegraph --bench baseline -o artifacts_before/flamegraph.svg 2>&1
echo "Flamegraph (before) saved to artifacts_before/flamegraph.svg"

# === Возвращаем оптимизированную версию ===
cat > src/algo.rs << 'FAST'
use std::collections::HashSet;

/// ОПТИМИЗАЦИЯ: O(n) через HashSet вместо O(n² log n) с линейным поиском + sort на каждой вставке.
/// Убраны лишние аллокации: pre-alloc capacity для Vec и HashSet.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());
    for &v in values {
        if seen.insert(v) {
            out.push(v);
        }
    }
    out.sort_unstable();
    out
}

/// ОПТИМИЗАЦИЯ: O(n) итеративно через fold вместо O(2^n) рекурсивно.
/// Без аллокаций — только кортеж на стеке.
pub fn slow_fib(n: u64) -> u64 {
    (0..n).fold((0u64, 1u64), |(a, b), _| (b, a + b)).0
}
FAST

# === Flamegraph ПОСЛЕ оптимизации ===
cargo flamegraph --bench baseline -o artifacts_after/flamegraph.svg 2>&1
echo "Flamegraph (after) saved to artifacts_after/flamegraph.svg"