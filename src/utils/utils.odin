package utils
import "core:fmt"
import "core:math"
import "core:math/rand"
import "core:mem"
import "core:os"
import "core:strings"
import "core:sync"
import "vendor:raylib"
import "vendor:stb/image"
import "vendor:stb/truetype"

vec2i :: [2]i32
vec2f :: [2]f32
color_t :: [4]u8

BoundingBox :: struct {
	x, y, width, height: i32,
}

RectBounds :: struct {
	center_x, center_y, width, height: i32,
	rot:                               f32,
}

random :: proc() -> u32 {
	return rand.uint32()
}

random_in_range :: proc(min: i32, max: i32) -> i32 {
	assert(min < max)
	dist := max - min
	tmp := random() % cast(u32)dist
	out := cast(i32)tmp + min
	fmt.println("out:", out)
	return out
}

random_bool :: proc(chance: f64) -> bool {
	assert(chance >= 0)
	assert(chance <= 1)
	return rand.norm_float64() < chance
}

Image :: struct {
	pixels:    []color_t,
	height:    i32,
	width:     i32,
	allocator: mem.Allocator,
}

image_draw_pixel :: proc(img: ^Image, x, y: i32, color: color_t) {
	if x < 0 || x >= img.width || y < 0 || y >= img.height {
		return
	}
	prev := cast([4]f32)img.pixels[y * img.width + x]
	c := cast([4]f32)color
	a := c.a / 255.
	tmp := (1 - a) * prev + a * c
	alpha := max(prev.a, c.a)
	tmp.a = alpha
	img.pixels[y * img.width + x] = cast(color_t)tmp
}

image_set_pixel :: proc(img: ^Image, x, y: i32, color: color_t) {
	if x < 0 || x >= img.width || y < 0 || y >= img.height {
		return
	}
	img.pixels[y * img.width + x] = color
}


image_get_pixel :: proc(img: ^Image, x, y: i32) -> color_t {
	if x < 0 || x >= img.width || y < 0 || y >= img.height {
		return {0, 0, 0, 0}
	}
	return img.pixels[y * img.width + x]
}

image_render_out :: proc(img: ^Image, path: string) {
	name := strings.clone_to_cstring(path)
	defer delete(name)
	if strings.ends_with(path, ".png") {
		image.write_png(name, img.width, img.height, 4, &img.pixels[0], img.width * size_of(i32))
	} else if strings.ends_with(path, ".jpg") || strings.ends_with(path, ".jpeg") {
		image.write_jpg(name, img.width, img.height, 4, &img.pixels[0], img.width * size_of(i32))
	} else if strings.ends_with(path, ".bmp") {
		image.write_bmp(name, img.width, img.height, 4, &img.pixels[0])

	} else {
		fmt.eprintf("error unsupported file format:%s\n", path)
		return
	}
}

image_create :: proc(
	color: color_t,
	width: i32,
	height: i32,
	allocator := context.allocator,
) -> ^Image {
	data := make_slice([]color_t, width * height, allocator)
	for &i in data {
		i = color
	}
	return new_clone(Image{data, width, height, allocator}, allocator)
}

image_clone :: proc(img: ^Image, allocator := context.allocator) -> ^Image {
	out := new(Image, allocator)
	out.width = img.width
	out.height = img.height
	out.pixels = make_slice([]color_t, img.height * img.width, allocator)
	copy_slice(out.pixels, img.pixels)
	return out
}

image_destroy :: proc(img: ^Image) {
	delete(img.pixels, img.allocator)
	free(img, img.allocator)
}

image_draw_rect :: proc(img: ^Image, x, y, width, height: i32, color: color_t) {
	for dy in y ..< y + height {
		for dx in x ..< x + width {
			image_draw_pixel(img, dy, dx, color)
		}
	}
}

image_draw_circle :: proc(img: ^Image, x, y: i32, radius: f32, color: color_t) {
	min_x := x - cast(i32)math.ceil(radius)
	min_y := y - cast(i32)math.ceil(radius)
	max_x := x + cast(i32)math.ceil(radius)
	max_y := y + cast(i32)math.ceil(radius)
	rad := cast(i32)(radius * radius)
	for dy in min_y ..= max_y {
		for dx in min_x ..= max_x {
			dx0 := (dx - x)
			dy0 := (dy - y)
			dst := dx0 * dx0 + dy0 * dy0
			if dst < rad {
				image_draw_pixel(img, dx, dy, color)
			}
		}
	}
}

image_draw_line :: proc(img: ^Image, x0, y0, x1, y1: i32, color: color_t) {
	bounds := bounding_box_of_points({{x0, y0}, {x1, y1}})
	for dy in bounds.y ..< bounds.y + bounds.height {
		for dx in bounds.x ..< bounds.x + bounds.width {
			dist := point_distance_to_line({dx, dy}, {x0, y0}, {x1, y1})
			if dist < 0.8 {
				image_draw_pixel(img, dx, dy, color)
			}
		}
	}
}

image_draw_line_v :: proc(img: ^Image, start: vec2i, end: vec2i, color: color_t) {
	bounds := bounding_box_of_points({start, end})
	for dy in bounds.y ..< bounds.y + bounds.height {
		for dx in bounds.x ..< bounds.x + bounds.width {
			dist := point_distance_to_line({dx, dy}, start, end)
			if dist < 0.8 {
				image_draw_pixel(img, dx, dy, color)
			}
		}
	}
}

image_draw_line_w :: proc(img: ^Image, x0, y0, x1, y1: i32, width: f32, color: color_t) {
	bounds := bounding_box_of_points({{x0, y0}, {x1, y1}})
	for dy in bounds.y ..< bounds.y + bounds.height {
		for dx in bounds.x ..< bounds.x + bounds.width {
			dist := point_distance_to_line({dx, dy}, {x0, y0}, {x1, y1})
			if dist < width {
				image_draw_pixel(img, dx, dy, color)
			}
		}
	}
}


image_draw_line_vw :: proc(img: ^Image, start: vec2i, end: vec2i, width: f32, color: color_t) {
	bounds := bounding_box_of_points({start, end})
	for dy in bounds.y ..< bounds.y + bounds.height {
		for dx in bounds.x ..< bounds.x + bounds.width {
			dist := point_distance_to_line({dx, dy}, start, end)
			if dist < width {
				image_draw_pixel(img, dx, dy, color)
			}
		}
	}
}

image_draw_triangle :: proc(img: ^Image, p0, p1, p2: vec2i, color: color_t) {
	bounds := bounding_box_of_points({p0, p1, p2})
	/*	bounds.x = 0
	bounds.y = 0
	bounds.width = img.width
	bounds.height = img.height*/
	for dy in bounds.y ..< bounds.y + bounds.height {
		for dx in bounds.x ..< bounds.x + bounds.width {
			if check_collision_triangle_point({dx, dy}, p0, p1, p2) {
				image_draw_pixel(img, dx, dy, color)
			}
		}
	}
}

image_draw_text :: proc(img: ^Image, text: string, x, y, text_height: i32, color: color_t) {
	init_font()
	scale := truetype.ScaleForPixelHeight(FONT, cast(f32)text_height) * 100.
	fmt.printf("scale:%f\n", scale)
	x1 := x
	y1 := y
	prev: rune = 0
	for i in text {
		if i == '\n' {
			x1 = x
			y1 += text_height
			prev = 0
		} else {
			if prev != 0 {
				adv: i32 = cast(i32)(cast(f32)9 * scale)
				x1 += adv
				//fmt.printf("char:%c, advance:%d\n", i, adv)
			}
			image_draw_codepoint(img, i, x1, y1, text_height, color)
			prev = i
		}
	}
}

image_draw_codepoint :: proc(
	img: ^Image,
	codepoint: rune,
	x: i32,
	y: i32,
	text_height: i32,
	color: color_t,
) {
	init_font()
	scale := truetype.ScaleForPixelHeight(FONT, cast(f32)text_height)
	width, height, x_off, y_off: i32
	bmp := truetype.GetCodepointBitmap(
		FONT,
		scale,
		scale,
		codepoint,
		&width,
		&height,
		&x_off,
		&y_off,
	)
	defer truetype.FreeBitmap(bmp, nil)
	for dy in 0 ..< height {
		for dx in 0 ..< width {
			x1 := dx + x + x_off
			y1 := dy + y + y_off
			c := bmp[dy * width + dx]
			v := cast([4]f32)color
			v *= cast(f32)c / (255.)
			c2 := cast([4]u8)v
			image_draw_pixel(img, x1, y1, c2)
		}
	}
}

FONT_GUARD: sync.Mutex
FONT: ^truetype.fontinfo = nil
BITMAP: []u8 = nil
FONT_DATA: [^]u8 = nil

init_font :: proc() {
	sync.mutex_lock(&FONT_GUARD)
	defer sync.mutex_unlock(&FONT_GUARD)
	if FONT != nil {
		return
	}
	FONT = new(truetype.fontinfo)
	bites, err := os.read_entire_file_from_path("Ac437_IBM_VGA_9x16.ttf", context.allocator)
	assert(err == nil)
	FONT_DATA = &bites[0]
	truetype.InitFont(FONT, FONT_DATA, truetype.GetFontOffsetForIndex(FONT_DATA, 0))
}


deinit_font :: proc() {
	sync.mutex_lock(&FONT_GUARD)
	defer sync.mutex_unlock(&FONT_GUARD)
	delete(BITMAP)
	free(FONT)
	free(FONT_DATA)
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
	return min, cast([2]i32)guess
}


vec2f_dist :: proc(a, b: vec2f) -> f32 {
	return math.sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y))
}

vec2f_len :: proc(v: vec2f) -> f32 {
	return math.sqrt(v.x * v.x + v.y * v.y)
}

vec2f_rotate :: proc(v: vec2f, angle: f32) -> vec2f {
	len := vec2f_len(v)
	if len == 0.0 {
		return v
	}
	base_angle := math.atan2(v.y / len, v.x / len)
	out_angle := base_angle + angle
	out := vec2f{math.cos(out_angle) * len, math.sin(out_angle) * len}
	return out
}

vec2f_angle :: proc(v: vec2f) -> f32 {
	len := vec2f_len(v)
	if len == 0.0 {
		return 0.0
	}
	base_angle := math.atan2(v.y / len, v.x / len)
	return base_angle
}

vec2f_angle_between :: proc(a: vec2f, b: vec2f) -> f32 {
	an := vec2f_normalized(a)
	bn := vec2f_normalized(b)
	dot := vec2f_dot(an, bn)
	return math.acos(dot)
}

vec2f_normalized :: proc(v: vec2f) -> vec2f {
	len := vec2f_len(v)
	return v / len
}

vec2f_dot :: proc(a: vec2f, b: vec2f) -> f32 {
	return a.x * b.x + a.y * b.y
}


point_distance_to_line :: proc(point, start, end: vec2i) -> f32 {
	dist, _ := point_nearest_point_on_line(point, start, end)
	return dist
}

bounding_box_of_points :: proc(args: []vec2i) -> BoundingBox {
	out: BoundingBox
	if (len(args) < 0) {
		return out
	}
	min_x := args[0].x
	max_x := args[0].x
	min_y := args[0].y
	max_y := args[0].y
	for i in args {
		if i.x < min_x {
			min_x = i.x
		}
		if i.x > max_x {
			max_x = i.x
		}

		if i.y < min_y {
			min_y = i.y
		}
		if i.y > max_y {
			max_y = i.y
		}
	}
	out.x = min_x
	out.y = min_y
	out.width = max_x - min_x + 1
	out.height = max_y - min_y + 1
	return out
}

check_collision_triangle_point :: proc(point: vec2i, p0: vec2i, p1: vec2i, p2: vec2i) -> bool {
	tri := [3]vec2f{cast(vec2f)p0, cast(vec2f)p1, cast(vec2f)p2}
	center := (tri[0] + tri[1] + tri[2]) / 3
	for di in 0 ..< 3 {
		v0 := tri[di]
		v1 := tri[(di + 1) % 3]
		mid := (v0 + v1) / 2
		n := vec2f_rotate(vec2f_normalized(v1 - v0), math.PI / 2)

		delta := mid - center
		if vec2f_dot(n, delta) < 0.0 {
			n *= -1
		}
		d2 := cast(vec2f)point - mid
		if vec2f_dot(d2, n) > 0.0 {
			return false
		}
	}
	return true
}


rotate_point_set :: proc(points: []vec2i, angle: f32) {
	center: vec2i
	for i in points {
		center += i
	}
	center /= cast(i32)len(points)
	for &i in points {
		delta := i - center
		d2 := cast(vec2f)delta
		d3 := vec2f_rotate(d2, angle)
		d4 := cast(vec2i)d3
		i = center + d4
	}
}

rotate_point_set_about :: proc(points: []vec2i, angle: f32, center: vec2i) {
	for &i in points {
		delta := i - center
		d2 := cast(vec2f)delta
		d3 := vec2f_rotate(d2, angle)
		d4 := cast(vec2i)d3
		i = center + d4
	}
}

distance_between_lines :: proc(s1: vec2i, e1: vec2i, s2: vec2i, e2: vec2i) -> f32 {
	con: vec2f
	if raylib.CheckCollisionLines(
		cast(vec2f)s1,
		cast(vec2f)e1,
		cast(vec2f)s2,
		cast(vec2f)e2,
		&con,
	) {
		return 0.0
	}
	min := vec2f_dist(cast(vec2f)s1, cast(vec2f)e1)
	d1 := point_distance_to_line(s1, s2, e2)
	if d1 < min {
		min = d1
	}
	d2 := point_distance_to_line(e1, s2, e2)
	if d2 < min {
		min = d2
	}
	d3 := point_distance_to_line(s2, s1, e1)
	if d3 < min {
		min = d3
	}
	d4 := point_distance_to_line(e2, s1, e1)
	if d4 < min {
		min = d4
	}
	return min
}

rect_bounds_vertices :: proc(a: RectBounds) -> [4]vec2i {
	base := [4]vec2i {
		{a.center_x - a.width / 2, a.center_y - a.height / 2},
		{a.center_x - a.width / 2, a.center_y + a.height / 2},
		{a.center_x + a.width / 2, a.center_y + a.height / 2},
		{a.center_x + a.width / 2, a.center_y - a.height / 2},
	}
	rotate_point_set_about(base[:], a.rot, {a.center_x, a.center_y})
	return base
}

rect_bounds_normals :: proc(a: RectBounds) -> [4]vec2f {
	out := [4]vec2f{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for &i in out {
		i = vec2f_rotate(i, a.rot)
	}
	return out
}
rect_bounds_corner_directions :: proc(a: RectBounds) -> [4]vec2f {
	out := [4]vec2f{{-1, 1}, {-1, -1}, {1, -1}, {1, 1}}
	for &i in out {
		i = vec2f_normalized(vec2f_rotate(i, a.rot))
	}
	return out
}

seperating_axis_theorem_collision_check :: proc(
	a_verts: []vec2f,
	a_norms: []vec2f,
	a_corns: []vec2f,
	b_verts: []vec2f,
	b_norms: []vec2f,
	b_corns: []vec2f,
) -> bool {
	for i in a_norms {
		a_min := vec2f_dot(a_verts[0], i)
		a_max := a_min
		b_min := vec2f_dot(b_verts[0], i)
		b_max := b_min
		for j in a_verts {
			tmp := vec2f_dot(j, i)
			if tmp < a_min {
				a_min = tmp
			}
			if tmp > a_max {
				a_max = tmp
			}
		}
		for j in b_verts {
			tmp := vec2f_dot(j, i)
			if tmp < b_min {
				b_min = tmp
			}
			if tmp > b_max {
				b_max = tmp
			}
		}
		if a_max < b_min || b_max < a_min {
			return false
		}
	}
	for i in b_norms {
		a_min := vec2f_dot(a_verts[0], i)
		a_max := a_min
		b_min := vec2f_dot(b_verts[0], i)
		b_max := b_min
		for j in a_verts {
			tmp := vec2f_dot(j, i)
			if tmp < a_min {
				a_min = tmp
			}
			if tmp > a_max {
				a_max = tmp
			}
		}
		for j in b_verts {
			tmp := vec2f_dot(j, i)
			if tmp < b_min {
				b_min = tmp
			}
			if tmp > b_max {
				b_max = tmp
			}
		}
		if a_max < b_min || b_max < a_min {
			return false
		}
	}
	for i in a_corns {
		a_min := vec2f_dot(a_verts[0], i)
		a_max := a_min
		b_min := vec2f_dot(b_verts[0], i)
		b_max := b_min
		for j in a_verts {
			tmp := vec2f_dot(j, i)
			if tmp < a_min {
				a_min = tmp
			}
			if tmp > a_max {
				a_max = tmp
			}
		}
		for j in b_verts {
			tmp := vec2f_dot(j, i)
			if tmp < b_min {
				b_min = tmp
			}
			if tmp > b_max {
				b_max = tmp
			}
		}
		if a_max < b_min || b_max < a_min {
			return false
		}
	}
	for i in b_corns {
		a_min := vec2f_dot(a_verts[0], i)
		a_max := a_min
		b_min := vec2f_dot(b_verts[0], i)
		b_max := b_min
		for j in a_verts {
			tmp := vec2f_dot(j, i)
			if tmp < a_min {
				a_min = tmp
			}
			if tmp > a_max {
				a_max = tmp
			}
		}
		for j in b_verts {
			tmp := vec2f_dot(j, i)
			if tmp < b_min {
				b_min = tmp
			}
			if tmp > b_max {
				b_max = tmp
			}
		}
		if a_max < b_min || b_max < a_min {
			return false
		}
	}
	return true
}

check_collision_bounds :: proc(a: RectBounds, b: RectBounds) -> bool {
	a_verts := cast([4]vec2f)rect_bounds_vertices(a)
	b_verts := cast([4]vec2f)rect_bounds_vertices(b)
	a_norms := rect_bounds_normals(a)
	b_norms := rect_bounds_normals(b)
	a_corns := rect_bounds_corner_directions(a)
	b_corns := rect_bounds_corner_directions(b)
	return seperating_axis_theorem_collision_check(
		a_verts[:],
		a_norms[:],
		a_corns[:],
		b_verts[:],
		b_norms[:],
		b_corns[:],
	)
}

distance_between_bounds :: proc(a: RectBounds, b: RectBounds) -> f32 {
	out: f32 = vec2f_dist(
		cast(vec2f)vec2i{a.center_x, a.center_y},
		cast(vec2f)vec2i{b.center_x, b.center_y},
	)
	if check_collision_bounds(a, b) {
		return 0
	}
	a_points := rect_bounds_vertices(a)
	b_points := rect_bounds_vertices(b)
	for i in 0 ..< len(a_points) {
		s1 := a_points[i]
		e1 := a_points[(i + 1) % len(a_points)]
		for j in 0 ..< len(b_points) {
			s2 := b_points[j]
			e2 := b_points[(j + 1) % len(b_points)]
			d := distance_between_lines(s1, e1, s2, e2)
			if d < out {
				out = d
			}
		}
	}
	return out
}

bounds_contains_point :: proc(bounds: RectBounds, point: vec2i) -> bool {
	p := cast(vec2f)point
	c := cast(vec2f)vec2i{bounds.center_x, bounds.center_y}
	adj_point := p - c
	rot_adj_point := vec2f_rotate(adj_point, -bounds.rot)
	w := (cast(f32)bounds.width) / 2
	h := (cast(f32)bounds.height) / 2
	return(
		rot_adj_point.x >= -w &&
		rot_adj_point.y >= -h &&
		rot_adj_point.x <= w &&
		rot_adj_point.y <= h \
	)
}

get_bounding_box_of_bounds :: proc(bounds: RectBounds) -> BoundingBox {
	points := rect_bounds_vertices(bounds)
	return bounding_box_of_points(points[:])
}

image_draw_rect_bounds :: proc(img: ^Image, bounds: RectBounds, color: color_t) {
	bx := get_bounding_box_of_bounds(bounds)
	for dy in bx.y ..< bx.y + bx.height {
		for dx in bx.x ..< bx.x + bx.width {
			if bounds_contains_point(bounds, {dx, dy}) {
				image_draw_pixel(img, dx, dy, color)
			}
		}
	}
}
image_draw_rect_bounds_line :: proc(img: ^Image, bounds: RectBounds, color: color_t) {
	verts := rect_bounds_vertices(bounds)
	for i in 0 ..< len(verts) {
		image_draw_line_v(img, verts[i], verts[(i + 1) % len(verts)], color)
	}
}
