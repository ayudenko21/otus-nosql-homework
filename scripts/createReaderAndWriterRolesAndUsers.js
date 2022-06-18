db = db.getSiblingDB("restaurant")
db.createRole({role: "reader", privileges: [{resource: {db: "restaurant", collection: "checks"}, actions: ["find"]}], roles: []})
db.createRole({role: "writer", privileges: [{resource: {db: "restaurant", collection: "checks"}, actions: ["find", "insert", "update", "remove"]}], roles: []})

db.createUser({user: "readerUser", pwd: "111", roles: [{role: "reader", db: "restaurant"}]})
db.createUser({user: "writerUser", pwd: "111", roles: [{role: "writer", db: "restaurant"}]})
