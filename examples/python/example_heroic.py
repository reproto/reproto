import json

import heroic.v1 as v1

q = v1.Query.decode({
  "query": "hello world",
  "aggregation": {
    "type": "sum"
  }
})

print(q.query)
print(q.aggregation)
print(json.dumps(q.encode()))
