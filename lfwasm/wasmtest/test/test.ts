const assert = require('assert');
import * as fs from "fs";
import * as path from "path";

const fsReadCapability = (p: string) => fs.readFileSync(p, {encoding: "utf-8"})

describe('lfw', function () {
  describe('lfc_json', function () {
    it('should produce json from physical file without panicking', async function () {
      const lfw = await import('lfwasm')
      lfw.init();
      Object.keys(lfw).forEach((prop)=> console.log("lfw property: " + prop));
      assert.equal(lfw.lfc_json("/home/peter/xronos/editor-support/lfwasm/wasmtest/lf/src/minimal.lf", fsReadCapability), "");
    });
  });
});
