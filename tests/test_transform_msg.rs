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

#![cfg(target_os = "linux")]

mod tests {
    use nalgebra as na;

    mod my_msg {
        #[rustfmt::skip]
        rosrust::rosmsg_include!(
            geometry_msgs / Transform,
        );
    }

    ros_nalgebra::ros_nalgebra_msg!(my_msg, Quaternion);
    ros_nalgebra::ros_nalgebra_msg!(my_msg, Vector3);
    ros_nalgebra::ros_nalgebra_msg!(my_msg, Transform);

    #[test]
    #[allow(clippy::float_cmp)]
    fn pose_to_iso() {
        let mut pose_msg = my_msg::geometry_msgs::Transform::default();
        pose_msg.translation.x = 1.0;
        let pose = na::Isometry3::from(pose_msg);
        assert_eq!(pose.translation.x, 1.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn pose_into_iso2() {
        let mut pose_msg = my_msg::geometry_msgs::Transform::default();
        pose_msg.translation.x = 1.0;
        let pose2: na::Isometry3<f64> = pose_msg.into();
        assert_eq!(pose2.translation.x, 1.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_from() {
        let pose1 = na::Isometry3::identity();
        let msg = my_msg::geometry_msgs::Transform::from(pose1);
        assert_eq!(msg.translation.y, 0.0);
    }
}
