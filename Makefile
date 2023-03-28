build:
	docker compose build

up:
	docker compose up -d

down:
	docker compose down

send_email:
	curl -X POST http://localhost:$(PORT)/send

fetch_inbox:
	curl -X GET http://localhost:$(PORT)/inbox
