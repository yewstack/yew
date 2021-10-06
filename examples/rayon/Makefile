dist:
	wasm-pack build --target web --no-typescript
	cp static/* pkg/

serve:
	cd pkg/ && python3 ../serve.py

clean:
	rm -rf pkg/
