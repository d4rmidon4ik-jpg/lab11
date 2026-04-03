# PROMPT_LOG.md

## Средн.4: Собрать образы и сравнить размеры

### Промпт 1
**Инструмент:** Claude
**Промпт:** "Напиши Dockerfile для Go с многоэтапной сборкой и scratch-образом,
для Python на slim, для Rust на bookworm-slim."
**Результат:** Три Dockerfile получил. Go scratch не запускался — healthcheck
не мог использовать wget, которого нет в scratch.

### Промпт 2
**Инструмент:** Claude
**Промпт:** "В scratch нет wget — healthcheck падает. Как сделать healthcheck
для Go в scratch без внешних утилит?"
**Результат:** Добавил флаг -health-check в main.go — бинарник сам проверяет
свой /health эндпоинт через http.Get и завершается с кодом 0 или 1.

### Итого
- Количество промптов: 2
- Что исправлял вручную: добавил -health-check флаг в main.go, порт в Cargo.toml
- Время: ~30 мин

---

## Средн.6: Настроить сеть между контейнерами

### Промпт 1
**Инструмент:** Claude
**Промпт:** "Добавь в docker-compose bridge-сеть lab11-network.
Python должен видеть Go по имени контейнера go-service."
**Результат:** Получил networks секцию и GO_SERVICE_URL=http://go-service:8080.
depends_on с condition: service_healthy пришлось добавить вручную.

### Итого
- Количество промптов: 1
- Что исправлял вручную: depends_on condition
- Время: ~10 мин

---

## Средн.8: Healthcheck для каждого сервиса

### Промпт 1
**Инструмент:** Claude
**Промпт:** "Добавь healthcheck в docker-compose для всех трёх сервисов,
interval 10s, timeout 5s, retries 3."
**Результат:** Healthcheck для Python через wget упал — wget не установлен
в python:3.12-slim.

### Промпт 2
**Инструмент:** Claude
**Промпт:** "Python healthcheck падает — нет wget в slim. Перепиши через urllib."
**Результат:** Исправил на python -c "import urllib.request; urllib.request.urlopen(...)".

### Итого
- Количество промптов: 2
- Что исправлял вручную: healthcheck команду для Python и Go (scratch)
- Время: ~15 мин

---

## Повыш.4: docker buildx кросс-платформенная сборка

### Промпт 1
**Инструмент:** Claude
**Промпт:** "Напиши bash-скрипт для docker buildx, который собирает все три
образа для linux/amd64 и linux/arm64."
**Результат:** Получил buildx.sh. Пришлось вручную добавить создание builder
если он ещё не существует.

### Итого
- Количество промптов: 1
- Что исправлял вручную: проверку существования builder через buildx inspect
- Время: ~10 мин

---

## Повыш.1: Go в scratch со статической компиляцией

### Промпт 1
**Инструмент:** Claude
**Промпт:** "Собери Go-приложение с CGO_ENABLED=0 и запусти в scratch-образе."
**Результат:** Dockerfile написан правильно с первого раза.
Проблема возникла только с healthcheck — решена в средн.8.

### Итого
- Количество промптов: 1
- Что исправлял вручную: ничего в Dockerfile, но потребовалось изменить main.go
- Время: ~10 мин