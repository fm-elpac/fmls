// sled/ihex_util.js
//
// Intel HEX 校验码计算
//
// 用法:
//
// ```js
// import { c } from './ihex_util.js';
// c('00000001');  // 'FF'
// ```

// 计算 Intel HEX 一行的校验码
export function c(text) {
  let a = read_text(text);
  let o = c_add(a);
  return o.toString(16).toUpperCase();
}

// 输入一个字节值数组, 输出校验值
function c_add(a) {
  let s = 0;
  // 所有字节的值加起来
  for (let i of a) {
    s += i;
  }
  // 取低 8 位, 取反, +1
  return ((s & 0xff) ^ 0xff) + 1;
}

// 将输入的字符串解析为字节值数组
function read_text(t) {
  let o = [];
  let i = 0;
  while (i < t.length) {
    o.push(Number.parseInt(t.slice(i, i + 2), 16));
    i += 2;
  }
  return o;
}
