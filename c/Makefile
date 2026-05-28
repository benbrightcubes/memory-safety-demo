CC ?= gcc

.PHONY: all run asan clean

all: run

# Stap 1 — zonder sanitizer. De freed buffer wordt hergebruikt door de
# volgende malloc; de tweede printf leest de inhoud van de hergebruikte slot.
run:
	@echo "=== gcc (zonder sanitizer) ==="
	$(CC) -Wall -O0 demo.c -o demo
	./demo
	@echo ""

# Stap 2 — met AddressSanitizer. ASAN vangt de use-after-free runtime
# en print een gedetailleerd rapport, inclusief stack-traces voor
# allocatie, vrijgave en gebruik.
asan:
	@echo "=== gcc -fsanitize=address (met ASAN) ==="
	$(CC) -fsanitize=address -g -O0 demo.c -o demo_asan
	./demo_asan || true
	@echo ""

clean:
	rm -f demo demo_asan
