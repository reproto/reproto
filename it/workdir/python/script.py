import sys
import json
import test

for line in sys.stdin:
    e = test.Entry.decode(json.loads(line))
    print(json.dumps(e.encode()))
