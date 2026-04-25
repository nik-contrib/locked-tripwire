#![no_std]

// The tripwire is triggered.
compile_error!(include_str!(concat!(env!("OUT_DIR"), "/message.txt")));
