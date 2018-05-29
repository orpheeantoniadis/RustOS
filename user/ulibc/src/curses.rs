use io::*;

static mut SCR: FrameBuffer = [[Character::null();BUFFER_WIDTH];BUFFER_HEIGHT];

pub struct Image {
    content: *const u8,
    width: usize,
    height: usize
}

pub struct Window {
    img: Image,
    x: usize,
    y: usize,
    color: ColorAttribute
}

pub fn init_scr() {
    unsafe {
        copy_scr(&SCR);
        set_cursor(0, 0);
        cursor_disable(true);
    }
}

pub fn destroy_scr() {
    unsafe {
        SCR = [[Character::null();BUFFER_WIDTH];BUFFER_HEIGHT];
        copy_scr(&SCR);
        set_cursor(0, 0);
        cursor_disable(false);
    }
}

pub fn update_scr() {
    unsafe {
        copy_scr(&SCR);
    }
}

pub fn display_window(w: Window) {
    unsafe {
        for i in 0..w.img.height {
            for j in 0..w.img.width {
                let ascii = *w.img.content.offset((j + i * w.img.width) as isize);
                SCR[i+w.x][j+w.y] = Character::new(ascii, w.color);
            }
        }
    }
    update_scr();
}

impl Image {
    pub fn new(data: &str, width: usize, height: usize) -> Image {
        Image {
            content: data.as_ptr(),
            width: width,
            height: height
        }
    }
}

impl Window {
    pub fn new(img: Image, x: usize, y: usize, background: Color, foreground: Color) -> Window {
        Window {
            img: img,
            x: x,
            y: y,
            color: ColorAttribute::new(background, foreground)
        }
    }
}