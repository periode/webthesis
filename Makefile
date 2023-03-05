thesis:
	cd parser && cargo run -- -i /home/pierre/forschung/phd/redaction/thesis.tex

test:
	cd parser && cargo test

data: thesis
	cp ./parser/output/text.json ./web/src/routes/text.json
	cp ./parser/output/toc.json ./web/src/routes/toc.json
	cp ./parser/output/introduction.json ./web/src/routes/introduction.json
	cp ./parser/output/ideals.json ./web/src/routes/ideals.json
	cp ./parser/output/understanding.json ./web/src/routes/understanding.json
	cp ./parser/output/beauty.json ./web/src/routes/beauty.json
	cp ./parser/output/programming.json ./web/src/routes/programming.json
	cp ./parser/output/conclusion.json ./web/src/routes/conclusion.json
	cp /home/pierre/forschung/phd/redaction/images/* ./web/static/images
	cp /home/pierre/forschung/phd/redaction/corpus/* ./web/src/corpus
	cp /home/pierre/forschung/phd/redaction/thesis.pdf ./web/static/Depaz_AestheticsUnderstandingSourceCode.pdf
	pandoc /home/pierre/forschung/phd/redaction/thesis.bib -t csljson -o ./web/src/routes/bib.json

dev:
	cd web && npm run dev -- --open
build:
	cd web && npm run build

upload: 
	rsync -zvur web/build/ enframed:/mnt/volume_nyc3_01/www/public/thesis

all: data build upload

