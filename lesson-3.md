
# Домашняя работа по лекции 03

## Установка MongoDB и вход в контейнер MongoDB

`$docker run --name lesson-3  -d mongo:latest`  
`$docker exec -it lesson-3 bash`

## Вход в MongoDB CLI и вывод списка баз данных

#mongo

## Создать бд

`>use lesson-3`

*Вывод:*  
![Создание БД](/images/lesson-3/create-db.png)

## Импорт тестовых данных

`>mongoimport --db lesson-3 --collection titanic --type csv --headerline --ignoreBlanks --file /data/titanic.csv`

*Вывод:*
![Импорт данных](/images/lesson-3/import.png)

## Запросы к БД

`>db.titanic.find({})`

*Вывод:*
![Получить все записи из коллекции](/images/lesson-3/find-all.png)

`>db.titanic.find({Survived: 0})`

*Вывод:*
![Получить всех не выживших пассажиров](/images/lesson-3/dead-passanges.png)

`>d.titanic.find({Survived: 1, Sex: "female"})`

*Вывод:*
![Получить всех живых женщин](/images/lesson-3/find-alive-women.png)

`>d.titanic.find({ $or: [{Sex: "male"}, {Pclass: 2}]})`

*Вывод:*
![Получить всех пассажиров, которые являются мужчинами или находятся во 2 классе](/images/lesson-3/find-men-or-p2.png)

`>d.titanic.update({Sex:"male"}, {$set: {Pclass: 4}}, { $multi: true})`

*Вывод:*
![Переместить всех пассажиров мужского пола в 4 класс](/images/lesson-3/update.png)
