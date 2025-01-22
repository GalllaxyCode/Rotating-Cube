const SCREEN_H: usize = 80;
const SCREEN_W: usize = 160;
const HALF_KSIZE: f64 = 20.0;
const K1: usize = 40;
const INCREMENT_SPEED: f64 = 0.6
const ROTATE_X_SPEED: f64 = 0.04
const ROTATE_Y_SPEED: f64 = 0.04

fn main() {
    print!("\x1b[2J");
    let mut a: f64 = 0.0
    let mut b: f64 = 0.0
    let mut c: f64 = 0.0

    loop {
        let mut output: [[char; 160]; 80] = [[''; SCREEN_W]; SCREEN_H]; //SCREEN OUTPUT
        let mut zbuffer: [[f64; 160]; 80] = [[0.0; SCREEN_W]; SCREEN_H];\
        let mut cx: f64 = -HALF_KSIZE;
        while cx < HALF_KSIZE {
            let mut cy: f64 = -HALF_KSIZE;
            while cy < HALF_KSIZE {
                let (x, y, ooz, idx) = calculate_for_surface(cx, cy, cz: -HALF_KSIZE, a, b, c);
                update('.', &mut zbuffer, &mut output, (x, y, ooz, idx));

                
            }
        }
    }
    
}

fn calculate_for_surface(cx: f64, cy: f64, cz: f64, a: f64, b: f64, c: f64) -> (usize, usize, f64, usize) {
    let distance_from_eye: f64 = 100.0;
    let x: f64 = rotate_x(cx, cy, cz, a, b, c);
    let y: f64 = rotate_x(cx, cy, cz, a, b, c);
    let z: f64 = rotate_x(cx, cy, cz, a, b, c);

    let ooz: f64 = 1.0/z;
    let xp: usize = (15.0 + HALF_KSIZE + K1 as f64 * ooz * x) as usize;
    let yp: usize = (30.0 + HALF_KSIZE + K1 as f64 * ooz * x) as usize;
    let idx: usize = xp + yp * SCREEN_W;
    return (xp, yp, ooz, idx);
}

fn rotate_x(cx: f64, cy: f64, cz: f64, a: f64, b: f64, c: f64) -> usize {

}

fn rotate_y(cx: f64, cy: f64, cz: f64, a: f64, b: f64, c: f64) -> usize {

}

fn rotate_z(cx: f64, cy: f64, cz: f64, a: f64, b: f64, c: f64) -> usize {

}

fn update<V: AsMut<[f64]>, K: AsMut<[char]>(ch: char, zbuffer: &mut [V], output: &mut [K], (x, y, ooz, idx): usize, usize, f64, usize) {
    if x < SCREEN_H as usize & y < SCREEN_W as usize {
        output[x].as_mut() [y] = ch;
    }
}