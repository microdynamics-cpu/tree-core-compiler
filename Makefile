setup:
	./scripts/setup.sh

test-x86:
	@printf "[\033[0;33mx86 test start...\033[0m]\n"
	@./scripts/test.sh -x
	@printf "[\033[0;32mx86 test done\033[0m]\n"

test-riscv:
	@printf "[\033[0;33mriscv test start...\033[0m]\n"
	@./scripts/test.sh -r
	@printf "[\033[0;32mriscv test done\033[0m]\n"

test: test-x86 test-riscv
	@printf "[\033[0;32mall-done\033[0m]\n"

clean:
	rm -f ./res/tmp*

.PHONY:
	setup \
	test-x86 test-riscv test\
	clean
