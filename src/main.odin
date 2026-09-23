package main
import "core:encoding/json"
import "core:fmt"
import "core:mem"
import "core:thread"
import "threadpool"
import "utils"
Test :: struct {
	name:   string,
	number: i32,
}

main :: proc() {
	when ODIN_DEBUG {
		track: mem.Tracking_Allocator
		mem.tracking_allocator_init(&track, context.allocator)
		context.allocator = mem.tracking_allocator(&track)
		defer {
			if len(track.allocation_map) > 0 {
				fmt.eprintf("=== %v allocations not freed: ===\n", len(track.allocation_map))
				for _, entry in track.allocation_map {
					fmt.eprintf("- %v bytes @ %v\n", entry.size, entry.location)
				}
			}
			mem.tracking_allocator_destroy(&track)
		}
	}
	main_act()
}
is_prime :: proc(i: int) -> bool {
	for j in 2 ..< i {
		if i % j == 0 {
			return false
		}
	}
	return true
}
main_act :: proc() {
	threadpool.thread_pool_init()
	defer threadpool.thread_pool_fini()
	{
		ThreadData :: struct {
			start: int,
		}
		thread_func :: proc(data: ^ThreadData) {
			for i in data.start ..< data.start * 100 {
				if is_prime(i) {
					fmt.println(data.start, i)
				}
				for _ in 0 ..< 100 {
					thread.yield()
				}
			}
		}
		group := threadpool.task_group_create()
		for i in 1 ..= 100 {
			threadpool.task_group_spawn_task(group, ThreadData{start = i}, thread_func)
		}
		threadpool.task_group_await(group)
	}
}
