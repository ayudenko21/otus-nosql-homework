rs.initiate(
    {
        "_id": "ReplicaSet3",
        members:
            [
                {
                    "_id": 0,
                    host: "replica-7:27017"
                },
                {
                    "_id": 1,
                    host: "replica-8:27017"
                },
                {
                    "_id": 2,
                    host: "replica-9:27017",
                    arbiterOnly: true
                }
            ]
    }
)
