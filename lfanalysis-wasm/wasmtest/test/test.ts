// var lfw = require('lfanalysis-wasm')
// import * as lfw from 'lfanalysis-wasm'

var assert = require('assert');
describe('lfw', function () {
  describe('lfc_json', function () {
    it('should produce json from physical file without panicking', async function () {
      const lfw = await import('lfanalysis-wasm')
      Object.keys(lfw).forEach((prop)=> console.log("lfw property: " + prop));
      assert.equal(lfw.lfc_json("/home/peter/xronos/editor-support/lfanalysis-wasm/wasmtest/lf/src/minimal.lf"), "");
    });
  });
});
