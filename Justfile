b: build
build:
    rm -rf out
    cargo r -- --drafts
    cp -r static/* out

s: serve
serve: build
    miniserve  ./out/ --index index.html -p 8080
