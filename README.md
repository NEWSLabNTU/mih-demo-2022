# MIH 2022 Demo Repository

This repositroy stores demo programs for MIH 2022 exhibition.

## Prerequisites

**ROS 2 Galactic** is required to build the source code. For Ubuntu
users, follow the official installation guide for Debian/ubuntu
[](https://docs.ros.org/en/galactic/Installation/Ubuntu-Install-Debians.html). Arch
Linux users can install `ros2-galactic` on AUR. Other Linux users can
refer to the "fat" archive installtion guide
([link](https://docs.ros.org/en/galactic/Installation/Alternatives/Ubuntu-Install-Binary.html)).

**Rust toolchain** is required to compile Rust programs. Please
install the toolchain using the command on
[rustup.rs](https://rustup.rs/).

## Build

### Using Makefile

```bash
## Source ROS setup script
source /opt/ros/galactic/setup.sh

make build_ros_dependencies
make build
```

### Manually

```bash
## Pull dependent ROS repos
mkdir repos
vcs import repos < dependencies.repos
vcs pull repos < dependencies.repos

## Source ROS setup script
source /opt/ros/galactic/setup.sh

## Build ROS dependencies
cd repos && colcon build

## Source setup.sh for dependent ROS pacakges
source repos/install/setup.sh

## Build the project
cargo build --all-targets --release
```

## Usage

### (A) Publish Velodyne LiDAR point clouds

Edit the configuration file. Specify the LiDAR device IP address in
the `device_ip` field.

```
./repos/velodyne/velodyne_driver/config/VLP32C-velodyne_driver_node-params.yaml
```

Run the following commands in two separate terminals.

```bash
# Terminal 1
ros2 launch velodyne_driver velodyne_driver_node-VLP32C-launch.py

# Terminal 2
ros2 launch velodyne_pointcloud velodyne_convert_node-VLP32C-launch.py
```


### (B) Run 2D detection server for Kneron camera

To retrieve detected bounding boxes from a Kneron camera, connect the
Kneron board via Ethernet cable. Set the network static address to
172.23.230.N/24 (pick a N here). Start the ROS node by:

```bash
./target/release/kneron_bbox_server_node
```


### (C) Capture images from Otobrite camera

Set the video device path in the config file
`crates/v4l2_node/config/example.json5`. For example, `/dev/video0`.

Start the image capturing node.

```bash
cargo run --bin v4l2_node --release -- --config crates/v4l2_node/config/example.json5
```

### (D) Run the visualizer for Otobrite and Kneron cameras

Modify input topic names in the `crates/camera_viz/config/example.json5`
configuration file. Then,

```bash
./target/release/camera_viz --config crataes/camera_viz/config/example.json5
```

Before running the visualizer, you may run (A), (B) and (C) first.

### (E) Run `lidar_centerpoint`

Go check [wiki](https://newslabn.csie.ntu.edu.tw:3000/en/wayside-team/notes/2022-09-22_run-autoware-lidar_centerpoint) to setup autoware and Rviz2 environment. And follow the commend to run lidar_centerpoint.

### (F) Run `det_conv_node`

```bash
./target/release/det_conv_node
```

## Architecture

![](doc/ARCHITECTURE.png)
