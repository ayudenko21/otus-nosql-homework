
# Домашняя работа по лекции 03

## Установка MongoDB и вход в контейнер MongoDB

$docker run --name lesson-3  -d mongo:latest
$docker exec -it lesson-3 bash

## Вход в MongoDB CLI и вывод списка баз данных

#mongo

## Создать бд

>use lesson-3

**Вывод:
![Создание БД](/images/lesson-3/create-db.png)

## Импортировать тестовые данные


