package utils

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

GlobalGcData :: struct {
	is_init:           bool,
	medium_term_arena: mem.Dynamic_Arena,
}


gc_alloc :: proc() -> mem.Allocator {
	if (!thread_allocator.is_init) {
		mem.dynamic_arena_init(get_short_term_arena())
		mem.dynamic_arena_init(get_medium_term_arena())
		mem.dynamic_arena_init(get_long_term_arena())
		thread_allocator.is_init = true
	}
	return mem.dynamic_arena_allocator(get_short_term_arena())
}

gc_mt_alloc :: proc() -> mem.Allocator {
	if (!thread_allocator.is_init) {
		mem.dynamic_arena_init(get_short_term_arena())
		mem.dynamic_arena_init(get_medium_term_arena())
		mem.dynamic_arena_init(get_long_term_arena())
		thread_allocator.is_init = true
	}
	return mem.dynamic_arena_allocator(get_medium_term_arena())
}

gc_lt_alloc :: proc() -> mem.Allocator {
	if (!thread_allocator.is_init) {
		mem.dynamic_arena_init(get_short_term_arena())
		mem.dynamic_arena_init(get_medium_term_arena())
		mem.dynamic_arena_init(get_long_term_arena())
		thread_allocator.is_init = true
	}
	return mem.dynamic_arena_allocator(get_long_term_arena())
}

gc_collect :: proc(location := #caller_location) {
	mem.dynamic_arena_free_all(get_short_term_arena(), location)
}

gc_mt_collect :: proc(location := #caller_location) {
	mem.dynamic_arena_free_all(get_short_term_arena(), location)
	mem.dynamic_arena_free_all(get_medium_term_arena(), location)
}


gc_lt_collect :: proc(location := #caller_location) {
	mem.dynamic_arena_free_all(get_short_term_arena(), location)
	mem.dynamic_arena_free_all(get_medium_term_arena(), location)
	mem.dynamic_arena_free_all(get_long_term_arena(), location)
}


gc_on_thread_end :: proc() {
	assert(thread_allocator.is_init)
	mem.dynamic_arena_destroy(&thread_allocator.short_term_arena)
	mem.dynamic_arena_destroy(&thread_allocator.medium_term_arena)
	mem.dynamic_arena_destroy(&thread_allocator.long_term_arena)
	thread_allocator.is_init = false
}

gc_shallow_clone_ptr :: proc(
	v: ^$T,
	allocator: mem.Allocator,
	location := #caller_location,
) -> ^T {
	out := new_clone(v, allocator, location)
	return out
}

gc_shallow_clone_slice :: proc(v: []$T, alloc: mem.Allocator) -> []T {
	out := make_slice([]T, len(v), alloc)
	for i, idx in v {
		out[idx] = i
	}
	return out
}

gc_shallow_clone_slice_ptrs :: proc(v: []^$T, alloc: mem.Allocator) -> []^T {
	out := make_slice([]T, len(v), alloc)
	for i, idx in v {
		out[idx] = gc_shallow_clone_ptr(i, alloc)
	}
	return out
}

gc_shallow_clone_string :: proc(v: string, alloc: mem.Allocator) -> string {
	out := make_slice([]u8, len(v), alloc)
	for i in 0 ..< len(v) {
		out[i] = v[i]
	}
	return cast(string)out
}

gc_shallow_clone_slice_of_strings :: proc(v: []string, alloc: mem.Allocator) -> []string {
	out := make_slice([]string, len(v), alloc)
	for i, idx in v {
		out[idx] = gc_shallow_clone_string(i, alloc)
	}
	return out
}

gc_shallow_clone :: proc {
	gc_shallow_clone_ptr,
	gc_shallow_clone_slice,
	gc_shallow_clone_slice_ptrs,
	gc_shallow_clone_string,
	gc_shallow_clone_slice_of_strings,
}

did_arena_alloc_pointer :: proc(ptr: ^$T, arena: ^mem.Dynamic_Arena) -> bool {
	if ptr == nil {
		return false
	}
	if cast(uintptr)arena.current_block < cast(uintptr)ptr &&
	   cast(uintptr)arena.current_block + cast(uintptr)arena.block_size > cast(uintptr)ptr {
		return true
	}
	for i in arena.used_blocks {
		if cast(uintptr)i < cast(uintptr)ptr &&
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

gc_extend_ptr_lifetime :: proc(ptr: ^$T) -> ^T {
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

gc_extend_slice_lifetime :: proc(ptr: []$T) -> []T {
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

gc_extend_string_lifetime :: proc(ptr: string) -> string {
	sts := transmute([]u8)(ptr)
	assert(thread_allocator.is_init)
	if did_arena_alloc_pointer(&sts[0], get_short_term_arena()) {
		return gc_shallow_clone(ptr, gc_mt_alloc())
	}
	if did_arena_alloc_pointer(&sts[0], get_medium_term_arena()) {
		return gc_shallow_clone(ptr, gc_lt_alloc())
	}
	if did_arena_alloc_pointer(&sts[0], get_long_term_arena()) {
		return gc_shallow_clone(ptr, context.allocator)
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
