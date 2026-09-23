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
	defer utils.deinit_font()
	img := utils.image_create({255, 0, 255, 255}, 1000, 1000)
	p0 := utils.vec2i{utils.random_in_range(0, 200), utils.random_in_range(800, 1000)}
	p1 := utils.vec2i{utils.random_in_range(400, 600), utils.random_in_range(0, 200)}
	p2 := utils.vec2i{utils.random_in_range(800, 1000), utils.random_in_range(800, 1000)}
	utils.image_draw_triangle(img, p0, p1, p2, {255, 0, 0, 255})
	utils.image_draw_circle(img, p0.x, p0.y, 5., {0, 255, 0, 255})
	utils.image_draw_circle(img, p1.x, p1.y, 5., {0, 255, 0, 255})
	utils.image_draw_circle(img, p2.x, p2.y, 5., {0, 255, 0, 255})
	//utils.image_draw_text(img, "hello there toast i love you", 20, 20, 32, {255, 255, 255, 255})
	//utils.image_draw_line(img, 200, 200, 800, 800, {255, 0, 0, 255})
	utils.image_render_out(img, "test.png")
	utils.image_destroy(img)
}
