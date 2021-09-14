.PHONY: serve-files
serve-files:
	python -m http.server 8080


.PHONY: serve-websocket
serve-websocket:
	python websocket.py


.PHONY: requirements
requirements:
	pip install -U pip
	pip install -U wheel
	pip install -U autopep8
	pip install -U pylint
	pip install -U rope
	pip install -U websockets
	pip install -U psutil
