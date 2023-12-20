program Test;
const c := 1, cc := 2;
var v, vv, vvv;
procedure proc(x);
const inner := 10;
begin
  read(v,vv);
	v:=v+c;
	vvv:=vv+cc;
	write(v,vv,vvv);
  write(inner);
  write(x)
end
begin
	call proc(cc)
end