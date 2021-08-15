dev:
	yarn tauri dev;
clean:
	cd src-tauri ; cargo clean ; cd .. ;
bundle: 
	yarn tauri build;