db = db.getSiblingDB("restaurant");
db.checks.createIndex({waiter: 1})
