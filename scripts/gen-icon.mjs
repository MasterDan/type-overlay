import zlib from "node:zlib";
import { writeFileSync } from "node:fs";

const SIZE = 1024;
const buf = Buffer.alloc(SIZE * SIZE * 4);

const set = (x, y, [r, g, b, a = 255]) => {
  if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) return;
  const i = (y * SIZE + x) * 4;
  buf[i] = r;
  buf[i + 1] = g;
  buf[i + 2] = b;
  buf[i + 3] = a;
};

const roundedRect = (x0, y0, x1, y1, radius, color) => {
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const cx = Math.min(Math.max(x, x0 + radius), x1 - 1 - radius);
      const cy = Math.min(Math.max(y, y0 + radius), y1 - 1 - radius);
      const dx = x - cx;
      const dy = y - cy;
      if (dx * dx + dy * dy <= radius * radius) set(x, y, color);
    }
  }
};

// background
roundedRect(0, 0, SIZE, SIZE, 232, [15, 23, 42, 255]);

// keyboard body
const bodyX = 150;
const bodyY = 320;
const bodyW = SIZE - bodyX * 2;
const bodyH = 420;
roundedRect(bodyX, bodyY, bodyX + bodyW, bodyY + bodyH, 56, [30, 41, 59, 255]);

// key grid
const cols = 10;
const rows = 3;
const pad = 40;
const gap = 18;
const keyW = (bodyW - pad * 2 - gap * (cols - 1)) / cols;
const keyH = (bodyH - pad * 2 - gap * (rows - 1)) / rows;
for (let r = 0; r < rows; r++) {
  for (let c = 0; c < cols; c++) {
    const kx = bodyX + pad + c * (keyW + gap);
    const ky = bodyY + pad + r * (keyH + gap);
    const accent = r === 1 && c === 4;
    roundedRect(
      Math.round(kx),
      Math.round(ky),
      Math.round(kx + keyW),
      Math.round(ky + keyH),
      16,
      accent ? [56, 189, 248, 255] : [71, 85, 105, 255],
    );
  }
}

// encode PNG
const crcTable = (() => {
  const t = new Uint32Array(256);
  for (let n = 0; n < 256; n++) {
    let c = n;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    t[n] = c >>> 0;
  }
  return t;
})();

const crc32 = (data) => {
  let c = 0xffffffff;
  for (let i = 0; i < data.length; i++) c = crcTable[(c ^ data[i]) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
};

const chunk = (type, data) => {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length, 0);
  const typeBuf = Buffer.from(type, "ascii");
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(Buffer.concat([typeBuf, data])), 0);
  return Buffer.concat([len, typeBuf, data, crc]);
};

const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(SIZE, 0);
ihdr.writeUInt32BE(SIZE, 4);
ihdr[8] = 8; // bit depth
ihdr[9] = 6; // color type RGBA
ihdr[10] = 0;
ihdr[11] = 0;
ihdr[12] = 0;

const raw = Buffer.alloc((SIZE * 4 + 1) * SIZE);
for (let y = 0; y < SIZE; y++) {
  raw[y * (SIZE * 4 + 1)] = 0; // filter none
  buf.copy(raw, y * (SIZE * 4 + 1) + 1, y * SIZE * 4, (y + 1) * SIZE * 4);
}
const idat = zlib.deflateSync(raw, { level: 9 });

const png = Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk("IHDR", ihdr),
  chunk("IDAT", idat),
  chunk("IEND", Buffer.alloc(0)),
]);

writeFileSync(new URL("../app-icon.png", import.meta.url), png);
console.log("app-icon.png generated", png.length, "bytes");
