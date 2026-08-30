use std::time::Instant;

// ---- Variant A: verbatim transcription of qsv-stats MinMax::add_bytes steady state ----
struct MmA { len: u32, asc: u32, desc: u32, last: Option<Vec<u8>>, min: Option<Vec<u8>>, max: Option<Vec<u8>> }
impl MmA {
    fn new() -> Self { MmA{len:0,asc:0,desc:0,last:None,min:None,max:None} }
    #[inline(always)]
    fn add(&mut self, sample: &[u8]) {
        if self.len >= 2 {
            let last = unsafe { self.last.as_ref().unwrap_unchecked() };
            if sample >= last.as_slice() {
                self.asc += 1;
                let max = unsafe { self.max.as_mut().unwrap_unchecked() };
                if sample > max.as_slice() { max.clear(); max.extend_from_slice(sample); }
            } else {
                self.desc += 1;
                let min = unsafe { self.min.as_mut().unwrap_unchecked() };
                if sample < min.as_slice() { min.clear(); min.extend_from_slice(sample); }
            }
            let lm = unsafe { self.last.as_mut().unwrap_unchecked() };
            lm.clear(); lm.extend_from_slice(sample);
            self.len += 1;
            return;
        }
        let o = sample.to_vec();
        if self.len == 0 { self.min=Some(o.clone()); self.max=Some(o.clone()); self.last=Some(o); }
        else { let lm=self.last.as_mut().unwrap(); lm.clear(); lm.extend_from_slice(sample); }
        self.len += 1;
    }
}

// ---- Variant B: H1c — inline first-byte short-circuit + Equal fast path ----
struct MmB { len: u32, asc: u32, desc: u32, last: Option<Vec<u8>>, min: Option<Vec<u8>>, max: Option<Vec<u8>> }
impl MmB {
    fn new() -> Self { MmB{len:0,asc:0,desc:0,last:None,min:None,max:None} }
    #[inline(always)]
    fn add(&mut self, sample: &[u8]) {
        if self.len >= 2 {
            let last = unsafe { self.last.as_ref().unwrap_unchecked() };
            // sample is non-empty by contract; last non-empty when len>=2
            let (a0, b0) = (sample[0], unsafe { *last.get_unchecked(0) });
            if a0 != b0 {
                // lexicographic order decided by the first differing byte: no memcmp needed
                if a0 > b0 {
                    self.asc += 1;
                    let max = unsafe { self.max.as_mut().unwrap_unchecked() };
                    if sample > max.as_slice() { max.clear(); max.extend_from_slice(sample); }
                } else {
                    self.desc += 1;
                    let min = unsafe { self.min.as_mut().unwrap_unchecked() };
                    if sample < min.as_slice() { min.clear(); min.extend_from_slice(sample); }
                }
            } else if sample == last.as_slice() {
                // Equal: ascending pair; max >= last == sample so max cannot be exceeded;
                // last already holds these bytes -> skip the copy entirely.
                self.asc += 1;
                self.len += 1;
                return;
            } else if sample > last.as_slice() {
                self.asc += 1;
                let max = unsafe { self.max.as_mut().unwrap_unchecked() };
                if sample > max.as_slice() { max.clear(); max.extend_from_slice(sample); }
            } else {
                self.desc += 1;
                let min = unsafe { self.min.as_mut().unwrap_unchecked() };
                if sample < min.as_slice() { min.clear(); min.extend_from_slice(sample); }
            }
            let lm = unsafe { self.last.as_mut().unwrap_unchecked() };
            lm.clear(); lm.extend_from_slice(sample);
            self.len += 1;
            return;
        }
        let o = sample.to_vec();
        if self.len == 0 { self.min=Some(o.clone()); self.max=Some(o.clone()); self.last=Some(o); }
        else { let lm=self.last.as_mut().unwrap(); lm.clear(); lm.extend_from_slice(sample); }
        self.len += 1;
    }
}

fn load(path:&str)->Vec<Vec<Vec<u8>>>{
    let d=std::fs::read(path).unwrap(); let mut p=0usize;
    let rd32=|d:&[u8],p:&mut usize|{let v=u32::from_le_bytes(d[*p..*p+4].try_into().unwrap()); *p+=4; v as usize};
    let nc=rd32(&d,&mut p); let mut cols=Vec::with_capacity(nc);
    for _ in 0..nc {
        let n=rd32(&d,&mut p); let mut c=Vec::with_capacity(n);
        for _ in 0..n { let l=d[p] as usize; p+=1; c.push(d[p..p+l].to_vec()); p+=l; }
        cols.push(c);
    }
    cols
}

fn main(){
    let cols=load("/tmp/statsperf/fields.bin");
    let total:usize=cols.iter().map(|c|c.len()).sum();
    println!("replaying {} columns, {} values\n", cols.len(), total);
    let reps=5;
    let mut best_a=f64::MAX; let mut best_b=f64::MAX;
    for _ in 0..reps {
        let t=Instant::now(); let mut s=0u64;
        for c in &cols { let mut m=MmA::new(); for v in c { m.add(v); } s+=m.asc as u64+m.desc as u64; }
        let e=t.elapsed().as_secs_f64(); if e<best_a {best_a=e;} std::hint::black_box(s);

        let t=Instant::now(); let mut s=0u64;
        for c in &cols { let mut m=MmB::new(); for v in c { m.add(v); } s+=m.asc as u64+m.desc as u64; }
        let e=t.elapsed().as_secs_f64(); if e<best_b {best_b=e;} std::hint::black_box(s);
    }
    // equivalence check
    for c in &cols {
        let mut a=MmA::new(); let mut b=MmB::new();
        for v in c { a.add(v); b.add(v); }
        assert_eq!(a.asc,b.asc,"asc mismatch"); assert_eq!(a.desc,b.desc,"desc mismatch");
        assert_eq!(a.min,b.min,"min mismatch"); assert_eq!(a.max,b.max,"max mismatch");
        assert_eq!(a.last,b.last,"last mismatch");
    }
    println!("equivalence: OK (asc/desc/min/max/last identical on all {} columns)", cols.len());
    println!("\n  A current      : {:.2} ms  ({:.2} ns/value)", best_a*1e3, best_a*1e9/total as f64);
    println!("  B H1c          : {:.2} ms  ({:.2} ns/value)", best_b*1e3, best_b*1e9/total as f64);
    println!("  delta          : {:+.1}%", 100.0*(best_b-best_a)/best_a);
}
