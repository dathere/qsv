import gzip,json,sys,collections
sys.path.insert(0,'/tmp/statsperf')
from sym import build_symmap, lookup
def inclusive(prof, targets):
    syms=prof.replace('.json.gz','.json.syms.json')
    d=json.load(gzip.open(prof)); libs=build_symmap(syms); liblist=d['libs']
    hits=collections.Counter(); tot=0
    for th in d['threads']:
        stab=th['stackTable']; ft=th['frameTable']; fu=th['funcTable']
        rt=th['resourceTable']; strs=th['stringArray']
        sam=th['samples']; stacks=sam.get('stack') or sam.get('data') or []
        pref=stab['prefix']; sfr=stab['frame']
        ffunc=ft['func']; faddr=ft['address']; fname=fu['name']; fres=fu['resource']; rlib=rt['lib']
        cache={}
        def nameof(si):
            if si in cache: return cache[si]
            fr=sfr[si]; fnc=ffunc[fr]; addr=faddr[fr]; res=fres[fnc]; n=None
            if res is not None and 0<=res<len(rlib):
                li=rlib[res]
                if li is not None and li<len(liblist):
                    L=liblist[li]
                    n=lookup(libs,(L.get('debugName'),(L.get('breakpadId') or '')[:32]),addr) or lookup(libs,(L.get('debugName'),''),addr)
            n = n or strs[fname[fnc]]
            cache[si]=n; return n
        for s in stacks:
            if s is None: continue
            tot+=1
            seen=set(); cur=s
            while cur is not None:
                n=nameof(cur)
                for t in targets:
                    if t in n and t not in seen: seen.add(t)
                cur=pref[cur]
            for t in seen: hits[t]+=1
    print(f"\n=== INCLUSIVE share in {prof} ({tot} samples) ===")
    for t in targets:
        print(f"  {100*hits[t]/tot:6.2f}%  {t}")
inclusive(sys.argv[1], sys.argv[2:])
