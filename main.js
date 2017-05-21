var mallocFloat32 = function(len) {
  var bufsize = len * 4;
  var bufptr = Module._malloc(bufsize);
  Module._memset(bufptr, 0, bufsize)
  var buf = new Float32Array(Module.HEAPF32.buffer, bufptr, len);
  return {ptr: bufptr, buf: buf};
}

window.callHello = function() {
  const helloFn = Module.cwrap('hello', null, ['number', 'number']);

  var len = 5;
  var mem = mallocFloat32(len);
  var buf = mem.buf;
  for (var i = 0; i < len;i ++){
    buf[i] = 10.1 * i;
  }

  helloFn(len, buf.byteOffset);

  Module._free(mem.ptr);
  console.log("end");
}

window.callPerson = function() {
  const init = Module.cwrap('init_person', 'number', ['number']);
  var tom = init(29);
  console.log(tom);
  const print = Module.cwrap('print_person', null, ['number']);
  print(tom);
  console.log("end");
}
