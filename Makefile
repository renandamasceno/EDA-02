RUSTC = rustc
SRC = src/main.rs
BIN = programa.exe
INPUT ?= entrada.txt

.PHONY: build run clean

build: $(BIN)

$(BIN): $(SRC)
	$(RUSTC) $(SRC) -O -o $(BIN)

run: $(BIN)
	@./$(BIN) "$(INPUT)"

clean:
	rm -f $(BIN)
