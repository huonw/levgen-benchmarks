use std::rand::{Rng, RngUtil};
use std::{rand, os, int, uint, vec};

static TileDim: uint=50;
static MinWid: uint=2;
static MaxWid: uint=8;

fn main(){
    let args = os::args();
    let str = (args[1]);
    let v = int::from_str(str).get_or_default(18);
    println(fmt!("The random seed is: %?",v));

    let vstr = int::to_str(v);
    let vbytes = vstr.as_bytes_with_null_consume();
    let mut rng = rand::IsaacRng::new_seeded(vbytes);

    let ls: ~[Lev] = do vec::from_fn(100) |_| {
        let rs = rooms(&mut rng, 99);
        let mut ts: ~[Tile] = do vec::from_fn(TileDim * TileDim) |ii| {
            Tile {
                X: ii % TileDim,
                Y: ii / TileDim,
                T: 0
            }
        };

        for rs.iter().advance |r| {
            Room2Tiles(r, &mut ts);
        }
        Lev { TS: ts, RS: rs }
    };
    let BiggestLev = FindMostRooms(ls);
    PrintLev(&ls[BiggestLev]);
}

struct Tile {
    X: uint,
    Y: uint,
    T: uint,
}

struct Room {
    X: uint,
    Y: uint,
    W: uint,
    H: uint,
    N: uint
}

struct Lev {
    TS: ~[Tile],
    RS: ~[Room],
}

fn FindMostRooms(ls: &[Lev]) -> uint {
    let mut max = 0;
    let mut biggestLev = 0;
    for ls.iter().enumerate().advance |(i,l)| {
        let len = l.RS.len();
        if len > max {
            max = len;
            biggestLev = i;
        }
    }
    biggestLev
}

fn rooms<R: Rng>(rng: &mut R, n: uint) -> ~[Room] {
    let mut rooms = vec::with_capacity(n);
    for 50000.times {
        let x = rng.gen_uint_range(0,TileDim);
        let y = rng.gen_uint_range(0,TileDim);
        let w = rng.gen_uint_range(MinWid,MaxWid);
        let h = rng.gen_uint_range(MinWid,MaxWid);
        if x+w>=TileDim || y+h>=TileDim || x==0 || y==0 {
            loop
        }
        if NotCrash(x, y, w, h, rooms) {
            let r = Room { X: x, Y: y, W: w, H: h, N: rooms.len() };
            rooms.push(r);
            if rooms.len() == 99 { break } // found a room, so start on the next one
        }
    }
    rooms
}

fn NotCrash(x: uint, y: uint, w: uint, h: uint, rs: &[Room]) -> bool{
    do rs.iter().all |r| {
        let Room { X, Y, W, H, _ } = *r;

        ((X + W + 1) < x ||
         X > (x + w + 1) ||
         (Y + H + 1) < y ||
         Y > (y + h + 1))
    }
}

fn Room2Tiles(r: &Room, ts: &mut~[Tile]){
    let Room { X, Y, W, H, _} = *r;

    for uint:: range(X, X + W + 1) |xi| {
        for uint:: range(Y, Y + H + 1) |yi| {
            let num= yi*TileDim+xi;
            ts[num].T=1;
        }
    }
}

fn PrintLev(l: &Lev){
    for l.TS.iter().enumerate().advance |(i, tile)| {
        print(tile.T.to_str());
        if i % TileDim == 49 {
            print("\n");
        }
    }
}
