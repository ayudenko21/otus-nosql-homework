db = db.getSiblingDB("admin")
db.createRole({role: "admin", privileges: [{resource: {anyResource: true}, actions: ["anyAction"]}], roles: []})
db.createUser({user: "adminUser", pwd: "111", roles: ["admin"]})
