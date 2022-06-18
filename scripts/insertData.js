db = db.getSiblingDB("restaurant");
let waiters = ['John', 'Ken', 'Andrew', 'Jack', 'Paul', 'Kate', 'Jessica', 'Mary', 'Eva', 'Adam']; for (var i=0; i<100000; i++) { db.checks.insert({"checkId": i, "waiter": waiters[Math.floor(Math.random()*10)], amount: Math.random()*100, mealsCount: Math.floor(Math.random()*5)+1}) }
