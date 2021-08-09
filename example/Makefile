CC := riscv64-unknown-elf-gcc
OBJDUMP := riscv64-unknown-elf-objdump

binary:
	$(CC) -fno-common -fno-builtin-printf -march=rv32i -mabi=ilp32 -c main.c

dump:
	$(OBJDUMP) -h -d main.o > dump

.PHONY: binary dump 
