rs.initiate(
    {
        "_id": "ReplicaSet2",
        members:
            [
                {
                    "_id": 0,
                    host: "replica-4:27017"
                },
                {
                    "_id": 1,
                    host: "replica-5:27017"
                },
                {
                    "_id": 2,
                    host: "replica-6:27017",
                    arbiterOnly: true
                }
            ]
    }
)
