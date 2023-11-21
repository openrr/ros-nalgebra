#![cfg(target_os = "linux")]

mod nested {
    pub(crate) mod msg {
        rosrust::rosmsg_include!(
            geometry_msgs / Point,
            geometry_msgs / Pose,
            geometry_msgs / Quaternion,
            geometry_msgs / Transform,
            geometry_msgs / Vector3,
        );
    }
}
ros_nalgebra::ros_nalgebra!(nested::msg);
