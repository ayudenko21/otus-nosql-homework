sh.enableSharding("restaurant")
db = db.getSiblingDB("config")
db.settings.save({_id: "chunksize", value: 1})
