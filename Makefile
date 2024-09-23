SHELL := /bin/bash
NAME = declarix
BASE_DIR := /etc/$(NAME)
CONFIG_FILE := $(BASE_DIR)/$(NAME).toml
BIN_DIR := /usr/bin
DIRS := config system
CONFDIRS := link recursive
SYSDIRS := home root other
USER := $(SUDO_USER)
DATA_DIR := /home/$(USER)/.local/share/$(NAME)

define make_dir
	install -d -m 0755 -o $(USER) -g users $(1)
endef


install:
	@$(call make_dir,$(BASE_DIR));
	@echo "Created base: $(BASE_DIR).";
	@touch $(CONFIG_FILE);
	@chown $(USER):users $(CONFIG_FILE);
	@echo "Created config file.";
	@for dir in $(DIRS); do \
		dir_path=$(BASE_DIR)/$$dir; \
		$(call make_dir,$$dir_path); \
		if [[ "$$dir_path" =~ "config" ]]; then \
			for subdir in $(CONFDIRS); do \
				dir_path=$(BASE_DIR)/$$dir/$$subdir; \
				$(call make_dir,$$dir_path); \
			done; \
		else \
			for subdir in $(SYSDIRS); do \
				dir_path=$(BASE_DIR)/$$dir/$$subdir; \
				if [[ "$$subdir" == "root" ]]; then \
					install -d $$dir_path; \
				else \
					$(call make_dir,$$dir_path); \
				fi; \
			done; \
		fi; \
	done;
	@echo "Created base sub-directories.";
	@install -d -m 0755 -o $(USER) -g users $(DATA_DIR);
	@echo "Created $(NAME) data-dir.";
	@install -Dm 755 ./target/release/$(NAME) -t $(BIN_DIR)
	@install -Dm 755 ./target/release/$(NAME)Root -t $(BIN_DIR)
	@echo "Moved binaries to /usr/bin"


build:
	@cargo build --release


setup:
	@rustup install stable
	@rustup default stable

cleanup:
	@cargo clean

uninstall:
	@cargo clean
	@rm -r $(DATA_DIR)
	@rm $(BIN_DIR)/$(NAME)
	@rm $(BIN_DIR)/$(NAME)Root
