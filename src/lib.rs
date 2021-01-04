/*
  Copyright 2020 Smile Robotics, Inc

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/
//! Generate code to convert `geometry_msgs` into nalgebra structs, for [rosrust](https://github.com/adnanademovic/rosrust).
//!
//! ## Pre-requirements & dependencies
//!
//! * [ROS](https://ros.org)
//! * [Rust](https://rust-lang.org)
//! * [rosrust](https://github.com/adnanademovic/rosrust)
//! * [nalgebra](https://nalgebra.org/)
//!
//! ## How to use
//!
//! ### Easy usage: `ros_nalgebra::rosmsg_include!()`
//!
//! Use `ros_nalgebra::rosmsg_include` instead of `rosrust::rosmsg_include` in your code.
//!
//! # Example
//!
//! ```
//! mod msg {
//!    ros_nalgebra::rosmsg_include!(nav_msgs/Odometry);
//! }
//! ```

/// Use this macro instead of rosrust::rosmsg_include!() to generate `impl From`.
///
/// # Example
///
/// ```
/// mod msg {
///    ros_nalgebra::rosmsg_include!(nav_msgs/Odometry);
/// }
/// ```
#[macro_export]
macro_rules! rosmsg_include {
    ( $($ns:ident / $msg:ident),* $(,)?) => {
        ::rosrust::rosmsg_include!(
            geometry_msgs/Point,
            geometry_msgs/Pose,
            geometry_msgs/Quaternion,
            geometry_msgs/Transform,
            geometry_msgs/Vector3,
            $($ns/$msg),*
        );
        ::ros_nalgebra::ros_nalgebra!(self);
    }
}

/// Generate `impl From<<ROS_MESSAGE>` and `impl From<NALGEBRA_STRUCT>`
/// # Example
///
/// ```
/// mod msg {
///    rosrust::rosmsg_include!(geometry_msgs/Pose);
/// }
/// ros_nalgebra::ros_nalgebra_msg!(msg, Quaternion);
/// ros_nalgebra::ros_nalgebra_msg!(msg, Point);
/// ros_nalgebra::ros_nalgebra_msg!(msg, Pose);
/// let pose: nalgebra::Isometry3<f64> = msg::geometry_msgs::Pose::default().into();
/// ```
#[macro_export]
macro_rules! ros_nalgebra_msg {
    ($ns:ident, Vector3) => {
        // Vector3
        impl From<$ns::geometry_msgs::Vector3> for ::nalgebra::Vector3<f64> {
            fn from(vec_msg: $ns::geometry_msgs::Vector3) -> Self {
                ::nalgebra::Vector3::new(vec_msg.x, vec_msg.y, vec_msg.z)
            }
        }

        impl From<::nalgebra::Vector3<f64>> for $ns::geometry_msgs::Vector3 {
            fn from(vec: ::nalgebra::Vector3<f64>) -> Self {
                $ns::geometry_msgs::Vector3 {
                    x: vec.x,
                    y: vec.y,
                    z: vec.z,
                }
            }
        }

        // Vector3 <-> Translation3
        impl From<$ns::geometry_msgs::Vector3> for ::nalgebra::Translation3<f64> {
            fn from(vec_msg: $ns::geometry_msgs::Vector3) -> Self {
                ::nalgebra::Translation3::new(vec_msg.x, vec_msg.y, vec_msg.z)
            }
        }

        impl From<::nalgebra::Translation3<f64>> for $ns::geometry_msgs::Vector3 {
            fn from(p: ::nalgebra::Translation3<f64>) -> Self {
                $ns::geometry_msgs::Vector3 {
                    x: p.vector.x,
                    y: p.vector.y,
                    z: p.vector.z,
                }
            }
        }
    };
    ($ns:ident, Point) => {
        // Point <-> Translation3
        impl From<$ns::geometry_msgs::Point> for ::nalgebra::Translation3<f64> {
            fn from(vec_msg: $ns::geometry_msgs::Point) -> Self {
                ::nalgebra::Translation3::new(vec_msg.x, vec_msg.y, vec_msg.z)
            }
        }

        impl From<::nalgebra::Translation3<f64>> for $ns::geometry_msgs::Point {
            fn from(p: ::nalgebra::Translation3<f64>) -> Self {
                $ns::geometry_msgs::Point {
                    x: p.vector.x,
                    y: p.vector.y,
                    z: p.vector.z,
                }
            }
        }

        // Point <-> Point3
        impl From<$ns::geometry_msgs::Point> for ::nalgebra::Point3<f64> {
            fn from(vec_msg: $ns::geometry_msgs::Point) -> Self {
                ::nalgebra::Point3::new(vec_msg.x, vec_msg.y, vec_msg.z)
            }
        }

        impl From<::nalgebra::Point3<f64>> for $ns::geometry_msgs::Point {
            fn from(p: ::nalgebra::Point3<f64>) -> Self {
                $ns::geometry_msgs::Point {
                    x: p.coords.x,
                    y: p.coords.y,
                    z: p.coords.z,
                }
            }
        }
    };
    ($ns:ident, Quaternion) => {
        // Quaternion <-> UnitQuaternion
        impl From<$ns::geometry_msgs::Quaternion> for ::nalgebra::UnitQuaternion<f64> {
            fn from(q_msg: $ns::geometry_msgs::Quaternion) -> Self {
                ::nalgebra::UnitQuaternion::from_quaternion(::nalgebra::Quaternion::new(
                    q_msg.w, q_msg.x, q_msg.y, q_msg.z,
                ))
            }
        }

        impl From<::nalgebra::UnitQuaternion<f64>> for $ns::geometry_msgs::Quaternion {
            fn from(q: ::nalgebra::UnitQuaternion<f64>) -> Self {
                $ns::geometry_msgs::Quaternion {
                    x: q.coords.x,
                    y: q.coords.y,
                    z: q.coords.z,
                    w: q.coords.w,
                }
            }
        }

        // Quaternion
        impl From<$ns::geometry_msgs::Quaternion> for ::nalgebra::Quaternion<f64> {
            fn from(q_msg: $ns::geometry_msgs::Quaternion) -> Self {
                ::nalgebra::Quaternion::new(q_msg.w, q_msg.x, q_msg.y, q_msg.z)
            }
        }

        impl From<::nalgebra::Quaternion<f64>> for $ns::geometry_msgs::Quaternion {
            fn from(q: ::nalgebra::Quaternion<f64>) -> Self {
                $ns::geometry_msgs::Quaternion {
                    x: q.coords.x,
                    y: q.coords.y,
                    z: q.coords.z,
                    w: q.coords.w,
                }
            }
        }
    };
    ($ns:ident, Pose) => {
        // Pose <-> Isometry3
        impl From<$ns::geometry_msgs::Pose> for ::nalgebra::Isometry3<f64> {
            fn from(pose_msg: $ns::geometry_msgs::Pose) -> Self {
                ::nalgebra::Isometry3::from_parts(
                    pose_msg.position.into(),
                    pose_msg.orientation.into(),
                )
            }
        }

        impl From<::nalgebra::Isometry3<f64>> for $ns::geometry_msgs::Pose {
            fn from(pose: ::nalgebra::Isometry3<f64>) -> Self {
                $ns::geometry_msgs::Pose {
                    position: pose.translation.into(),
                    orientation: pose.rotation.into(),
                }
            }
        }
    };
    ($ns:ident, Transform) => {
        // Transform <-> Isometry3
        impl From<$ns::geometry_msgs::Transform> for ::nalgebra::Isometry3<f64> {
            fn from(pose_msg: $ns::geometry_msgs::Transform) -> Self {
                ::nalgebra::Isometry3::from_parts(
                    pose_msg.translation.into(),
                    pose_msg.rotation.into(),
                )
            }
        }

        impl From<::nalgebra::Isometry3<f64>> for $ns::geometry_msgs::Transform {
            fn from(pose: ::nalgebra::Isometry3<f64>) -> Self {
                $ns::geometry_msgs::Transform {
                    translation: pose.translation.into(),
                    rotation: pose.rotation.into(),
                }
            }
        }
    };
}

/// Generate Point/Vector3/Quaternion/Pose/Transform converter in `mod $ns`.
/// `$ns` is the namespace for the rust messages.
/// For example `msg`.
///
/// # Example
///
/// ```
/// mod msg {
///   rosrust::rosmsg_include!(
///     geometry_msgs/Point,
///     geometry_msgs/Pose,
///     geometry_msgs/Quaternion,
///     geometry_msgs/Transform,
///     geometry_msgs/Vector3,
///   );
/// }
/// ros_nalgebra::ros_nalgebra!(msg);
/// ```
#[macro_export]
macro_rules! ros_nalgebra {
    ($ns:ident) => {
        ::ros_nalgebra::ros_nalgebra_msg!($ns, Point);
        ::ros_nalgebra::ros_nalgebra_msg!($ns, Vector3);
        ::ros_nalgebra::ros_nalgebra_msg!($ns, Quaternion);
        ::ros_nalgebra::ros_nalgebra_msg!($ns, Pose);
        ::ros_nalgebra::ros_nalgebra_msg!($ns, Transform);
    };
}
