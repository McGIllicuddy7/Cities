package main
import "core:math"
import "gc"
import "utils"

vec2i :: [2]i32
vec2f :: [2]f32
color_t :: [4]u8

Building :: struct {
	center:        vec2i,
	width, height: i32,
	rotation:      f32,
	description:   string,
}

City :: struct {
	buildings: [dynamic]Building,
}
