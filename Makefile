.PHONY: build clean

build:
	@echo 'Pull dependent repositories' >&2
	@mkdir -p repos && \
	vcs import repos < dependencies.repos && \
	vcs pull repos < dependencies.repos

	@echo 'Build dependent repositories' >&2
	cd repos && \
	colcon build

	@echo 'Building this project' >&2
	@if [ -d /opt/ros2/galactic/setup.sh ] ; then \
		source /opt/ros2/galactic/setup.sh; \
	else \
		echo 'Warning: /opt/ros2/galactic/setup.zsh does not exist.' >&2; \
		echo '         Compilation might fail.' >&2; \
	fi && \
	source repos/install/setup.sh && \
	cargo build --release --all-targets

clean:
	cargo clean