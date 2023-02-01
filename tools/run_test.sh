#!/bin/sh

tools/run_qemu.sh \
rusmikan-loader/target/x86_64-unknown-uefi/debug/rusmikan-loader.efi \
rusmikan-kernel/target/x86_64-rusmikan/debug/kernel.elf

if [ $? -eq 33 ]; then
	exit 0;
else
	exit 1;
fi
