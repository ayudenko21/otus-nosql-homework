rs.initiate(
    {
        "_id": "ReplicaSet1",
        members:
            [
                {
                    "_id": 0,
                    host: "replica-1:27017"
                },
                {
                    "_id": 1,
                    host: "replica-2:27017"
                },
                {
                    "_id": 2,
                    "host": "replica-3:27017",
                    arbiterOnly: true
                }
            ]
    }
)
