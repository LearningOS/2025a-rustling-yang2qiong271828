//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 测试8：定义 cfg 条件
    println!("cargo:rustc-cfg=pass");
}