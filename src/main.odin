package main
import "core:encoding/json"
import "core:fmt"
import "core:mem"

import "gc"
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

main_act :: proc() {
	img := utils.image_create({255, 0, 255, 255}, 1000, 1000)
	utils.image_draw_text(img, "hello there toast i love you", 20, 20, 32, {255, 255, 255, 255})
	utils.image_render_out(img, "test.png")
	utils.image_destroy(img)
}
