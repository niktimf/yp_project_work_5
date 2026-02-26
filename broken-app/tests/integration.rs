use broken_app::{algo, concurrency, leak_buffer, normalize, safe_box_value, sum_even};
use proptest::prelude::*;

// === sum_even ===

proptest! {
    /// Инвариант: результат sum_even — сумма только чётных элементов.
    /// Регрессия: off-by-one в unsafe get_unchecked вызывал UB.
    #[test]
    fn prop_sum_even(values in prop::collection::vec(-10_000i64..10_000, 0..200)) {
        let result = sum_even(&values);
        let expected: i64 = values.iter().filter(|v| *v % 2 == 0).sum();
        prop_assert_eq!(result, expected);
    }

    /// Инвариант: добавление нечётного числа не меняет результат.
    #[test]
    fn prop_sum_even_odd_invariant(
        values in prop::collection::vec(-10_000i64..10_000, 0..100),
        odd in (-10_000i64..10_000).prop_filter("must be odd", |v| v % 2 != 0)
    ) {
        let before = sum_even(&values);
        let mut extended = values.clone();
        extended.push(odd);
        prop_assert_eq!(sum_even(&extended), before);
    }
}

// === leak_buffer ===

proptest! {
    /// Инвариант: результат <= длины входа.
    /// Регрессия: буфер не освобождался (утечка памяти).
    #[test]
    fn prop_leak_buffer(input in prop::collection::vec(any::<u8>(), 0..300)) {
        let result = leak_buffer(&input);
        let expected = input.iter().filter(|b| **b != 0).count();
        prop_assert_eq!(result, expected);
        prop_assert!(result <= input.len());
    }
}

// === normalize ===

proptest! {
    /// Инвариант: результат не содержит пробельных символов и состоит из строчных букв.
    /// Регрессия: replace(' ', "") не убирала табуляции и другие пробельные символы.
    #[test]
    fn prop_normalize_no_whitespace(input in "[ -~\\t\\n\\r]{0,100}") {
        let result = normalize(&input);
        let lower = result.to_lowercase();
        prop_assert!(!result.chars().any(|c| c.is_whitespace()));
        prop_assert_eq!(result, lower);
    }

    /// Инвариант: нормализация идемпотентна.
    #[test]
    fn prop_normalize_idempotent(input in "\\PC{0,100}") {
        let once = normalize(&input);
        let twice = normalize(&once);
        prop_assert_eq!(once, twice);
    }
}

// === average_positive ===

proptest! {
    /// Инвариант: результат в диапазоне [min_positive, max_positive] или 0.0 при отсутствии положительных.
    /// Регрессия: делилось на все элементы, а не только положительные.
    #[test]
    fn prop_average_positive_bounds(values in prop::collection::vec(-1000i64..1000, 0..100)) {
        let result = broken_app::average_positive(&values);
        let positives: Vec<i64> = values.iter().copied().filter(|v| *v > 0).collect();
        if positives.is_empty() {
            prop_assert_eq!(result, 0.0);
        } else {
            let min = *positives.iter().min().unwrap() as f64;
            let max = *positives.iter().max().unwrap() as f64;
            prop_assert!(result >= min && result <= max,
                "average {} not in [{}, {}]", result, min, max);
        }
    }

    /// Инвариант: результат совпадает с эталонной реализацией.
    #[test]
    fn prop_average_positive_reference(values in prop::collection::vec(-100i64..100, 1..50)) {
        let result = broken_app::average_positive(&values);
        let positives: Vec<i64> = values.iter().copied().filter(|v| *v > 0).collect();
        let expected = if positives.is_empty() {
            0.0
        } else {
            positives.iter().sum::<i64>() as f64 / positives.len() as f64
        };
        prop_assert!((result - expected).abs() < f64::EPSILON);
    }
}

// === dedup ===

proptest! {
    /// Инвариант: результат содержит только уникальные элементы и отсортирован.
    #[test]
    fn prop_dedup_unique_sorted(values in prop::collection::vec(0u64..500, 0..200)) {
        let result = algo::slow_dedup(&values);
        // все элементы уникальны
        let mut uniq = result.clone();
        uniq.dedup();
        prop_assert_eq!(&result, &uniq);
        // отсортированы
        let mut sorted = result.clone();
        sorted.sort_unstable();
        prop_assert_eq!(&result, &sorted);
        // все оригинальные значения присутствуют
        for v in &values {
            prop_assert!(result.contains(v));
        }
    }
}

// === fib ===

proptest! {
    /// Инвариант: fib(n) = fib(n-1) + fib(n-2) для n >= 2.
    /// Диапазон ограничен, т.к. slow_fib ещё экспоненциальный (до оптимизации).
    #[test]
    fn prop_fib_recurrence(n in 2u64..20) {
        prop_assert_eq!(algo::slow_fib(n), algo::slow_fib(n - 1) + algo::slow_fib(n - 2));
    }
}

// === safe_box_value: регрессия use-after-free ===

#[test]
fn test_safe_box_value() {
    assert_eq!(safe_box_value(), 84);
}

// === concurrency: регрессия data race на static mut ===
// Один тест, т.к. все проверки делят глобальный COUNTER и не могут идти параллельно.

#[test]
fn test_concurrency_regression() {
    // Несколько потоков
    assert_eq!(concurrency::race_increment(1000, 4), 4000);
    // Один поток
    assert_eq!(concurrency::race_increment(5000, 1), 5000);
    // Много потоков
    assert_eq!(concurrency::race_increment(100, 10), 1000);
    // read_after_sleep возвращает актуальное значение
    concurrency::reset_counter();
    let _ = concurrency::race_increment(500, 2);
    assert_eq!(concurrency::read_after_sleep(), 1000);
}