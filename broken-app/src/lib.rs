pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// BUG FIX: убран unsafe с off-by-one (`0..=len` -> итератор).
pub fn sum_even(values: &[i64]) -> i64 {
    values.into_iter().filter(|v| *v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов.
/// BUG FIX: убрана утечка памяти — теперь реализация без unsafe.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|b| **b != 0).count()
}

/// Нормализация строки: убираем все виды пробельных символов и приводим к нижнему регистру.
/// BUG FIX: `replace(' ', "")` заменён на `split_whitespace()`,
/// чтобы обрабатывались табуляции и другие пробельные символы.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Корректное усреднение только положительных чисел.
/// BUG FIX: фильтруем только положительные значения, делим на их количество.
pub fn average_positive(values: &[i64]) -> f64 {
    let positives: Vec<i64> = values.iter().copied().filter(|v| *v > 0).collect();
    if positives.is_empty() {
        return 0.0;
    }
    let sum: i64 = positives.iter().sum();
    sum as f64 / positives.len() as f64
}

/// BUG FIX: убран use-after-free. Теперь значение читается до освобождения.
pub fn safe_box_value() -> i32 {
    let b = Box::new(42_i32);
    let val = *b;
    // Box автоматически освобождается при выходе из области видимости
    val + val
}
