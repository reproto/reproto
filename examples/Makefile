.PHONY: heroic

heroic:
	(cd .. && cargo run -- --debug --backend plain_python --out target/python --path examples heroic.v1)
	PYTHONPATH=../target/python python python/example_$@.py
