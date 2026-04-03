# Лабораторная работа №11

## Студент
**ФИО:** Глазков Александр Валерьевич
**Группа:** 221331
**Вариант:** 4

## Выполненные задания

- **Средн.4** — Собрать образы и сравнить размеры (`image-sizes.txt`)
- **Средн.6** — Настроить сеть между контейнерами (`lab11-network` в docker-compose.yml)
- **Средн.8** — Добавить healthcheck для каждого сервиса (docker-compose.yml)
- **Повыш.4** — docker buildx для кросс-платформенной сборки arm64/amd64 (`buildx.sh`)
- **Повыш.1** — Go в scratch-образе со статической компиляцией (CGO_ENABLED=0)

## Сервисы

| Сервис | Язык | Порт | Эндпоинты |
|---|---|---|---|
| go-service | Go + Gin | 8080 | /ping, /health |
| python-service | Python + FastAPI | 8000 | /ping, /health, /info |
| rust-service | Rust + Axum | 8081 | /ping, /health |

## Запуск
```bash
# Собрать и запустить все сервисы
docker compose up --build

# Проверить статус healthcheck
docker compose ps

# Кросс-платформенная сборка (повыш.4)
chmod +x buildx.sh
./buildx.sh
```

## Сеть

Все сервисы находятся в общей сети `lab11-network`.
Python видит Go по имени контейнера: `http://go-service:8080`.

## Размеры образов

| Образ | База | Размер |
|---|---|---|
| go-service | scratch | ~12 MB |
| python-service | python:3.12-slim | ~180 MB |
| rust-service | debian:bookworm-slim | ~90 MB |

Go минимален за счёт статической компиляции и scratch-образа.