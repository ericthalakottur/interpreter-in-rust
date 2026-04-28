TARGET_DIR=build
.PHONY: all clean build

all: build

clean:
	rm -r $(TARGET_DIR) expr.rs

build:
	rustc generate_ast.rs --out-dir $(TARGET_DIR) && ./build/generate_ast .
