window.callWasm = function() {
  const helloFn = Module.cwrap('hello', null, ['number', 'number']);
  var len = 5;
  var bufsize = len * 4;
  var bufptr = Module._malloc(bufsize);
  Module._memset(bufptr, 0, bufsize)
  var buf = new Float32Array(Module.HEAPF32.buffer, bufptr, len);
  for (var i = 0; i < len;i ++){
    buf[i] = 10.1 * i;
  }
  helloFn(len, buf.byteOffset);
  Module._free(bufptr);
}
