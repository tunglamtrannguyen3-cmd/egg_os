#!/usr/bin/env bash
set -e

echo "=== 1. Compiling Host and Security Unikernels ==="
cargo build --manifest-path host_kernel/Cargo.toml --release
cargo build --manifest-path security_kernel/Cargo.toml --release

# Locate your compiled binaries
HOST_BIN="target/x86_64-unknown-none/release/host_kernel"
SECURITY_BIN="target/x86_64-unknown-none/release/security_kernel"

echo "=== 2. Setting Up ISO Staging Directory ==="
# Clean and create a temporary folder that represents the CD-ROM root
rm -rf iso_root
mkdir -p iso_root/boot/limine

echo "=== 3. Copying Kernels and Config to CD Root ==="
# Limine needs these at the root level of the ISO
cp limine.conf iso_root/
cp "$HOST_BIN" iso_root/host_kernel.elf
cp "$SECURITY_BIN" iso_root/security_kernel.elf

echo "=== 4. Fetching Limine Boot Files from Arch Linux ==="
# On Arch, the 'limine' package installs its CD deployment stages here:
ARCH_LIMINE_DIR="/usr/share/limine"

if [ -d "$ARCH_LIMINE_DIR" ]; then
    cp "$ARCH_LIMINE_DIR/limine-bios-cd.bin" iso_root/boot/limine/
    cp "$ARCH_LIMINE_DIR/limine-uefi-cd.bin" iso_root/boot/limine/
    cp "$ARCH_LIMINE_DIR/limine.sys" iso_root/boot/limine/
else
    echo "Error: Limine system files not found at $ARCH_LIMINE_DIR."
    echo "Please run: sudo pacman -S limine"
    exit 1
fi

echo "=== 5. Packaging into Bootable egg_os.iso ==="
# This standard xorriso command builds a modern hybrid (BIOS + UEFI) bootable ISO
xorriso -as mkisofs -R -J \
    -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot boot/limine/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o egg_os.iso

# Clean up the temporary folder after building
rm -rf iso_root

echo "=== SUCCESS: egg_os.iso is ready! ==="
echo "Boot with: qemu-system-x86_64 -cpu host -enable-kvm -m 512M -cdrom egg_os.iso"
