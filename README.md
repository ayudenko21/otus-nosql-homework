# Домашняя работа 09

Программа для тестирования скорости чтения и записи Redis написана на Rust.

Файл с данными содержит 240000 записей и занимает на диске примерно 20.3Mb.
Данные в файле созданы используя рандомные значения и сериализованы в JSON.

Данные сохраняются в Redis в виде:
1. Все содержимое файла сохраняется как строка
2. Все содержимое файла сохраняется как набор строк, где каждая строка - отдельная запись сериализованная в JSON
3. Все содержимое файла сохраняется как набор хешей, каждый хеш формируется из отдельной записи
4. Все содержимое файла сохраняется в структуру list
5. Все содержимое файла сохраняется в структуру sorted set, все записи имеют одинаковый вес 1

Результаты:   
![image1](/images/image1.png)   
![image2](/images/image2.png)
