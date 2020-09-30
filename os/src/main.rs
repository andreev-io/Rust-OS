#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use os::task::{Task, simple_executor::SimpleExecutor, executor::Executor};
use os::task::keyboard;

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::allocator;
    use os::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("hello world{}", "!");
    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let heap_value = Box::new(41);
    println!("heap value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count: {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("current reference count: {}", Rc::strong_count(&cloned_reference));

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();
}

async fn async_number() -> i32 {
    11
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}