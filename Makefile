# Cargo commands
CARGO := cargo

# Logger
define log_info
	echo -e "[\033[0;33m*\033[0m] $(1)"
endef

define log_success
	echo -e "[\033[0;32m+\033[0m] Done"
endef

# Targets for different architectures
all: x64 x86
debug: x64-debug x86-debug

# x64 builds
x64:
	@ $(call log_info,Compiling x64 project)
	@ $(CARGO) build -q --release --target x86_64-pc-windows-gnu
	@ $(call log_success)

x64-debug:
	@ $(call log_info,Compiling x64 project (debug))
	@ $(CARGO) build -q --features debug --target x86_64-pc-windows-gnu
	@ $(call log_success)

# x86 builds
x86:
	@ $(call log_info,Compiling x86 project)
	@ $(CARGO) build -q --release --target i686-pc-windows-gnu
	@ $(call log_success)

x86-debug:
	@ $(call log_info,Compiling x86 project (debug))
	@ $(CARGO) build -q --features debug --target i686-pc-windows-gnu
	@ $(call log_success)

clean:
	@ $(call log_info,Cleaning build artifacts)
	@ rm -rf target
	@ $(call log_success)

.PHONY: all debug x64 x86 x64-debug x86-debug clean