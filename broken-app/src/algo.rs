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
