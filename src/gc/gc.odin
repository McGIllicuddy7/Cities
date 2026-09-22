package gc

import "core:fmt"
import "core:mem"

GcData :: struct {
	is_init:           bool,
	short_term_arena:  mem.Dynamic_Arena,
	medium_term_arena: mem.Dynamic_Arena,
	long_term_arena:   mem.Dynamic_Arena,
}

@(thread_local)
@(private)
thread_allocator: GcData


alloc :: proc() -> mem.Allocator {
	if (!thread_allocator.is_init) {
		mem.dynamic_arena_init(get_short_term_arena())
		mem.dynamic_arena_init(get_medium_term_arena())
		mem.dynamic_arena_init(get_long_term_arena())
		thread_allocator.is_init = true
	}
	return mem.dynamic_arena_allocator(get_short_term_arena())
}

mt_alloc :: proc() -> mem.Allocator {
	if (!thread_allocator.is_init) {
		mem.dynamic_arena_init(get_short_term_arena())
		mem.dynamic_arena_init(get_medium_term_arena())
		mem.dynamic_arena_init(get_long_term_arena())
		thread_allocator.is_init = true
	}
	return mem.dynamic_arena_allocator(get_medium_term_arena())
}

lt_alloc :: proc() -> mem.Allocator {
	if (!thread_allocator.is_init) {
		mem.dynamic_arena_init(get_short_term_arena())
		mem.dynamic_arena_init(get_medium_term_arena())
		mem.dynamic_arena_init(get_long_term_arena())
		thread_allocator.is_init = true
	}
	return mem.dynamic_arena_allocator(get_long_term_arena())
}

collect :: proc(location := #caller_location) {
	mem.dynamic_arena_free_all(get_short_term_arena(), location)
}

mt_collect :: proc(location := #caller_location) {
	mem.dynamic_arena_free_all(get_short_term_arena(), location)
	mem.dynamic_arena_free_all(get_medium_term_arena(), location)
}


lt_collect :: proc(location := #caller_location) {
	mem.dynamic_arena_free_all(get_short_term_arena(), location)
	mem.dynamic_arena_free_all(get_medium_term_arena(), location)
	mem.dynamic_arena_free_all(get_long_term_arena(), location)
}

collect_reachable :: proc(ptr: rawptr, dbg := false, location := #caller_location) {
	if did_arena_alloc_pointer(ptr, get_short_term_arena()) {
		if dbg {
			fmt.printf("gc freed nothing\n")
		}
		return
	}
	if did_arena_alloc_pointer(ptr, get_medium_term_arena()) {
		if dbg {
			fmt.printf("gc freed nursery\n")
		}
		collect(location)
		return
	}
	if did_arena_alloc_pointer(ptr, get_long_term_arena()) {
		if dbg {
			fmt.printf("gc freed short and medium term\n")
		}
		mt_collect(location)
		return
	}
	if dbg {
		fmt.printf("gc freed all")
	}
	lt_collect(location)
}

on_thread_end :: proc() {
	assert(thread_allocator.is_init)
	mem.dynamic_arena_destroy(&thread_allocator.short_term_arena)
	mem.dynamic_arena_destroy(&thread_allocator.medium_term_arena)
	mem.dynamic_arena_destroy(&thread_allocator.long_term_arena)
	thread_allocator.is_init = false
}

shallow_clone_ptr :: proc(v: ^$T, allocator: mem.Allocator, location := #caller_location) -> ^T {
	out := new_clone(v, allocator, location)
	return out
}

shallow_clone_slice :: proc(v: []$T, alloc: mem.Allocator) -> []T {
	out := make_slice([]T, len(v), alloc)
	for i, idx in v {
		out[idx] = i
	}
	return out
}

shallow_clone_slice_ptrs :: proc(v: []^$T, alloc: mem.Allocator) -> []^T {
	out := make_slice([]T, len(v), alloc)
	for i, idx in v {
		out[idx] = gc_shallow_clone_ptr(i, alloc)
	}
	return out
}

shallow_clone_string :: proc(v: string, alloc: mem.Allocator) -> string {
	out := make_slice([]u8, len(v), alloc)
	for i in 0 ..< len(v) {
		out[i] = v[i]
	}
	return cast(string)out
}

shallow_clone_slice_of_strings :: proc(v: []string, alloc: mem.Allocator) -> []string {
	out := make_slice([]string, len(v), alloc)
	for i, idx in v {
		out[idx] = shallow_clone_string(i, alloc)
	}
	return out
}

shallow_clone :: proc {
	shallow_clone_ptr,
	shallow_clone_slice,
	shallow_clone_slice_ptrs,
	shallow_clone_string,
	shallow_clone_slice_of_strings,
}

did_arena_alloc_pointer :: proc(ptr: rawptr, arena: ^mem.Dynamic_Arena) -> bool {
	if ptr == nil {
		return false
	}
	if cast(uintptr)arena.current_block <= cast(uintptr)ptr &&
	   cast(uintptr)arena.current_block + cast(uintptr)arena.block_size > cast(uintptr)ptr {
		return true
	}
	for i in arena.used_blocks {
		if cast(uintptr)i <= cast(uintptr)ptr &&
		   cast(uintptr)i + cast(uintptr)arena.block_size > cast(uintptr)ptr {
			return true
		}
	}
	for i in arena.out_band_allocations {
		if cast(uintptr)i == cast(uintptr)ptr {
			return true
		}
	}
	return false
}

extend_ptr_lifetime :: proc(ptr: ^$T) -> ^T {
	assert(thread_allocator.is_init)
	if did_arena_alloc_pointer(ptr, get_short_term_arena()) {
		return gc_shallow_clone(ptr, gc_mt_alloc())
	}
	if did_arena_alloc_pointer(ptr, get_medium_term_arena()) {
		return gc_shallow_clone(ptr, gc_lt_alloc())
	}
	if did_arena_alloc_pointer(ptr, get_long_term_arena()) {
		return gc_shallow_clone(ptr, context.allocator)
	}
	return ptr
}

extend_slice_lifetime :: proc(ptr: []$T) -> []T {
	assert(thread_allocator.is_init)
	if did_arena_alloc_pointer(&ptr[0], get_short_term_arena()) {
		return gc_shallow_clone(ptr, gc_mt_alloc())
	}
	if did_arena_alloc_pointer(&ptr[0], get_medium_term_arena()) {
		return gc_shallow_clone(ptr, gc_lt_alloc())
	}
	if did_arena_alloc_pointer(&ptr[0], get_long_term_arena()) {
		return gc_shallow_clone(ptr, context.allocator)
	}
	return ptr
}

extend_string_lifetime :: proc(ptr: string) -> string {
	sts := transmute([]u8)(ptr)
	assert(thread_allocator.is_init)
	if did_arena_alloc_pointer(&sts[0], get_short_term_arena()) {
		return shallow_clone(ptr, mt_alloc())
	}
	if did_arena_alloc_pointer(&sts[0], get_medium_term_arena()) {
		return shallow_clone(ptr, lt_alloc())
	}
	if did_arena_alloc_pointer(&sts[0], get_long_term_arena()) {
		return shallow_clone(ptr, context.allocator)
	}
	return ptr
}

get_short_term_arena :: proc() -> ^mem.Dynamic_Arena {
	return &thread_allocator.short_term_arena
}

get_medium_term_arena :: proc() -> ^mem.Dynamic_Arena {
	return &thread_allocator.medium_term_arena
}

get_long_term_arena :: proc() -> ^mem.Dynamic_Arena {
	return &thread_allocator.long_term_arena
}
