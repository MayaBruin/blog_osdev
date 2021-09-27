boot:
	cd efi; cargo +nightly xbuild --target x86_64-unknown-uefi
	cp efi/target/x86_64-unknown-uefi/debug/efi.efi ./BOOTX64.efi

disk: boot
	dd if=/dev/zero of=fat.img bs=1k count=1440
	mformat -i fat.img -f 1440 ::
	mmd -i fat.img ::/EFI
	mmd -i fat.img ::/EFI/BOOT
	mcopy -i fat.img BOOTX64.efi ::/EFI/BOOT

test: disk
	qemu-system-x86_64 -pflash OVMF.fd -usb fat.img

clean:
	rm BOOTX64.efi fat.img
