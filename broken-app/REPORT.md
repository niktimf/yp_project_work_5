# Отчёт: broken-app — исправление, валидация, оптимизация

## 1. Найденные и исправленные баги (6 шт.)

| # | Тип | Где | Описание | Исправление |
|---|---|---|---|---|
| 1 | UB: off-by-one | `lib.rs:sum_even` | `0..=values.len()` в unsafe `get_unchecked` — доступ за пределы среза | Убран unsafe, заменён на итератор `.filter().sum()` |
| 2 | Утечка памяти | `lib.rs:leak_buffer` | `Box::into_raw` без `Box::from_raw` — буфер не освобождался | Переписан без unsafe через `.iter().filter().count()` |
| 3 | UB: use-after-free | `lib.rs:use_after_free` | Чтение `*raw` после `drop(Box::from_raw(raw))` | Заменён на safe `safe_box_value()` — чтение до освобождения |
| 4 | Логическая ошибка | `lib.rs:average_positive` | Среднее по всем элементам, а не только положительным | Фильтрация `v > 0`, деление на количество положительных |
| 5 | Логическая ошибка | `lib.rs:normalize` | `replace(' ', "")` не убирает табы и другие пробельные символы | Заменён на `split_whitespace().collect()` |
| 6 | Data race | `concurrency.rs` | `static mut COUNTER: u64` без синхронизации в многопоточном коде | Заменён на `static COUNTER: AtomicU64` с `Ordering::SeqCst` |

## 2. Профилирование (callgrind, до оптимизации)

```
221,423 (42.39%)  broken_app::algo::slow_fib    ← главное узкое место
 46,603 ( 8.92%)  libc: __vfscanf_internal
 46,441 ( 8.89%)  ld: do_lookup_x
```

`slow_fib` занимает **42%** всех инструкций программы из-за экспоненциальной рекурсии O(2^n).

## 3. Оптимизации (2 шт.)

### 3.1. Алгоритмическая: `slow_fib` O(2^n) → O(n)

Рекурсивная реализация заменена на итеративную через `fold`:
```rust
pub fn slow_fib(n: u64) -> u64 {
    (0..n).fold((0u64, 1u64), |(a, b), _| (b, a + b)).0
}
```

### 3.2. Алгоритмическая + микро: `slow_dedup` O(n² log n) → O(n)

- Линейный поиск дубликатов заменён на `HashSet`
- Убрана `sort_unstable()` на каждой вставке (одна сортировка в конце)
- Добавлен `with_capacity` для Vec и HashSet (убраны лишние реаллокации)

## 4. Бенчмарки до/после

| Функция | До | После | Ускорение |
|---|---|---|---|
| `sum_even` (50K) | 13.4 µs | 14.7 µs | ~то же (не оптимизировали) |
| `slow_fib(32)` | 5.84 ms | 3.98 ns | **~1 500 000x** |
| `slow_dedup` (10K) | 12.09 ms | 119 µs | **~100x** |

## 5. Валидация

| Инструмент | Результат |
|---|---|
| `cargo test` | 11/11 passed |
| `valgrind --leak-check=full` | definitely lost: 0 bytes, ERROR SUMMARY: 0 errors |
| `cargo +nightly miri test` | UB не обнаружено |
| ASan (`-Zsanitizer=address`) | 2/2 passed, ошибок нет |
| TSan (`-Zsanitizer=thread`) | 2/2 passed, ошибок нет |

## 6. Регрессионные тесты

Добавлены property-based тесты через `proptest`:
- `prop_sum_even` / `prop_sum_even_odd_invariant` — инварианты суммирования
- `prop_leak_buffer` — результат <= длины, совпадает с эталоном
- `prop_normalize_no_whitespace` / `prop_normalize_idempotent` — нет пробелов, идемпотентность
- `prop_average_positive_bounds` / `prop_average_positive_reference` — диапазон и совпадение с эталоном
- `prop_dedup_unique_sorted` — уникальность, сортировка, полнота
- `prop_fib_recurrence` — fib(n) = fib(n-1) + fib(n-2)
- `test_safe_box_value` — регрессия use-after-free
- `test_concurrency_regression` — регрессия data race (4 сценария)

## 7. Артефакты

- `artifacts_before/` — бенчмарки и профиль до оптимизации
- `artifacts_after/` — бенчмарки и валидация после оптимизации
- `scripts/sanitizers.sh` — скрипт запуска ASan/TSan
- `scripts/profile.sh` — шаблон профилирования
