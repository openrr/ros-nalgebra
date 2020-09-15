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
#[cfg(test)]
mod tests {
    use nalgebra as na;

    mod msg2 {
        #[rustfmt::skip]
        ros_nalgebra::rosmsg_include!(
            nav_msgs / Odometry,
            sensor_msgs / Imu,
        );
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn pose_to_iso() {
        let mut pose_msg = msg2::geometry_msgs::Pose::default();
        pose_msg.position.x = 1.0;
        let pose = na::Isometry3::from(pose_msg);
        assert_eq!(pose.translation.x, 1.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn pose_into_iso2() {
        let mut pose_msg = msg2::geometry_msgs::Pose::default();
        pose_msg.position.x = 1.0;
        let pose2: na::Isometry3<f64> = pose_msg.into();
        assert_eq!(pose2.translation.x, 1.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn odometry_into_iso2() {
        let mut odom_msg = msg2::nav_msgs::Odometry::default();
        odom_msg.pose.pose.position.x = 3.0;
        let pose2: na::Isometry3<f64> = odom_msg.pose.pose.into();
        assert_eq!(pose2.translation.x, 3.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn imu_into_iso() {
        let mut imu_msg = msg2::sensor_msgs::Imu::default();
        imu_msg.orientation.x = 1.0;
        imu_msg.angular_velocity.y = 2.0;
        let o1: na::Quaternion<f64> = imu_msg.orientation.into();
        let v1: na::Vector3<f64> = imu_msg.angular_velocity.into();
        assert_eq!(o1.coords.x, 1.0);
        assert_eq!(v1.y, 2.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_from() {
        let pose1 = na::Isometry3::identity();
        let msg = msg2::geometry_msgs::Pose::from(pose1);
        assert_eq!(msg.position.y, 0.0);
    }
}
