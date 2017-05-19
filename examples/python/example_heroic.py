import json

import heroic.v1 as v1

q = v1.Query.decode({
  "query": "hello world",
  "aggregation": {
    "type": "sum",
  }
})

d = json.loads(json.dumps(q.encode()))

q2 = v1.Query.decode(d)

print(q2.encode())
