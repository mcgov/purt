/* hash functions

*/

use libc;

pub const DIR_SEED: [u32; 4] =
    [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

pub mod dirhash {
    pub fn create_dirhash(s: [u32; 4], filename: &str) -> (u32, u32) {
        // md4 specific
        let mut seed: [u32; 4];
        if s == [0, 0, 0, 0] {
            seed = super::DIR_SEED.clone();
        } else {
            println!("{:X?}", s);
            seed = s;
        }
        let bytes = filename.as_bytes();
        let mut len = bytes.len();
        let mut input = [0u32; 8];
        let mut fnamep = 0;
        loop {
            println!("len:{}", len);
            str_to_hashbuf(&filename[fnamep..], 8, &mut input);
            super::mdfour::half_md4_transform(&mut seed, input);
            if len < 32 {
                break;
            }
            len -= 32;
            fnamep += 32;
        }
        if filename == "165filler" {
            println!("{:X?}", seed[1] & !1);
            std::process::exit(0);
        }
        (seed[1] & !1, seed[2])
    }
    /*
        pub unsafe extern "C" fn str2hashbuf_signed(
            mut msg: *const libc::c_char,
            mut len: libc::c_int,
            mut buf: *mut libc::c_uint,
            mut num: libc::c_int,
        ) {
            let mut pad: libc::c_uint = 0;
            let mut val: libc::c_uint = 0;
            let mut i: libc::c_int = 0;
            let mut scp: *const libc::c_schar =
                msg as *const libc::c_schar;
            pad = len as libc::c_uint
                | (len as libc::c_uint) << 8 as libc::c_int;
            pad |= pad << 16 as libc::c_int;
            val = pad;
            if len > num * 4 as libc::c_int {
                len = num * 4 as libc::c_int
            }
            i = 0 as libc::c_int;
            while i < len {
                val = (*scp.offset(i as isize) as libc::c_int
                    as libc::c_uint)
                    .wrapping_add(val << 8 as libc::c_int);
                if i % 4 as libc::c_int == 3 as libc::c_int {
                    let fresh0 = buf;
                    buf = buf.offset(1);
                    *fresh0 = val;
                    val = pad;
                    num -= 1
                }
                i += 1
            }
            num -= 1;
            if num >= 0 as libc::c_int {
                let fresh1 = buf;
                buf = buf.offset(1);
                *fresh1 = val
            }
            loop {
                num -= 1;
                if !(num >= 0 as libc::c_int) {
                    break;
                }
                let fresh2 = buf;
                buf = buf.offset(1);
                *fresh2 = pad
            }
        }
    */
    fn str_to_hashbuf(fname: &str, num_: i32, buf: &mut [u32; 8]) {
        // in honor of the original code I will also not
        // comment any of this.
        let mut pad: u32;
        let mut val: u32;
        let mut num: i32 = num_;
        let bytes = fname.as_bytes();
        let mut len = (bytes.len()) as u32;
        pad = len | (len << 8);
        pad |= pad << 16;
        val = pad;
        println!("{:x}", pad);
        if len > (num as u32 * 4) {
            len = num as u32 * 4;
        }
        let mut outc = 0;
        for i in 0..len as usize {
            val = bytes[i] as u32 + (val << 8);
            if (i % 4) == 3 {
                buf[outc] = val;
                val = pad;
                num -= 1;
                outc += 1;
            }
        }
        num -= 1;
        if num >= 0 {
            buf[outc] = val;
            outc += 1;
        }
        while num - 1 >= 0 {
            buf[outc] = pad;
            outc += 1;
            num -= 1;
        }
        println!("BUF: {:x?}", buf);
    }
    /*
        static void str2hashbuf_signed(const char *msg, int len, __u32 *buf, int num)
    {
        __u32	pad, val;
        int	i;
        const signed char *scp = (const signed char *) msg;

        pad = (__u32)len | ((__u32)len << 8);
        pad |= pad << 16;

        val = pad;
        if (len > num*4)
            len = num * 4;
        for (i = 0; i < len; i++) {
            val = ((int) scp[i]) + (val << 8);
            if ((i % 4) == 3) {
                *buf++ = val;
                val = pad;
                num--;
            }
        }
        if (--num >= 0)
            *buf++ = val;
        while (--num >= 0)
            *buf++ = pad;
    }*/
}

pub mod tea {
    // this is probably broken, completely untested
    fn transform(v: [u32; 4], k: [u32; 4]) -> u64 {
        let mut sum: u32 = 0; /* set up */
        let delta: u32 = 0x9E3779B9; /* a key schedule constant */
        let mut v0 = v[0];
        let mut v1 = v[1];
        for _ in 0..16 {
            /* basic cycle start */
            sum = sum.wrapping_add(delta);
            v0 += ((v1 << 4).wrapping_add(k[0]))
                ^ (v1.wrapping_add(sum))
                ^ ((v1 >> 5).wrapping_add(k[1]));
            v1 += ((v0 << 4).wrapping_add(k[2]))
                ^ (v0.wrapping_add(sum))
                ^ ((v0 >> 5).wrapping_add(k[3]));
        } /* end cycle */
        let mut accum = v0.to_ne_bytes().to_vec();
        accum.append(&mut v1.to_ne_bytes().to_vec());

        u64::from_ne_bytes(accum[..].try_into().unwrap())
    }
}

pub mod mdfour {
    use md4::{Digest, Md4};
    /*I think ext4 doesn't actually use md4, by 'half' it literally
    only runs half the rounds and only uses half the result. */
    /*
    pub fn hash(data: &[u8]) -> [u8; 16] {
        let mut hasher = Md4::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    pub fn hash_ext4(data: &[u8]) -> u32 {
        let mut hasher = Md4::new();
        hasher.update(data);
        let hash = hasher.finalize();
        u32::from_le_bytes([hash[4], hash[5], hash[6], hash[7]])
    }*/

    const K1: u32 = 0;
    const K2: u32 = 0x5a827999;
    const K3: u32 = 0x6ed9eba1;

    /*
     * Basic cut-down MD4 transform.  Returns only 32 bits of
     * result.
     */

    fn efph(x: u32, y: u32, z: u32) -> u32 {
        (z) ^ ((x) & ((y) ^ (z)))
    }
    fn geez(x: u32, y: u32, z: u32) -> u32 {
        ((x) & (y)).wrapping_add(((x) ^ (y)) & (z))
    }
    fn eych(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    fn round(
        f: fn(u32, u32, u32) -> u32,
        a: &mut u32,
        b: u32,
        c: u32,
        d: u32,
        x: u32,
        s: u32,
    ) {
        println!("1 {:X}", a);
        let inner = f(b, c, d).wrapping_add(x);
        *a = a.wrapping_add(inner);
        *a = a.rotate_left(s);
        println!("2 {:X}", a);
    }

    pub fn half_md4_transform(
        seed: &mut [u32; 4],
        data: [u32; 8],
    ) -> u32 {
        let mut a = seed[0];
        let mut b = seed[1];
        let mut c = seed[2];
        let mut d = seed[3];
        /* Round 1 */

        round(efph, &mut a, b, c, d, (data[0]).wrapping_add(K1), 3);
        round(efph, &mut d, a, b, c, (data[1]).wrapping_add(K1), 7);
        round(efph, &mut c, d, a, b, (data[2]).wrapping_add(K1), 11);
        round(efph, &mut b, c, d, a, (data[3]).wrapping_add(K1), 19);
        round(efph, &mut a, b, c, d, (data[4]).wrapping_add(K1), 3);
        round(efph, &mut d, a, b, c, (data[5]).wrapping_add(K1), 7);
        round(efph, &mut c, d, a, b, (data[6]).wrapping_add(K1), 11);
        round(efph, &mut b, c, d, a, (data[7]).wrapping_add(K1), 19);
        /* Round 2 */
        println!("-----------------------------");
        round(geez, &mut a, b, c, d, (data[1]).wrapping_add(K2), 3);
        round(geez, &mut d, a, b, c, (data[3]).wrapping_add(K2), 5);
        round(geez, &mut c, d, a, b, (data[5]).wrapping_add(K2), 9);
        round(geez, &mut b, c, d, a, (data[7]).wrapping_add(K2), 13);
        round(geez, &mut a, b, c, d, (data[0]).wrapping_add(K2), 3);
        round(geez, &mut d, a, b, c, (data[2]).wrapping_add(K2), 5);
        round(geez, &mut c, d, a, b, (data[4]).wrapping_add(K2), 9);
        round(geez, &mut b, c, d, a, (data[6]).wrapping_add(K2), 13);
        /* Round 3 */
        println!("-----------------------------");

        round(eych, &mut a, b, c, d, (data[3]).wrapping_add(K3), 3);
        round(eych, &mut d, a, b, c, (data[7]).wrapping_add(K3), 9);
        round(eych, &mut c, d, a, b, (data[2]).wrapping_add(K3), 11);
        round(eych, &mut b, c, d, a, (data[6]).wrapping_add(K3), 15);
        round(eych, &mut a, b, c, d, (data[1]).wrapping_add(K3), 3);
        round(eych, &mut d, a, b, c, (data[5]).wrapping_add(K3), 9);
        round(eych, &mut c, d, a, b, (data[0]).wrapping_add(K3), 11);
        round(eych, &mut b, c, d, a, (data[4]).wrapping_add(K3), 15);
        seed[0] = seed[0].wrapping_add(a);
        seed[1] = seed[1].wrapping_add(b);
        seed[2] = seed[2].wrapping_add(c);
        seed[3] = seed[3].wrapping_add(d);
        return seed[1]; /* "most hashed" word */
    }
}
