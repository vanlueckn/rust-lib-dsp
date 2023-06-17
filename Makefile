bindgen:
	bindgen libdsp/include/lib_dsp.h -o src/bindings.rs --opaque-type "std::.+" --blocklist-type "(iterator|__min|__max)\b"  --blocklist-item "__mingw.*" --enable-cxx-namespaces  -- -x c++
