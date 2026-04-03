# Этап сборки
FROM golang:1.22 AS builder

WORKDIR /app

# Копируем зависимости отдельным слоем для кэширования
COPY go.mod go.sum ./
RUN go mod download

COPY . .

# CGO_ENABLED=0 — статическая компиляция (нужна для scratch)
# GOOS=linux — явно указываем целевую ОС
RUN CGO_ENABLED=0 GOOS=linux go build -o server .

# Финальный образ — scratch (повышенное задание 1)
# Размер ~12MB против ~800MB у golang:1.22
FROM scratch
COPY --from=builder /app/server /server
EXPOSE 8080
CMD ["/server"]