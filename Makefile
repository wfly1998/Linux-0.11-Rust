.PHONY: all clean qemu count disasm section elf

mode ?= debug

HDA_IMG = hdc-0.11.img

AS	= as --32
LD	= ld
LDFLAGS = -m elf_i386
CC	= gcc
CFLAGS  = -g -m32 -fno-builtin -fno-stack-protector -fomit-frame-pointer -fstrength-reduce

CPP	= cpp -nostdinc
AR	= ar
STRIP = strip
OBJCOPY = objcopy

LDFLAGS	+= -Ttext 0 -e startup_32

build_args := --target x86.json

ifeq ($(mode), release)
	build_args += --release
endif

all:	image

image:	boot/bootsect boot/setup tools/system

boot/head.o: boot/head.s
	make head.o -C boot/

boot/bootsect: boot/bootsect.s
	make bootsect -C boot

boot/setup: boot/setup.s
	make setup -C boot/

image:
	cp -f tools/system system.tmp
	$(STRIP) system.tmp
	$(OBJCOPY) -O binary -R .note -R .comment system.tmp tools/kernel
	tools/build.sh boot/bootsect boot/setup tools/kernel image $(ROOT_DEV)
	rm system.tmp
	rm -f tools/kernel
	sync

tools/system:
	cd init && cargo xbuild $(build_args)
	cp init/target/x86/$(mode)/init tools/system
	nm tools/system | grep -v '\(compiled\)\|\(\.o$$\)\|\( [aU] \)\|\(\.\.ng$$\)\|\(LASH[RL]DI\)'| sort > System.map

clean:
	make clean -C boot/
	rm -rf image System.map tools/system
	cd include && cargo clean
	cd init && cargo clean
	cd kernel && cargo clean

qemu: image
	qemu-system-i386 -m 16 -boot a -fda image -hda hdc-0.11.img

debug: image
	qemu-system-i386 -m 16 -boot a -fda image -hda hdc-0.11.img -s -S&
	sleep 1
	terminal -e "gdb -q -tui -x tools/gdbinit"

count:
	find . -name '*.rs' | xargs wc -l

disasm: tools/system
	objdump -d tools/system | less

section: tools/system
	objdump -h tools/system | less

elf: tools/system
	readelf -h tools/system | less

