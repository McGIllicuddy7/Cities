package main
import "core:math"
import "gc"


vec2i :: [2]i32
vec2f :: [2]f32
color :: [4]u8
Building :: struct {
	center:        vec2i,
	width, height: i32,
	rotation:      f32,
	description:   string,
}

City :: struct {
	buildings: [dynamic]Building,
}

point_nearest_point_on_line :: proc(point, start, end: vec2i) -> (f32, vec2i) {
	p := cast(vec2f)point
	s := cast(vec2f)start
	e := cast(vec2f)end
	guess := (e - s)
	min := vec2f_dist(guess, p)
	delta := (e - s) / 2
	epsilon := vec2f_len(delta)
	for epsilon > 0.01 {
		ms := guess + delta
		me := guess - delta
		ds := vec2f_dist(ms, p)
		de := vec2f_dist(me, p)
		if ds < min {
			min = ds
			guess = ms
		}
		if de < min {
			min = de
			guess = me
		}
		delta /= 2
		epsilon = vec2f_len(delta)
	}
	ms := vec2f_dist(p, s)
	if ms < min {
		min = ms
		guess = s
	}
	me := vec2f_dist(p, e)
	if me < min {
		min = me
		guess = e
	}
	return me, cast([2]i32)guess
}

vec2f_dist :: proc(a, b: vec2f) -> f32 {
	return math.sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y))
}

vec2f_len :: proc(v: vec2f) -> f32 {
	return math.sqrt(v.x * v.x + v.y + v.y)
}
