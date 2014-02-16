all: bin

bin:
	mkdir -p bin
	cp -r ./assets ./bin
	rustc --out-dir=bin src/main.rs

clean:
	rm -rf bin/
