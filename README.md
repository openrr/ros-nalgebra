# ros-nalgebra

[![Build Status](https://img.shields.io/github/workflow/status/openrr/ros-nalgebra/CI/main)](https://github.com/openrr/ros-nalgebra/actions) [![crates.io](https://img.shields.io/crates/v/ros-nalgebra.svg)](https://crates.io/crates/ros-nalgebra) [![docs](https://docs.rs/ros-nalgebra/badge.svg)](https://docs.rs/ros-nalgebra)

Generate code to convert `geometry_msgs` into nalgebra structs, for [rosrust](https://github.com/adnanademovic/rosrust).

## Pre-requirements & dependencies

* [ROS](https://ros.org)
* [Rust](https://rust-lang.org)
* [rosrust](https://github.com/adnanademovic/rosrust)
* [nalgebra](https://nalgebra.org/)

## How to use

### Easy usage: `ros_nalgebra::rosmsg_include!()`

Use `ros_nalgebra::rosmsg_include` instead of `rosrust::rosmsg_include` in your code.

```rust
use nalgebra as na;

mod msg {
    ros_nalgebra::rosmsg_include!(nav_msgs / Odometry);
}

fn main() {
    let mut odom_msg = msg::nav_msgs::Odometry::default();
    odom_msg.pose.pose.position.x = 1.0;
    odom_msg.pose.pose.position.y = -1.0;
    odom_msg.pose.pose.position.z = 2.0;
    odom_msg.pose.pose.orientation.x = 0.0;
    odom_msg.pose.pose.orientation.y = 0.0;
    odom_msg.pose.pose.orientation.z = 0.0;
    odom_msg.pose.pose.orientation.w = 1.0;

    // convert into nalgebra::Isometry<f64> by `from()`
    let pose = na::Isometry3::from(odom_msg.pose.pose);
    println!("{}", pose);

    let mut pose2 = pose.clone();
    pose2.translation.vector.x = -5.0;

    // convert into ROS msg using `into()`
    let pose_msg: msg::geometry_msgs::Pose = pose2.into();
    println!("{:?}", pose_msg);
}
```

#### Automatically defined messages by `ros_nalgebra::rosmsg_include!()`

Below messages are automatically included by `ros_nalgebra::rosmsg_include!()`. Do not include them in your code.

```text
geometry_msgs/Point,
geometry_msgs/Pose,
geometry_msgs/Quaternion,
geometry_msgs/Transform,
geometry_msgs/Vector3,
```

## Other usage: `ros_nalgebra!() and ros_nalgebra_msg!()`

If some messages are included already (for example in other crate), you can use `ros_nalgebra_msg!()`. The arguments are the rust namespace of the `geometry_msgs` (example:`msg`) and the message type (example: `Pose` for `geometry_msgs/Pose`, `Point` for `geometry_msgs/Point`).

### Example

In `some_other_crate`,

```rust
mod msg {
    rosrust::rosmsg_include!(geometry_msgs/Point);
}
```

Then you can use `ros_nalgebra::ros_nalgebra_msg!()` in your crate.

```rust,ignore
// generate conversion code only for `geometry_msgs/Point` which is defined in `some_other_crate::msg`.
ros_nalgebra::ros_nalgebra_msg!(some_other_crate::msg, Point);
```

## Supported conversions

See lib.rs.

## TODO

Handle dependencies.
