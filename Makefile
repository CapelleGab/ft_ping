binFile = target/debug/ft_ping

run:
	cargo build
	sudo ./$(binFile) $(ARGS)

clean:
	cargo clean