thesis:
	cd parser && cargo run -- -i /home/pierre/forschung/phd/redaction/thesis.tex

test:
	cd parser && cargo test

data: thesis
	cp ./parser/parsed.json ./web/src/routes/text.json
	cp /home/pierre/forschung/phd/redaction/images/* ./web/static/images
	cp /home/pierre/forschung/phd/redaction/corpus/* ./web/src/corpus
	pandoc /home/pierre/forschung/phd/redaction/thesis.bib -t csljson -o ./web/src/routes/bib.json

dev:
	cd web && npm run dev -- --open
build:
	cd web && npm run build

upload: 
	rsync -zvur web/build/ enframed:/mnt/volume_nyc3_01/www/public/thesis
