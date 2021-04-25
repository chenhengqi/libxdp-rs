gen:
	bindgen wrapper.h -o src/bindings.rs \
		--with-derive-default --with-derive-partialeq
