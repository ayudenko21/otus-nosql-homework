# Отчет по домашней работе №6

## Запуск инстансов mongodb
docker compose up -d

(для создания mongodb инстансов используются конфиги из директории ./configs)

**Результат**   
![Запуск контейнеров](/images/image1.png)

## Создание реплицированных кластеров нод данных
docker compose exec replica-1 mongo --host replica-1 /home/mongo/scripts/initReplicaSet1.js
docker compose exec replica-4 mongo --host replica-4 /home/mongo/scripts/initReplicaSet2.js
docker compose exec replica-7 mongo --host replica-7 /home/mongo/scripts/initReplicaSet3.js

## Создание кластера серверов конфигураций

docker compose exec config-server-1 mongo --host config-server-1 /home/mongo/scripts/configServerSet.js

## Добавление шардов в mongos

** Перед созданием пользователей надо добавить в mongos шарды, иначе к нему не получается подконнектиться
docker exec -it mongos-1 mongo --host mongos-1 /home/mongo/scripts/addShards.js

** Результат **   
![Статус mongos](/images/image2.png)

## Создание пользователей для mongos

docker exec -it mongos-1 mongo --host mongos-1 /home/mongo/scripts/createAdminRoleAndUser.js
docker restart mongos-1
docker exec -it mongos-1 mongo --host mongos-1 -u adminUser -p 111 --authenticationDatabase "admin" /home/mongo/scripts/createReaderAndWriterRolesAndUsers.js
docker restart mongos-1

**Результат**   
![Пользователи и роли](/images/image3.png)

## Включение шардирования

docker exec -it mongos-1 mongo --host mongos-1 -u adminUser -p 111 --authenticationDatabase "admin" /home/mongo/scripts/enableSharding.js

## Заполнение данными

docker exec -it mongos-1 mongo --host mongos-1 -u writerUser -p 111 --authenticationDatabase "restaurant" /home/mongo/scripts/insertData.js

## Создание ключа шардирования

docker exec -it mongos-1 mongo --host mongos-1 -u adminUser -p 111 --authenticationDatabase "admin" /home/mongo/scripts/createIndex.js

## Шардирование коллекции

docker exec -it mongos-1 mongo --host mongos-1 -u adminUser -p 111 --authenticationDatabase "admin" /home/mongo/scripts/shardCollection.js

**Результат**   
![Шардирование](/images/image4.png)
![Шардирование](/images/image5.png)

docker exec -it mongos-1 mongo --host mongos-1 -u readerUser -p 111 --authenticationDatabase "restaurant"
use restaurant
db.checks.find().count()

**Результат**   
![Результаты](/images/image6.png)
Пока идет ребалансинг шардов метод count выводит количество записей не соответствующее действительности
![Результаты](/images/image7.png)
Но после окончания ребалансинга количество записей стало корректным

**Уроним по одному инстансу из каждого реплика сета и один конфиг сервер**   
![Уронили инстансы](/images/image8.png)   
В результате праймари нодами стали replica-2, replica-5, replica-8, config-server-2   
Данные по-прежнему доступны
![Данные](/images/image9.png)

**Уроним вторую ноду первого реплика сета**   
![Уронили вторую ноду](/images/images10.png)   
Данные не доступны   
![Данные](/images/image11.png)

**Поднимем все ноды обратно**   
![Подняли ноды](/images/image12.png)   
Ноды replica-1, replica-5, replica-8, config-server-2 являются праймари нодами

Данные опять доступны   
![Данные](/images/image13.png)
