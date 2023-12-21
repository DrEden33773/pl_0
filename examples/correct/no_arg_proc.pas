program Test;
const c:=1, cc:=2;
var v,vv,vvv,cnt;
procedure proc();
const inner:=10;
begin
	v:=v+c;
	vvv:=vv+cc;
	write(v,vv,vvv);
  write(inner);
  while cnt<10 do begin
    v:=v+1;
    vv:=vv+1;
    cnt:=cnt+1;
    call proc()
  end
end
begin
  read(v,vv);
  cnt:=0;
	call proc()
end