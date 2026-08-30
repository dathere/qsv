use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use std::time::Instant;

fn load(path:&str)->Vec<Vec<Vec<u8>>>{
    let d=std::fs::read(path).unwrap(); let mut p=0usize;
    let rd=|d:&[u8],p:&mut usize|{let v=u32::from_le_bytes(d[*p..*p+4].try_into().unwrap()); *p+=4; v as usize};
    let nc=rd(&d,&mut p); let mut cols=Vec::with_capacity(nc);
    for _ in 0..nc { let n=rd(&d,&mut p); let mut c=Vec::with_capacity(n);
        for _ in 0..n { let l=d[p] as usize; p+=1; c.push(d[p..p+l].to_vec()); p+=l; } cols.push(c); }
    cols
}
const NCHUNK: usize = 16;

// build per-chunk maps exactly like Stats::new + add_borrowed
fn build(col:&[Vec<u8>], hint_div: usize)->Vec<HashMap<Vec<u8>,u64>>{
    let per = col.len()/NCHUNK;
    (0..NCHUNK).map(|i|{
        let lo=i*per; let hi=if i==NCHUNK-1 {col.len()} else {(i+1)*per};
        let cap=(per/hint_div).clamp(16,65_536);
        let mut m:HashMap<Vec<u8>,u64>=HashMap::with_capacity(cap);
        for v in &col[lo..hi] { *m.entry_ref(v.as_slice()).or_insert(0)+=1; }
        m
    }).collect()
}
// A: current Commute::merge — incremental reserve(v.len()) per chunk
fn merge_a(mut ms:Vec<HashMap<Vec<u8>,u64>>)->usize{
    let mut acc=ms.remove(0);
    for v in ms { acc.reserve(v.len());
        for (k,c) in v { match acc.entry(k){ Entry::Vacant(e)=>{e.insert(c);}, Entry::Occupied(mut e)=>{*e.get_mut()+=c;} } } }
    acc.len()
}
// B: reserve the summed total ONCE before merging
fn merge_b(mut ms:Vec<HashMap<Vec<u8>,u64>>)->usize{
    let total:usize=ms.iter().skip(1).map(|m|m.len()).sum();
    let mut acc=ms.remove(0);
    acc.reserve(total);
    for v in ms {
        for (k,c) in v { match acc.entry(k){ Entry::Vacant(e)=>{e.insert(c);}, Entry::Occupied(mut e)=>{*e.get_mut()+=c;} } } }
    acc.len()
}
fn main(){
    let cols=load("/tmp/statsperf/fields.bin");
    for (label,div) in [("current hint /10",10usize)] {
        let mut ba=f64::MAX; let mut bb=f64::MAX; let (mut la,mut lb)=(0,0);
        for _ in 0..3 {
            let pre:Vec<_>=cols.iter().map(|c|build(c,div)).collect();
            let t=Instant::now(); let mut s=0; for m in pre { s+=merge_a(m); }
            let e=t.elapsed().as_secs_f64(); if e<ba{ba=e;} la=s;
            let pre:Vec<_>=cols.iter().map(|c|build(c,div)).collect();
            let t=Instant::now(); let mut s=0; for m in pre { s+=merge_b(m); }
            let e=t.elapsed().as_secs_f64(); if e<bb{bb=e;} lb=s;
        }
        assert_eq!(la,lb,"merge results differ!");
        println!("[{label}]  final entries={la}");
        println!("  A incremental reserve : {:.2} ms", ba*1e3);
        println!("  B reserve-total-once  : {:.2} ms", bb*1e3);
        println!("  delta                 : {:+.1}%", 100.0*(bb-ba)/ba);
    }
}
