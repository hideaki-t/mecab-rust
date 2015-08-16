/*
extern crate libc;
use libc::size_t;
*/
use std::os::raw::{
    c_char,
    c_short,
    c_long,
    c_float,
    c_uchar,
    c_uint,
    c_ushort,
    c_ulong,
};
use std::ffi::CString;
use std::ffi::CStr;
use std::str;
use std::vec::Vec;

#[repr(C,packed)]
struct mecab_t;
#[repr(C,packed)]
struct mecab_path_t;
#[repr(C,packed)]
struct mecab_node_t {
    prev: *const mecab_node_t,
    next: *const mecab_node_t,
    enext: *const mecab_node_t,
    bnext: *const mecab_node_t,
    rpath: *const mecab_path_t,
    lpath: *const mecab_path_t,
    surface: *const c_char,
    feature: *const c_char,
    id: c_uint,
    length: c_ushort,
    rlength: c_ushort,
    rc_attr: c_ushort,
    lc_attr: c_ushort,
    posid: c_ushort,
    char_type: c_uchar,
    stat: c_uchar,
    is_best: c_uchar,
    alpha: c_float,
    beta: c_float,
    prob: c_float,
    wcost: c_short,
    cost: c_long
}

#[link(name="mecab")]
extern {
    fn mecab_new2(arg: *const c_char) -> &mecab_t;
    fn mecab_sparse_tonode2(mecab: &mecab_t, input: *const c_char, len: c_ulong) -> &mecab_node_t;
    fn mecab_destroy(mecab: &mecab_t);
}


#[test]
fn it_works() {
    let mut r = Vec::new();
    let arg = CString::new("").unwrap();
    let input = CString::new("すもももももももものうち").unwrap();
    unsafe {
        let mecab = mecab_new2(arg.as_ptr());
        let node = mecab_sparse_tonode2(mecab, input.as_ptr(), input.as_bytes().len() as c_ulong);
        let mut cur = node as *const mecab_node_t;
        while  !cur.is_null() {
            let n = &*cur;
            // the regions will be destroyed by mecab_destroy, need to own them
            r.push((
                str::from_utf8(CStr::from_ptr(n.surface).to_bytes()).unwrap().slice_unchecked(0, n.length as usize).to_string(),
                str::from_utf8(CStr::from_ptr(n.feature).to_bytes()).unwrap().to_string()
                    ));
            cur = (*cur).next;
        }
        mecab_destroy(mecab);
    }
    let e = vec![("", "BOS/EOS,*,*,*,*,*,*,*,*"),
                 ("すもも", "名詞,一般,*,*,*,*,すもも,スモモ,スモモ"),
                 ("も", "助詞,係助詞,*,*,*,*,も,モ,モ"),
                 ("もも", "名詞,一般,*,*,*,*,もも,モモ,モモ"),
                 ("も", "助詞,係助詞,*,*,*,*,も,モ,モ"),
                 ("もも", "名詞,一般,*,*,*,*,もも,モモ,モモ"),
                 ("の", "助詞,連体化,*,*,*,*,の,ノ,ノ"),
                 ("うち", "名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ"),
                 ("", "BOS/EOS,*,*,*,*,*,*,*,*"),
                 ].iter().map(|&x| (x.0.to_string(), x.1.to_string())).collect::<Vec<_>>();
    assert_eq!(e, r);
}
