import gzip, json, sys, bisect, collections, re

def build_symmap(symspath):
    s=json.load(open(symspath)); strs=s['string_table']
    libs={}
    for entry in s['data']:
        tab=sorted(entry['symbol_table'], key=lambda e:e['rva'])
        rvas=[e['rva'] for e in tab]
        libs[(entry['debug_name'], entry['debug_id'].replace('-','').upper())]=(rvas,tab,strs)
    # also key by debug_name alone as fallback
    for entry in s['data']:
        tab=sorted(entry['symbol_table'], key=lambda e:e['rva'])
        libs.setdefault(entry['debug_name'], ([e['rva'] for e in tab],tab,strs))
    return libs

def lookup(libs, libkey, addr):
    ent = libs.get(libkey) or libs.get(libkey[0] if isinstance(libkey,tuple) else libkey)
    if not ent: return None
    rvas,tab,strs=ent
    i=bisect.bisect_right(rvas,addr)-1
    if i<0: return None
    e=tab[i]
    if e['rva']<=addr<e['rva']+max(e.get('size') or 1,1):
        return strs[e['symbol']]
    return None

def report(prof, syms, topn=30, only_thread=None):
    d=json.load(gzip.open(prof)); libs=build_symmap(syms)
    liblist=d['libs']
    agg=collections.Counter(); total=0
    for th in d['threads']:
        if only_thread and th.get('name')!=only_thread: continue
        stab=th['stackTable']; ft=th['frameTable']; fu=th['funcTable']
        rt=th['resourceTable']; strs=th['stringArray']
        sam=th["samples"]; stacks=sam.get("stack") or sam.get("data") or []
        sfr=stab['frame']; ffunc=ft['func']; faddr=ft['address']
        fname=fu['name']; fres=fu['resource']; rlib=rt['lib']
        for s in stacks:
            if s is None: continue
            total+=1
            fr=sfr[s]; fnc=ffunc[fr]; addr=faddr[fr]
            res=fres[fnc]; nm=None
            if res is not None and res>=0 and res<len(rlib):
                li=rlib[res]
                if li is not None and li<len(liblist):
                    L=liblist[li]
                    key=(L.get('debugName'), (L.get('breakpadId') or '')[:32])
                    nm=lookup(libs,key,addr)
                    if nm is None: nm=lookup(libs,(L.get('debugName'),''),addr)
            if nm is None: nm=strs[fname[fnc]]
            agg[nm]+=1
    print(f"\n=== {prof} — {total} samples ===")
    for nm,c in agg.most_common(topn):
        print(f"  {100*c/total:6.2f}% {c:6d}  {nm[:120]}")
    return agg,total

if __name__=='__main__':
    p=sys.argv[1]; report(p, p.replace('.json.gz','.json.syms.json'),
                          topn=int(sys.argv[2]) if len(sys.argv)>2 else 30)
