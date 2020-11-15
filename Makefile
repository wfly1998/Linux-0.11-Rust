.PHONY: all image kernel clean qemu
HDA_IMG = hdc-0.11.img

include Makefile.header

LDFLAGS	+= -Ttext 0 -e startup_32

all:	image

image:	boot/bootsect boot/setup tools/system

boot/head.o: boot/head.s
	@make head.o -C boot/

boot/bootsect: boot/bootsect.s
	@make bootsect -C boot

boot/setup: boot/setup.s
	@make setup -C boot/

image:
	@cp -f tools/system system.tmp
	@$(STRIP) system.tmp
	@$(OBJCOPY) -O binary -R .note -R .comment system.tmp tools/kernel
	@tools/build.sh boot/bootsect boot/setup tools/kernel image $(ROOT_DEV)
	@rm system.tmp
	@rm -f tools/kernel
	@sync

kernel:
	@cd init && cargo xbuild

tools/system: kernel
	@cp init/target/x86/debug/init tools/system
	@nm tools/system | grep -v '\(compiled\)\|\(\.o$$\)\|\( [aU] \)\|\(\.\.ng$$\)\|\(LASH[RL]DI\)'| sort > System.map

clean:
	@make clean -C boot/
	@rm -rf image System.map tools/system
	@cd init && cargo clean

qemu: image
	qemu-system-x86_64 -m 16 -boot a -fda image -hda hdc-0.11.img

