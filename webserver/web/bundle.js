"use strict";
(() => {
  // pkg/smith_js.js
  var import_meta = {};
  var wasm;
  var heap = new Array(128).fill(void 0);
  heap.push(void 0, null, true, false);
  function getObject(idx) {
    return heap[idx];
  }
  var heap_next = heap.length;
  function dropObject(idx) {
    if (idx < 132)
      return;
    heap[idx] = heap_next;
    heap_next = idx;
  }
  function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
  }
  var WASM_VECTOR_LEN = 0;
  var cachedUint8Memory0 = null;
  function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
      cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
  }
  var cachedTextEncoder = typeof TextEncoder !== "undefined" ? new TextEncoder("utf-8") : { encode: () => {
    throw Error("TextEncoder not available");
  } };
  var encodeString = typeof cachedTextEncoder.encodeInto === "function" ? function(arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
  } : function(arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read: arg.length,
      written: buf.length
    };
  };
  function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === void 0) {
      const buf = cachedTextEncoder.encode(arg);
      const ptr2 = malloc(buf.length) >>> 0;
      getUint8Memory0().subarray(ptr2, ptr2 + buf.length).set(buf);
      WASM_VECTOR_LEN = buf.length;
      return ptr2;
    }
    let len = arg.length;
    let ptr = malloc(len) >>> 0;
    const mem = getUint8Memory0();
    let offset = 0;
    for (; offset < len; offset++) {
      const code = arg.charCodeAt(offset);
      if (code > 127)
        break;
      mem[ptr + offset] = code;
    }
    if (offset !== len) {
      if (offset !== 0) {
        arg = arg.slice(offset);
      }
      ptr = realloc(ptr, len, len = offset + arg.length * 3) >>> 0;
      const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
      const ret = encodeString(arg, view);
      offset += ret.written;
    }
    WASM_VECTOR_LEN = offset;
    return ptr;
  }
  function isLikeNone(x) {
    return x === void 0 || x === null;
  }
  var cachedInt32Memory0 = null;
  function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
      cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
  }
  var cachedTextDecoder = typeof TextDecoder !== "undefined" ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true }) : { decode: () => {
    throw Error("TextDecoder not available");
  } };
  if (typeof TextDecoder !== "undefined") {
    cachedTextDecoder.decode();
  }
  function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
  }
  function addHeapObject(obj) {
    if (heap_next === heap.length)
      heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];
    heap[idx] = obj;
    return idx;
  }
  function debugString(val) {
    const type = typeof val;
    if (type == "number" || type == "boolean" || val == null) {
      return `${val}`;
    }
    if (type == "string") {
      return `"${val}"`;
    }
    if (type == "symbol") {
      const description = val.description;
      if (description == null) {
        return "Symbol";
      } else {
        return `Symbol(${description})`;
      }
    }
    if (type == "function") {
      const name = val.name;
      if (typeof name == "string" && name.length > 0) {
        return `Function(${name})`;
      } else {
        return "Function";
      }
    }
    if (Array.isArray(val)) {
      const length = val.length;
      let debug = "[";
      if (length > 0) {
        debug += debugString(val[0]);
      }
      for (let i = 1; i < length; i++) {
        debug += ", " + debugString(val[i]);
      }
      debug += "]";
      return debug;
    }
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
      className = builtInMatches[1];
    } else {
      return toString.call(val);
    }
    if (className == "Object") {
      try {
        return "Object(" + JSON.stringify(val) + ")";
      } catch (_) {
        return "Object";
      }
    }
    if (val instanceof Error) {
      return `${val.name}: ${val.message}
${val.stack}`;
    }
    return className;
  }
  function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
  }
  function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
  }
  function handleError(f, args) {
    try {
      return f.apply(this, args);
    } catch (e) {
      wasm.__wbindgen_exn_store(addHeapObject(e));
    }
  }
  var SmithJS = class {
    static __wrap(ptr) {
      ptr = ptr >>> 0;
      const obj = Object.create(SmithJS.prototype);
      obj.__wbg_ptr = ptr;
      return obj;
    }
    __destroy_into_raw() {
      const ptr = this.__wbg_ptr;
      this.__wbg_ptr = 0;
      return ptr;
    }
    free() {
      const ptr = this.__destroy_into_raw();
      wasm.__wbg_smithjs_free(ptr);
    }
    constructor(src) {
      const ptr0 = passStringToWasm0(src, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      const ret = wasm.smithjs_new(ptr0, len0);
      return SmithJS.__wrap(ret);
    }
    serialize(json, typename) {
      try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(typename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.smithjs_serialize(retptr, this.__wbg_ptr, addHeapObject(json), ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        if (r3) {
          throw takeObject(r2);
        }
        var v2 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v2;
      } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
      }
    }
    deserialize(bin, typename) {
      try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(bin, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(typename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.smithjs_deserialize(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        if (r2) {
          throw takeObject(r1);
        }
        return takeObject(r0);
      } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
      }
    }
  };
  async function __wbg_load(module, imports) {
    if (typeof Response === "function" && module instanceof Response) {
      if (typeof WebAssembly.instantiateStreaming === "function") {
        try {
          return await WebAssembly.instantiateStreaming(module, imports);
        } catch (e) {
          if (module.headers.get("Content-Type") != "application/wasm") {
            console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
          } else {
            throw e;
          }
        }
      }
      const bytes = await module.arrayBuffer();
      return await WebAssembly.instantiate(bytes, imports);
    } else {
      const instance = await WebAssembly.instantiate(module, imports);
      if (instance instanceof WebAssembly.Instance) {
        return { instance, module };
      } else {
        return instance;
      }
    }
  }
  function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
      takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "string" ? obj : void 0;
      var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      var len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
      const ret = getStringFromWasm0(arg0, arg1);
      return addHeapObject(ret);
    };
    imports.wbg.__wbg_parse_4457078060869f55 = function() {
      return handleError(function(arg0, arg1) {
        const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_stringify_9003c389758d16d4 = function() {
      return handleError(function(arg0) {
        const ret = JSON.stringify(getObject(arg0));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
      const ret = new Error();
      return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
      const ret = getObject(arg1).stack;
      const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
      let deferred0_0;
      let deferred0_1;
      try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
      } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1);
      }
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
      const ret = debugString(getObject(arg1));
      const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    };
    return imports;
  }
  function __wbg_init_memory(imports, maybe_memory) {
  }
  function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;
    wasm.__wbindgen_start();
    return wasm;
  }
  async function __wbg_init(input) {
    if (wasm !== void 0)
      return wasm;
    if (typeof input === "undefined") {
      input = new URL("smith_js_bg.wasm", import_meta.url);
    }
    const imports = __wbg_get_imports();
    if (typeof input === "string" || typeof Request === "function" && input instanceof Request || typeof URL === "function" && input instanceof URL) {
      input = fetch(input);
    }
    __wbg_init_memory(imports);
    const { instance, module } = await __wbg_load(await input, imports);
    return __wbg_finalize_init(instance, module);
  }
  var smith_js_default = __wbg_init;

  // smith_types.ts
  var WebMovement = class {
    tag;
    val;
    constructor(tag, val) {
      this.tag = tag;
      this.val = val;
    }
    static Forward() {
      return new WebMovement("Forward");
    }
    as_Forward() {
      if (this.tag != "Forward") {
        throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Forward'");
      }
    }
    static Stop() {
      return new WebMovement("Stop");
    }
    as_Stop() {
      if (this.tag != "Stop") {
        throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Stop'");
      }
    }
    static Left() {
      return new WebMovement("Left");
    }
    as_Left() {
      if (this.tag != "Left") {
        throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Left'");
      }
    }
    static Right() {
      return new WebMovement("Right");
    }
    as_Right() {
      if (this.tag != "Right") {
        throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Right'");
      }
    }
    static Speed(v) {
      return new WebMovement("Speed", v);
    }
    as_Speed() {
      if (this.tag != "Speed") {
        throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Speed'");
      }
      return this.val;
    }
    getTag() {
      return this.tag;
    }
  };
  var WebPacket = class {
    tag;
    val;
    constructor(tag, val) {
      this.tag = tag;
      this.val = val;
    }
    static LidarScan(v) {
      return new WebPacket("LidarScan", v);
    }
    as_LidarScan() {
      if (this.tag != "LidarScan") {
        throw new Error("Enum WebPacket: trying to cast variant '" + this.tag + "' into 'LidarScan'");
      }
      return this.val;
    }
    static IMURotation(v) {
      return new WebPacket("IMURotation", v);
    }
    as_IMURotation() {
      if (this.tag != "IMURotation") {
        throw new Error("Enum WebPacket: trying to cast variant '" + this.tag + "' into 'IMURotation'");
      }
      return this.val;
    }
    static Movement(v) {
      return new WebPacket("Movement", v);
    }
    as_Movement() {
      if (this.tag != "Movement") {
        throw new Error("Enum WebPacket: trying to cast variant '" + this.tag + "' into 'Movement'");
      }
      return this.val;
    }
    getTag() {
      return this.tag;
    }
  };

  // coordinatesystem.ts
  var CoordinateSystem = class {
    width;
    height;
    ctx;
    zoomFactor = 1;
    points = [];
    constructor(canvas) {
      this.ctx = canvas.getContext("2d");
      this.width = canvas.width;
      this.height = canvas.height;
      canvas.addEventListener("wheel", (ev) => {
        this.zoomFactor += ev.deltaY * 1e-4;
        if (this.zoomFactor < 0) {
          this.zoomFactor = 0;
        }
        this.render(this.points, 0);
      });
      this.setZoom(0.2);
    }
    render(points, rot) {
      this.points = points;
      this.ctx.clearRect(0, 0, this.width, this.height);
      this.ctx.fillStyle = "blue";
      const pointSize = 1;
      points.forEach((point) => {
        const transformedX = this.transformCoordinate(point.x, "x");
        const transformedY = this.transformCoordinate(point.y, "y");
        this.ctx.beginPath();
        this.ctx.arc(transformedX, transformedY, pointSize, 0, Math.PI * 2, true);
        this.ctx.fill();
      });
      this.ctx.fillText(`zoom: ${Math.round(this.zoomFactor * 100) / 100}`, 10, 20);
      this.ctx.fillStyle = "green";
      this.ctx.beginPath();
      let x = this.transformCoordinate(0, "x");
      let y = this.transformCoordinate(0, "y");
      this.ctx.arc(x, y, 5, 0, Math.PI * 2, true);
      this.ctx.fill();
      let rotpi = rot / 180 * Math.PI;
      let len = 25;
      let x2 = -Math.cos(rotpi) * len + x;
      let y2 = Math.sin(rotpi) * len + y;
    }
    transformCoordinate(coordinate, axis) {
      const range = 24 * this.zoomFactor;
      const midPoint = range / 2;
      const canvasSize = axis === "x" ? this.width : this.height;
      const canvasMid = canvasSize / 2;
      let scaledCoordinate = (coordinate + midPoint) / range * canvasSize;
      return scaledCoordinate;
    }
    setZoom(zoomFactor) {
      this.zoomFactor = zoomFactor;
    }
  };

  // main.ts
  console.log("Starting");
  async function main() {
    await smith_js_default("./pkg/smith_js_bg.wasm");
    let smith = new SmithJS(await (await fetch("/schema.smith")).text());
    const proto = location.protocol.startsWith("https") ? "wss" : "ws";
    const wsUri = `${proto}://${location.host}/ws`;
    let ws = new WebSocket(wsUri);
    let canv = document.getElementById("lidarScan");
    let coordinatesystem = new CoordinateSystem(canv);
    function sendPacket(packet) {
      ws.send(smith.serialize(packet, "WebPacket"));
    }
    let last = "_";
    document.body.addEventListener("keyup", (ev) => {
      last = "_";
      sendPacket(WebPacket.Movement(WebMovement.Stop()));
    });
    document.body.addEventListener("keydown", (ev) => {
      let k = ev.key;
      if (k == last) {
        return;
      }
      last = k;
      if (k == "w") {
        sendPacket(WebPacket.Movement(WebMovement.Forward()));
      } else if (k == "a") {
        sendPacket(WebPacket.Movement(WebMovement.Left()));
      } else if (k == "s") {
        sendPacket(WebPacket.Movement(WebMovement.Stop()));
      } else if (k == "d") {
        sendPacket(WebPacket.Movement(WebMovement.Right()));
      }
    });
    let rotdisp = document.getElementById("rotdisplay");
    let rotations = [];
    let rot = 0;
    ws.onmessage = async (msg) => {
      let data = msg.data;
      if (data instanceof Blob) {
        let binary = new Uint8Array(await data.arrayBuffer());
        let packet = smith.deserialize(binary, "WebPacket");
        Object.setPrototypeOf(packet, WebPacket.prototype);
        if (packet.getTag() == "LidarScan") {
          let lidarscan = packet.as_LidarScan();
          let pts = lidarscan.points;
          let time_ms = Number.parseInt(lidarscan.time);
          if (rotations.length > 0) {
            let min = rotations[0];
            let mindiff = Math.abs(min.time - time_ms);
            for (let i = 1; i < rotations.length; i++) {
              let loc = rotations[i];
              let diff = Math.abs(loc.time - time_ms);
              if (diff < mindiff) {
                mindiff = diff;
                min = loc;
              }
            }
            rot = min.rotation;
          }
          rotdisp.innerText = rot + "";
          coordinatesystem.render(lidarscan.points, rot);
        } else if (packet.getTag() == "IMURotation") {
          let pack = packet.as_IMURotation();
          rotations.push({ rotation: pack.rotation, time: Number.parseInt(pack.time) });
          if (rotations.length > 20) {
            rotations = [...rotations.slice(1)];
          }
        }
      }
    };
  }
  main();
})();
