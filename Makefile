thesis:
	cd parser && cargo run -- -i /home/pierre/forschung/phd/redaction/thesis.tex

test:
	cd parser && cargo test

data: thesis
	cp ./parser/parsed.json ./web/src/routes/data.json

dev:
	cd web && npm run dev -- --open