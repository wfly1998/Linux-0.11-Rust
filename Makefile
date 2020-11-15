HDA_IMG = hdc-0.11.img

include Makefile.header

LDFLAGS	+= -Ttext 0 -e startup_32
# ARCHIVES=kernel/kernel.o mm/mm.o fs/fs.o

all:	image

image:	boot/bootsect boot/setup tools/system

boot/head.o: boot/head.s
	@make head.o -C boot/

boot/bootsect: boot/bootsect.s
	@make bootsect -C boot

boot/setup: boot/setup.s
	@make setup -C boot

tools/system:	# boot/head.o init/main.o \
		# $(ARCHIVES) $(DRIVERS) $(MATH) $(LIBS)
	@$(LD) $(LDFLAGS) boot/head.o init/main.o \
	# $(ARCHIVES) \
	# $(DRIVERS) \
	# $(MATH) \
	# $(LIBS) \
	-o tools/system
	@nm tools/system | grep -v '\(compiled\)\|\(\.o$$\)\|\( [aU] \)\|\(\.\.ng$$\)\|\(LASH[RL]DI\)'| sort > System.map
