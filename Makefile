gen:
	bindgen wrapper.h -o src/bindings.rs \
		--with-derive-default --with-derive-partialeq

clean:
	# rm -rf target
	rm -rf xdp-tools
	git checkout xdp-tools
