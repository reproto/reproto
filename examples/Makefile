.PHONY: heroic

heroic:
	(cd .. && cargo run -- --debug --backend python --out target/python --path examples heroic.v1)
	PYTHONPATH=../target/python python python/example_$@.py
