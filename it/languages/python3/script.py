import sys
import json
import test

for line in sys.stdin:
    e = test.Entry.decode(json.loads(line))

    sys.stdout.write("#<>")
    sys.stdout.write(json.dumps(e.encode()))
    sys.stdout.write("\n")
    sys.stdout.flush()
