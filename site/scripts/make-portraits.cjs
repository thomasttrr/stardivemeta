// Usage (from the repo root; needs ffmpeg with libwebp on PATH):
//   node site/scripts/make-portraits.cjs design-assets site/public/img/own/art
// Builds the site's medallion portraits: one 320x320 WebP per character, a square
// crop centred on the face, cut from the design-assets ORIGINAL (read only).
// Focal points are the hand-set ones from the mockup (face centre x%, y% and
// head width as a fraction of width), measured on the scratch copies; each
// scratch copy covered a known rect of its original, so they map back exactly.
const {execFileSync}=require('child_process');const fs=require('fs');const path=require('path');
const [assets,outDir]=process.argv.slice(2);
const F={cloud:[38,31,.26],daisy:[42,22,.2],esther:[41,13,.2],flare:[52,18,.13],francis:[40,13,.17],gabi:[45,20,.13],jiwon:[47,22,.22],leeho:[32,20,.15],mina:[58,27,.13],nagi:[56,20,.14],narae:[50,20,.2],ophelia:[47,21,.18],penny:[28,42,.22],reina:[62,25,.15],sangun:[64,13,.12],verna:[56,17,.18],yeonhwa:[51,13,.2],chloe:[56,33,.22],isabella:[53,35,.2],vivian:[48,27,.17],brisshell:[47,32,.18],mabel:[44,50,.22],'verna-summer-dive':[50,28,.17],'francis-summer-dive':[65,39,.15],benjamin:[60,37,.4],sera:[43,37,.45],ellie:[47,48,.5],bonnie:[47,47,.5],angel:[47,50,.45]};
const CARD={chloe:'chloe',isabella:'isabella',vivian:'vivian',brisshell:'brisshell',mabel:'mabel','verna-summer-dive':'verna_summer','francis-summer-dive':'francis_summer'};
const BUST=['benjamin','sera','ellie','bonnie','angel'];
const FILL=0.62,CY=0.52,OUT=320;
const size=f=>execFileSync('ffprobe',['-v','error','-select_streams','v:0','-show_entries','stream=width,height','-of','csv=p=0',f]).toString().trim().split(',').map(Number);
function alphaRect(src){ // same bbox as trim.js
 const N=200;const raw=execFileSync('ffmpeg',['-loglevel','error','-i',src,'-vf',`format=rgba,alphaextract,scale=${N}:${N}`,'-f','rawvideo','-pix_fmt','gray','-'],{maxBuffer:1<<26});
 let x0=N,y0=N,x1=-1,y1=-1;for(let y=0;y<N;y++)for(let x=0;x<N;x++)if(raw[y*N+x]>12){if(x<x0)x0=x;if(x>x1)x1=x;if(y<y0)y0=y;if(y>y1)y1=y}
 const [W,H]=size(src);const cx=Math.max(0,Math.floor((x0-1)*W/N)),cy=Math.max(0,Math.floor((y0-1)*H/N));
 return [cx,cy,Math.min(W-cx,Math.ceil((x1-x0+3)*W/N)),Math.min(H-cy,Math.ceil((y1-y0+3)*H/N))]}
fs.mkdirSync(outDir,{recursive:true});
for(const [id,[fx,fy,hw]] of Object.entries(F)){
 let src,rect;
 if(CARD[id]){src=path.join(assets,'characters/forum',CARD[id]+'_hero.jpg');const [W,H]=size(src);rect=[0,0,W,Math.round(H/2)]}
 else if(BUST.includes(id)){src=path.join(assets,'characters/wiki-exception','kaiden_'+id+'.webp');const [W,H]=size(src);rect=[0,0,W,H]}
 else{src=path.join(assets,'characters/official-site',id+'.png');rect=alphaRect(src)}
 const [rx,ry,rw,rh]=rect;
 const S=Math.round(hw*rw/FILL),x=Math.round(rx+fx/100*rw-S/2),y=Math.round(ry+fy/100*rh-CY*S);
 // pad by S on every side (transparent) so a crop near an edge never fails
 execFileSync('ffmpeg',['-loglevel','error','-y','-i',src,'-vf',`format=rgba,pad=iw+${2*S}:ih+${2*S}:${S}:${S}:color=black@0,crop=${S}:${S}:${x+S}:${y+S},scale=${OUT}:${OUT}:flags=lanczos`,'-c:v','libwebp','-quality','82','-frames:v','1',path.join(outDir,id+'.webp')]);
 console.log(id.padEnd(20),`src ${path.relative(assets,src)}`,`crop ${S}px`,S<OUT?'(upscaled)':'');
}
