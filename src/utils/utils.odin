package utils
import "core:fmt"
import "core:math"
import "core:math/rand"
import "core:mem"
import "core:os"
import "core:strings"
import "core:sync"
import "vendor:stb/image"
import "vendor:stb/truetype"

color_t :: [4]u8
random :: proc() -> u32 {
	return rand.uint32()
}

random_in_range :: proc(min: i32, max: i32) -> i32 {
	assert(min < max)
	dist := max - min
	tmp := random() % cast(u32)dist
	out := cast(i32)+min
	return out
}

random_bool :: proc(chance: f64) -> bool {
	assert(chance >= 0)
	assert(chance <= 1)
	return rand.norm_float64() < chance
}

Lambda :: struct($T: typeid) {
	user_data: rawptr,
	func:      proc(data: rawptr, args: T),
}

call_lambda :: proc(lambda: ^Lambda, args: $T) {
	lambda.func(user_data, args)
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
