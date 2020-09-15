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
    let pose = na::Isometry3::from(odom_msg.pose.pose);
    println!("{}", pose);

    let mut pose2 = pose;
    pose2.translation.vector.x = -5.0;
    let pose_msg: msg::geometry_msgs::Pose = pose2.into();
    println!("{:?}", pose_msg);
}
