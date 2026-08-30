import gzip,json,sys,bisect,collections
sys.path.insert(0,'/tmp/statsperf')
from sym import build_symmap, lookup

def callers_of(prof, target_sub, depth=3, topn=12):
    syms=prof.replace('.json.gz','.json.syms.json')
    d=json.load(gzip.open(prof)); libs=build_symmap(syms); liblist=d['libs']
    agg=collections.Counter(); tot=0; leafhits=0; allsamples=0
    for th in d['threads']:
        stab=th['stackTable']; ft=th['frameTable']; fu=th['funcTable']
        rt=th['resourceTable']; strs=th['stringArray']
        sam=th['samples']; stacks=sam.get('stack') or sam.get('data') or []
        pref=stab['prefix']; sfr=stab['frame']
        ffunc=ft['func']; faddr=ft['address']; fname=fu['name']; fres=fu['resource']; rlib=rt['lib']
        def nameof(stackidx):
            fr=sfr[stackidx]; fnc=ffunc[fr]; addr=faddr[fr]; res=fres[fnc]
            if res is not None and 0<=res<len(rlib):
                li=rlib[res]
                if li is not None and li<len(liblist):
                    L=liblist[li]
                    n=lookup(libs,(L.get('debugName'),(L.get('breakpadId') or '')[:32]),addr) \
                      or lookup(libs,(L.get('debugName'),''),addr)
                    if n: return n
            return strs[fname[fnc]]
        for s in stacks:
            if s is None: continue
            allsamples+=1
            if target_sub not in nameof(s): continue
            leafhits+=1
            # walk up to find first non-target ancestor
            cur=pref[s]; chain=[]
            while cur is not None and len(chain)<depth:
                n=nameof(cur)
                if target_sub not in n: chain.append(n)
                cur=pref[cur]
            agg[" <- ".join(chain[:2]) if chain else "(root)"]+=1
            tot+=1
    print(f"\n### callers of '{target_sub}' in {prof}")
    print(f"    leaf samples: {leafhits} / {allsamples} total ({100*leafhits/allsamples:.2f}% of profile)")
    for nm,c in agg.most_common(topn):
        print(f"      {100*c/tot:6.2f}% ({100*c/allsamples:5.2f}% of profile)  {nm[:115]}")

for prof in sys.argv[1:]:
    callers_of(prof, 'StrftimeItems')
