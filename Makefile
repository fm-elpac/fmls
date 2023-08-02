patch_code:
	cargo fetch
	cargo metadata | deno run -A p/openwrt/get_cargo_src.js
	mkdir -p patch
	cp -r cargo_src/aws-lc-rs-1.2.1 patch
	cp -r cargo_src/s2n-tls-sys-0.0.34 patch

clean:
	rm -r patch
	rm cargo_src
