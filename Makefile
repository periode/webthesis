thesis:
	cd parser && cargo run -- -i /home/pierre/forschung/phd/redaction/thesis.tex

test:
	cd parser && cargo test

data: thesis
	cp ./parser/output/full.json ./web/src/data/full.json
	cp ./parser/output/toc.json ./web/src/data/toc.json
	cp ./parser/output/front.json ./web/src/data/front.json
	cp ./parser/output/introduction.json ./web/src/data/introduction.json
	cp ./parser/output/ideals.json ./web/src/data/ideals.json
	cp ./parser/output/understanding.json ./web/src/data/understanding.json
	cp ./parser/output/beauty.json ./web/src/data/beauty.json
	cp ./parser/output/programming.json ./web/src/data/programming.json
	cp ./parser/output/conclusion.json ./web/src/data/conclusion.json
	cp ./parser/output/listings.json ./web/src/data/listings.json
	cp /home/pierre/forschung/phd/redaction/images/* ./web/static/images
	cp /home/pierre/forschung/phd/redaction/corpus/* ./web/src/corpus
	cp /home/pierre/forschung/phd/redaction/thesis.pdf ./web/static/Depaz_AestheticsUnderstandingSourceCode.pdf
	pandoc /home/pierre/forschung/phd/redaction/thesis.bib -t csljson -o ./web/src/data/bib.json

dev:
	cd web && npm run dev -- --open
build:
	cd web && npm run build

upload: 
	rsync -zvur web/build/ enframed:/mnt/volume_nyc3_01/www/public/thesis

all: data build upload

