use display;

pub fn draw_line( mut x0 : i64, mut y0 : i64, x1 : i64, y1 : i64, s : &mut Vec<Vec<[i64; 3]>>, c : [i64; 3] ){
    let (mx, my) = (x1 - x0, y1 - y0);
    if x0 > x1 { // we want the (x1,y1) pair to always be greater so we switch
        return draw_line(x1, y1, x0, y0, s, c);
    }
    let (A, B) = (my, -1 * (mx));
    let mut d : i64;
    if my > 0{ // first quadrant
        if mx > my  { // first octant
            d = 2*A + B;
            while x0 <= x1 {
                display::plot(s, c, x0, y0);
                if d > 0 {
                    y0 += 1;
                    d += 2*B;
                }
                x0 += 1;
                d += 2*A;
            }
        }
        else{ // second octant
            d = A + 2*B;
            while y0 <= y1 {
                display::plot(s, c, x0, y0);
                if d < 0 {
                    x0 += 1;
                    d += 2*A;
                }
                y0 += 1;
                d += 2*B;
            }
        }
    }
    else { // quadrant 4
        if mx + my < 0 { // octant 7, mx is always positive so my would need to be less to satisfy
            d = A - 2*B;
            while y0 >= y1 {
                display::plot(s, c, x0, y0);
                if d > 0 {
                    x0 += 1;
                    d += 2*A;
                }
                y0 -= 1;
                d -= 2*B;
            }
        }
        else { // octant 8
            d = 2*A - B;
            while x0 <= x1 {
                display::plot(s, c, x0, y0);
                if d < 0 {
                    y0 -= 1;
                    d -= 2*B;
                }
                x0 += 1;
                d += 2*A;
            }
        }
    }
}
