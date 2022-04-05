test:
	./scripts/test.sh

clean:
	rm -f ./res/tmp*

.PHONY:
	test clean
