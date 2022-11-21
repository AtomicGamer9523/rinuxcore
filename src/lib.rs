//
// MIT License
//
// Copyright (c) 2022 AtomicGamer9523
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

/*! # Rinux

## OS, written in rust

### Basic Example:

```rust
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rinuxcore::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rinuxcore::{
    println,
    task::{executor::Executor, Task},
    BootInfo
};

#[rinuxcore::main]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rinuxcore::init(&boot_info);

    let mut executor = Executor::new();
    executor.spawn(Task::new(rinuxcore::task::keyboard::init()));
    executor.spawn(Task::new(main()));

    executor.run()
}

async fn main() {
    println!("Hello World");
}



#[panic_handler]
fn panic(info: &std3::panic::PanicInfo) -> ! {
    rinuxcore::print_err!("{}", info);
    rinuxcore::hlt_loop();
}
```

[STD3 Docs Here](https://www.github.linkrbot.com/std3)
*/

#![no_std]
#![feature(staged_api)]
#![feature(rinuxcore_custom_config)]
#![stable(feature = "rinuxcore", since = "0.1.23")]



#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(std3_reexports)]
#![feature(std3_bootloader)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_attributes)]

#![warn(unused)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]




#[stable(feature = "rinuxcore", since = "0.1.23")]
#[doc(hidden)]
extern crate rinux_macros as pm;
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub use pm::main;

#[stable(feature = "rinuxcore_std3", since = "0.1.23")]
#[macro_use]
pub extern crate std3;
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub use std3::__bootloader::bootloader::BootInfo;
#[stable(feature = "rinuxcore", since = "0.1.23")]
use std3::panic::PanicInfo;
#[stable(feature = "rinuxcore", since = "0.1.23")]
use memory::BootInfoFrameAllocator;
#[unstable(feature = "rinuxcore_custom_config", issue = "none")]
pub mod conf;
#[stable(feature = "rinuxcore", since = "0.1.23")]
extern crate alloc;
#[stable(feature = "rinuxcore", since = "0.1.23")]
#[doc(hidden)]
pub mod allocator;
#[stable(feature = "rinuxcore", since = "0.1.23")]
#[doc(hidden)]
pub mod interrupts;
#[stable(feature = "rinuxcore", since = "0.1.23")]
#[doc(hidden)]
pub mod memory;
#[unstable(feature = "rinuxcore_serial", issue = "none")]
#[doc(hidden)]
pub mod serial;
#[unstable(feature = "rinuxcore_x86_64", issue = "none")]
use std3::__reexports::x86_64;
#[unstable(feature = "rinuxcore_gdt", issue = "none")]
#[doc(hidden)]
pub mod gdt;
#[unstable(feature = "rinuxcore_task", issue = "none")]
pub mod task;

#[unstable(feature = "rinuxcore_enderpearl", issue = "none")]
#[cfg(feature = "epearl")]
pub extern crate epearl;


#[unstable(feature = "rinuxcore_vga_buffer", issue = "none")]
#[allow(unused_imports)]
#[macro_use]
pub extern crate vga_buffer;
#[unstable(feature = "rinuxcore_vga_buffer", issue = "none")]
pub use vga_buffer::{print,println,print_err};


#[stable(feature = "rinuxcore", since = "0.1.23")]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BuildType {
    Debug = 0,
    Release = 1,
}

#[stable(feature = "rinuxcore", since = "0.1.23")]
static mut CONFIGURED: bool = false;
#[stable(feature = "rinuxcore", since = "0.1.23")]
static mut CONFIGTYPE: ConfigType = ConfigType::File;
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub(crate) static mut CONFIG: conf::Config = conf::Config::cnst();
#[stable(feature = "rinuxcore", since = "0.1.23")]
static mut TEST_MODE: BuildType = BuildType::Debug;
#[stable(feature = "rinuxcore", since = "0.1.23")]
static mut VERSION: &'static str = "v1.3.1";
#[stable(feature = "rinuxcore", since = "0.1.23")]
const AUTHORS: &'static str = "Atomic";
#[stable(feature = "rinuxcore", since = "0.1.23")]
const RINUX_ART: &'static str = r#"######   ###  #     #  #     #  #     #
#     #   #   ##    #  #     #   #   #
#     #   #   # #   #  #     #    # #
######    #   #  #  #  #     #     #
#   #     #   #   # #  #     #    # #
#    #    #   #    ##  #     #   #   #
#     #  ###  #     #   #####   #     #
"#;

/// Enum for Configuration Data Specification
#[stable(feature = "rinuxcore", since = "0.1.23")]
#[derive(Debug, Clone, Copy)]
pub enum ConfigType {
    /// File Configuration
    File,
    /// User Configuration
    Custom(conf::Config),
}

/// Used for setting source for rinux to get it's config from
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub fn set_config_type(config_type: ConfigType) {
    if cfg!(pureOS) {
        unsafe {
            CONFIG = conf::Config::new("PureOS",VERSION,false)
        };
    } else {
        match config_type {
            ConfigType::File => {
                unsafe {
                    CONFIG = conf::Config::cnst().get_config(conf::ConfigType::File);
                };
            }
            ConfigType::Custom(data) => unsafe {
                CONFIG = conf::Config::cnst().get_config(conf::ConfigType::UserDefined(data));
            },
        };
    };
    unsafe {
        CONFIGURED = true;
    };
}


#[doc(hidden)]
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub unsafe fn __core_init(){
    if CONFIGURED != true {
        set_config_type(ConfigType::File);
    }

    match CONFIGTYPE {
        ConfigType::Custom(data) => {
            if data.project_name == "" || data.project_version == "" {
                panic!("Please use the enderpearl build system");
            };
        }
        ConfigType::File => {
            if CONFIG.project_name == "" || CONFIG.project_version == "" {
                panic!("Please use the enderpearl build system");
            };
        }
    }

    if cfg!(debug_assertions) {
        TEST_MODE = BuildType::Debug;
    } else {
        TEST_MODE = BuildType::Release;
        VERSION = "v1.3.1-RELEASE";
    }

    vga_buffer::__set_init_rinux(print_init);
}



/// Initializes the std3 of Rinux
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub fn  init(boot_info: &'static BootInfo) {
    unsafe {
        if CONFIGURED != true {
            __core_init()
        }

        use x86_64::VirtAddr;
        gdt::init();
        interrupts::init_idt();
        interrupts::PICS.lock().initialize();
        if CONFIG.quiet_boot != true {
            print_ok!("[OK] Interupts initialized\n");
        };
        x86_64::instructions::interrupts::enable();
        if CONFIG.quiet_boot != true {
            print_ok!("[OK] Instructions initialized\n");
        };

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = {
            if CONFIG.quiet_boot != true {
                print_ok!("[OK] VRAM initialized\n");
            }
            memory::init(phys_mem_offset)
        };
        let mut frame_allocator = BootInfoFrameAllocator::init(&boot_info.memory_map);

        match allocator::init_heap(&mut mapper, &mut frame_allocator) {
            Ok(_) => {
                if CONFIG.quiet_boot != true {
                    print_ok!("[OK] Heap Initialization\n");
                };
            }
            Err(_) => {
                print_err!("[ERR] Heap Initialization\n");
            }
        };
    }

    #[cfg(test)]
    test_main();
}


fn print_init(){
    unsafe {
        vga_buffer::_print_logo(format_args!("{}\n", RINUX_ART));
        if VERSION.ends_with("-RELEASE") {
            if TEST_MODE == BuildType::Debug {
                vga_buffer::_print_logo(format_args!("Rinux Version: {}\n", VERSION));
            } else if TEST_MODE == BuildType::Release {
                vga_buffer::_print_logo(format_args!("Rinux Version: {}\n", VERSION));
            } else {
                panic!("Invalid BuildType");
            }
        } else {
            if TEST_MODE == BuildType::Debug {
                vga_buffer::_print_warn(format_args!("Rinux Version: {}\n", VERSION));
            } else if TEST_MODE == BuildType::Release {
                panic!("Please match VERSION and ENV.BUILD_TYPE");
            } else {
                panic!("Invalid BuildType");
            }
        }

        vga_buffer::_print_logo(format_args!(
            "Rinux Authors: [{}]\nScript: {}\nScript Version: {}\n\n",
            AUTHORS, CONFIG.project_name, CONFIG.project_version
        ));
    }
}

/// Useful for testing
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub trait Testable {
    /// Runs the test
    #[stable(feature = "rinuxcore", since = "0.1.23")]
    fn run(&self) -> ();
}

#[stable(feature = "rinuxcore", since = "0.1.23")]
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", std3::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Runs Tests
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// It's in the name
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Enum For Qemu Exit codes, sometimes useful
#[unstable(feature = "rinuxcore_qemu", issue = "none")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// Everything is fine
    Success = 0x10,
    /// Something went wrong
    Failed = 0x11,
}

/// Quits Qemu using certain exit code
#[unstable(feature = "rinuxcore_qemu", issue = "none")]
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Loop used to just do nothing
#[stable(feature = "rinuxcore", since = "0.1.23")]
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
