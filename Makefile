NAME = declarch
BASE_DIR := /etc/$(NAME)
CONFIG_FILE := $(BASE_DIR)/$(NAME).toml
BIN_DIR := /usr/bin
DIRS := config special_config secure_config secure_special_config system special_system secure_system secure_special_system
SUBDIRS := home other root
USER := $(SUDO_USER)
DATA_DIR := /home/$(USER)/.local/share/$(NAME)

define make_dir
	install -d -m 0755 -o $(USER) -g users $(1)
endef

define make_dir_root
	mkdir -p $(1); \
	chown root:root $(1)
endef

all: create_dirs build install cleanup

create_dirs:
	@$(call make_dir,$(BASE_DIR));
	@echo "Created base: $(BASE_DIR).";
	@touch $(CONFIG_FILE);
	@chown $(USER):users $(CONFIG_FILE);
	@echo "Created config file.";
	@for dir in $(DIRS); do \
		dir_path=$(BASE_DIR)/$$dir; \
		$(call make_dir,$$dir_path); \
		if [[ "$$dir_path" =~ "system" ]]; then \
			for subdir in $(SUBDIRS); do \
				dir_path=$(BASE_DIR)/$$dir/$$subdir; \
				if [ "$$subdir" != "root" ]; then \
					$(call make_dir,$$dir_path); \
				else \
					$(call make_dir_root,$$dir_path); \
				fi; \
			done; \
		fi; \
	done;
	@echo "Created base sub-directories.";
	@install -d -m 0755 -o $(USER) -g users $(DATA_DIR);
	@echo "Created declarch data-dir.";


build:
	@cargo build --release


install:
	@install -Dm 755 ./target/release/$(NAME) -t $(BIN_DIR)
	@install -Dm 755 ./target/release/$(NAME)Root -t $(BIN_DIR)


setup:
	@rustup install stable
	@rustup default stable

cleanup:
	@cargo clean
