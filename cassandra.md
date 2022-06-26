# Домашняя работа по лекции 07

## Cassandra

1. Запускаем minicube для работы с k8s   
![Старт minikube](/images/image1.png)

2. Запускаем сервис cassandra состоящий из 3 под   
`kubectl apply -f cassandra-service.yaml`   
`kubectl apply -f cassandra-statefulset.yaml`
![Старт k8s](/images/image2.png)

3. Список подов образующих ring    
![Статус сервиса](/images/image3.png)   

4. Статус ring   
![Статус ring](/images/image4.png)   

5. Запуск стресс-тестов   
`/usr/local/apache-cassandra-3.11.2/tools/bin/cassandra-stress write n=1000000`   
![Write](/images/image5.png)

`/usr/local/apache-cassandra-3.11.2/tools/bin/cassandra-stress read n=1000000`   
![Write](/images/image6.png)
