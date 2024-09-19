#![cfg_attr(not(test), no_std)]

pub mod console;
pub mod drivers;
pub mod memory;
pub mod riscv;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod trap;

pub fn kernel_main(hart_id: usize, dtb: usize) -> ! {
    console::init();

    println!("RustV OS Lab");
    println!("boot hart: {hart_id}");
    println!("device tree pointer: 0x{dtb:016x}");
    println!("uart: ready");
    println!("memory: page allocator and Sv39 model loaded");
    println!("trap: cause decoder loaded");
    println!("task: cooperative scheduler model loaded");

    loop {
        riscv::wfi();
    }
}

#[cfg(test)]
mod tests {
    use super::memory::page_allocator::{PageAllocator, PageDescriptor, PAGE_SIZE};
    use super::memory::sv39::{PteFlags, Sv39Address};
    use super::task::{Scheduler, Task};

    #[test]
    fn allocator_finds_contiguous_pages() {
        let mut pages = [PageDescriptor::empty(); 8];
        let mut allocator = PageAllocator::new(0x8000_0000, &mut pages);
        let addr = allocator.alloc_contiguous(3).unwrap();
        assert_eq!(addr, 0x8000_0000);
        assert_eq!(allocator.free_pages(), 5);
        allocator.dealloc(addr).unwrap();
        assert_eq!(allocator.free_pages(), 8);
        assert_eq!(PAGE_SIZE, 4096);
    }

    #[test]
    fn sv39_breaks_virtual_address_into_indices() {
        let addr = Sv39Address::new(0x7d_beef_cafe);
        assert_eq!(addr.vpn(), [0xfc, 0x1f7, 0x1f6]);
        assert_eq!(addr.offset(), 0xafe);
    }

    #[test]
    fn scheduler_round_robins_runnable_tasks() {
        let mut scheduler: Scheduler<4> = Scheduler::new();
        scheduler.add(Task::new(1, 0x1000, 0x2000)).unwrap();
        scheduler.add(Task::new(2, 0x1100, 0x2100)).unwrap();
        assert_eq!(scheduler.next_runnable().map(|task| task.id), Some(1));
        assert_eq!(scheduler.next_runnable().map(|task| task.id), Some(2));
        assert!(PteFlags::READ.union(PteFlags::WRITE).contains(PteFlags::READ));
    }
}

