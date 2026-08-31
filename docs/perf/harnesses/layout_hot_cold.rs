use stats::{Frequencies, MinMax, OnlineStats, Unsorted};
use hashbrown::HashMap;
use std::mem::size_of;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Default)]
enum FieldType { #[default] TNull, TString, TFloat, TInteger }
#[derive(Clone, Default)]
struct WhichStats { a:[bool;15], cap:u64, pl: Box<str> }
#[derive(Clone, Default)]
struct TypedSum { float: Option<f64>, integer: i64, stotlen: u64 }
#[derive(Clone, Default)]
struct WOS { a:f64,b:f64,c:f64,d:f64,e:f64,f:f64,g:f64,h:usize }
#[derive(Clone, Default)]
struct TypedMinMax { floats: MinMax<f64>, integers: MinMax<i64>, dates: MinMax<i64>,
 strings: MinMax<Vec<u8>>, str_len: MinMax<usize> }
impl TypedMinMax {
    #[inline(always)]
    fn add(&mut self, typ: FieldType, s:&[u8], fv:f64, iv:i64){
        let n=s.len();
        if n==0 { self.str_len.add(0); return; }
        self.str_len.add(n);
        self.strings.add_bytes(s);
        match typ { FieldType::TInteger=>{self.integers.add(iv); self.floats.add(fv);},
                    FieldType::TFloat=>{self.floats.add(fv);}, _=>{} }
    }
}
#[derive(Clone, Default)] struct TDigestSlot(Option<Box<[u8;0]>>);
#[derive(Clone, Default)] struct HllSlot(Option<Box<[u8;0]>>);

// ---------- Layout A: current, everything inline, 896 B ----------
#[repr(C, align(64))]
#[derive(Clone)]
struct StatsA {
    typ: FieldType, is_ascii: bool, modes_dropped: bool, zpn_d: bool, zpn_h: bool, max_prec: u16,
    nullcount: u64, total_weight: f64,
    which: WhichStats,
    sum: Option<TypedSum>,
    online: Option<OnlineStats>, online_len: Option<OnlineStats>,
    weighted_online: Option<WOS>,
    modes: Option<Frequencies<Vec<u8>>>,
    weighted_modes: Option<HashMap<Vec<u8>, f64>>,
    unsorted_stats: Option<Unsorted<f64>>,
    weighted_unsorted_stats: Option<Vec<(f64,f64)>>,
    tdigest: TDigestSlot, hll: HllSlot,
    minmax: Option<TypedMinMax>,
}
// ---------- Layout B: hot only; cold fields live in a parallel Vec ----------
#[repr(C, align(64))]
#[derive(Clone)]
struct StatsB {
    typ: FieldType, is_ascii: bool, modes_dropped: bool, zpn_d: bool, zpn_h: bool, max_prec: u16,
    nullcount: u64, total_weight: f64,
    sum: Option<TypedSum>,
    online: Option<OnlineStats>, online_len: Option<OnlineStats>,
    modes: Option<Frequencies<Vec<u8>>>,
    minmax: Option<TypedMinMax>,
}
#[derive(Clone, Default)]
struct ColdB { which: WhichStats, weighted_online: Option<WOS>,
 weighted_modes: Option<HashMap<Vec<u8>,f64>>, unsorted_stats: Option<Unsorted<f64>>,
 weighted_unsorted_stats: Option<Vec<(f64,f64)>>, tdigest: TDigestSlot, hll: HllSlot }

#[inline(always)]
fn infer(s:&[u8])->(FieldType,i64,f64){
    if s.is_empty() { return (FieldType::TNull,0,0.0); }
    let mut iv=0i64; let mut ok=true;
    for &b in s { if b.is_ascii_digit() { iv=iv.wrapping_mul(10).wrapping_add((b-b'0') as i64); } else { ok=false; break; } }
    if ok { (FieldType::TInteger, iv, iv as f64) } else { (FieldType::TString,0,0.0) }
}
macro_rules! addbody { ($self:expr, $s:expr) => {{
    let s=$s; let (t,iv,fv)=infer(s);
    if s.is_empty() { $self.nullcount+=1; }
    if !s.is_empty() { let sm=$self.sum.as_mut().unwrap(); sm.stotlen=sm.stotlen.saturating_add(s.len() as u64);
        if t==FieldType::TInteger { sm.integer=sm.integer.saturating_add(iv); } }
    $self.minmax.as_mut().unwrap().add(t,s,fv,iv);
    if let Some(m)=$self.modes.as_mut() { m.add_borrowed(s); }
    $self.online_len.as_mut().unwrap().add_f64(s.len() as f64);
    if t==FieldType::TInteger { $self.online.as_mut().unwrap().add_f64(fv); }
}}}
impl StatsA { #[inline(always)] fn add(&mut self, s:&[u8]){ addbody!(self,s) } }
impl StatsB { #[inline(always)] fn add(&mut self, s:&[u8]){ addbody!(self,s) } }

fn load(p:&str)->(usize,usize,Vec<Vec<Vec<u8>>>){
    let d=std::fs::read(p).unwrap(); let mut i=0;
    let nc=u32::from_le_bytes(d[0..4].try_into().unwrap()) as usize;
    let nr=u32::from_le_bytes(d[4..8].try_into().unwrap()) as usize; i=8;
    let mut cols=Vec::with_capacity(nc);
    for _ in 0..nc { let mut c=Vec::with_capacity(nr);
        for _ in 0..nr { let l=d[i] as usize; i+=1; c.push(d[i..i+l].to_vec()); i+=l; } cols.push(c); }
    (nc,nr,cols)
}
fn mkA(n:usize)->Vec<StatsA>{ (0..n).map(|_| StatsA{ typ:Default::default(), is_ascii:true,
 modes_dropped:false, zpn_d:false, zpn_h:false, max_prec:0, nullcount:0, total_weight:0.0,
 which:Default::default(), sum:Some(Default::default()), online:Some(OnlineStats::new()),
 online_len:Some(OnlineStats::new()), weighted_online:None,
 modes:Some(Frequencies::with_capacity(6250)), weighted_modes:None, unsorted_stats:None,
 weighted_unsorted_stats:None, tdigest:Default::default(), hll:Default::default(),
 minmax:Some(Default::default())}).collect() }
fn mkB(n:usize)->(Vec<StatsB>,Vec<ColdB>){ ((0..n).map(|_| StatsB{ typ:Default::default(),
 is_ascii:true, modes_dropped:false, zpn_d:false, zpn_h:false, max_prec:0, nullcount:0,
 total_weight:0.0, sum:Some(Default::default()), online:Some(OnlineStats::new()),
 online_len:Some(OnlineStats::new()), modes:Some(Frequencies::with_capacity(6250)),
 minmax:Some(Default::default())}).collect(), (0..n).map(|_| ColdB::default()).collect()) }

/// Full observable snapshot of one column's accumulators, for A-vs-B equivalence.
#[derive(PartialEq, Debug)]
struct Snap { nullcount:u64, stotlen:u64, integer:i64, len_n:u64, len_mean:u64,
              on_n:u64, on_mean:u64, on_var:u64, card:u64,
              smin:Option<Vec<u8>>, smax:Option<Vec<u8>>, lmin:Option<usize>, lmax:Option<usize>,
              asc:Option<u32>, desc:Option<u32> }
macro_rules! snap { ($s:expr) => {{ let s=&$s; let mm=s.minmax.as_ref().unwrap();
    let ol=s.online_len.as_ref().unwrap(); let on=s.online.as_ref().unwrap();
    Snap{ nullcount:s.nullcount, stotlen:s.sum.as_ref().unwrap().stotlen,
          integer:s.sum.as_ref().unwrap().integer,
          len_n:ol.len() as u64, len_mean:ol.mean().to_bits(), on_n:on.len() as u64,
          on_mean:on.mean().to_bits(), on_var:on.variance().to_bits(),
          card:s.modes.as_ref().unwrap().cardinality(),
          smin:mm.strings.min().cloned(), smax:mm.strings.max().cloned(),
          lmin:mm.str_len.min().copied(), lmax:mm.str_len.max().copied(),
          asc:Some(mm.strings.sort_order() as u32), desc:None } }} }

fn equivalence(nc:usize, nr:usize, cols:&Vec<Vec<Vec<u8>>>){
    let mut a=mkA(nc); let (mut b,_c)=mkB(nc);
    for r in 0..nr { for c in 0..nc { a[c].add(&cols[c][r]); b[c].add(&cols[c][r]); } }
    for c in 0..nc {
        assert_eq!(snap!(a[c]), snap!(b[c]), "layout A/B diverged on column {c}");
    }
    println!("equivalence: OK — A and B produce identical accumulator state on all {nc} columns");
}

fn bench(nc:usize, nr:usize, cols:&Vec<Vec<Vec<u8>>>)->(f64,f64){
    let reps=3; let (mut ba,mut bb)=(f64::MAX,f64::MAX);
    for _ in 0..reps {
        let mut st=mkA(nc);
        let t=Instant::now();
        for r in 0..nr { for c in 0..nc { st[c].add(&cols[c][r]); } }
        let e=t.elapsed().as_secs_f64(); if e<ba{ba=e;}
        std::hint::black_box(st.iter().map(|s|s.nullcount).sum::<u64>());
        let (mut st,cold)=mkB(nc);
        let t=Instant::now();
        for r in 0..nr { for c in 0..nc { st[c].add(&cols[c][r]); } }
        let e=t.elapsed().as_secs_f64(); if e<bb{bb=e;}
        std::hint::black_box((st.iter().map(|s|s.nullcount).sum::<u64>(), cold.len()));
    }
    (ba,bb)
}
fn main(){
    let (nc0,nr0,cols0)=load("/tmp/statsperf/fields_rm.bin");
    println!("size_of StatsA = {} B, StatsB = {} B, cold = {} B\n", size_of::<StatsA>(), size_of::<StatsB>(), size_of::<ColdB>());
    // hold rows fixed at a smaller count so wide runs stay comparable in total work
    let nr = 40_000.min(nr0);
    // Which variant to time. Running exactly ONE per process removes the
    // cross-contamination that made the in-process A-then-B ordering decide the result.
    let which = std::env::args().nth(1).unwrap_or_else(|| "both".into());
    if which == "eq" {
        equivalence(nc0, 20_000.min(nr0), &(0..nc0).map(|c| cols0[c][..20_000.min(nr0)].to_vec()).collect());
        return;
    }
    println!("{:>6} {:>8} {:>11}", "cols", "variant", "ms");
    for mult in [1usize,3,6,12,24] {
        let nc = nc0*mult;
        // replicate the real columns to widen the file
        let cols: Vec<Vec<Vec<u8>>> = (0..nc).map(|c| cols0[c % nc0][..nr].to_vec()).collect();
        let t = match which.as_str() {
            "A" => { let mut best=f64::MAX;
                     for _ in 0..3 { let mut st=mkA(nc); let t=Instant::now();
                       for r in 0..nr { for c in 0..nc { st[c].add(&cols[c][r]); } }
                       let e=t.elapsed().as_secs_f64(); if e<best{best=e;}
                       std::hint::black_box(st.iter().map(|s|s.nullcount).sum::<u64>()); }
                     best }
            "B" => { let mut best=f64::MAX;
                     for _ in 0..3 { let (mut st,cold)=mkB(nc); let t=Instant::now();
                       for r in 0..nr { for c in 0..nc { st[c].add(&cols[c][r]); } }
                       let e=t.elapsed().as_secs_f64(); if e<best{best=e;}
                       std::hint::black_box((st.iter().map(|s|s.nullcount).sum::<u64>(),cold.len())); }
                     best }
            _ => { let (a,b)=bench(nc,nr,&cols);
                   println!("{:>6} {:>11.1} {:>11.1} {:>+8.1}%", nc, a*1e3, b*1e3, 100.0*(b-a)/a);
                   continue; }
        };
        println!("{:>6} {:>8} {:>11.1}", nc, which, t*1e3);
    }
}
