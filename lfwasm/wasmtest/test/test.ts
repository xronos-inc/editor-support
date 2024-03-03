const assert = require('assert');
import * as fs from "fs";

const fsReadCapability = (p: string) => fs.readFileSync(p, {encoding: "utf-8"})

describe('lfw', function () {
  describe('lfc_json', function () {
    it('should produce json from physical file without panicking', async function () {
      const lfw = await import('lfwasm')
      lfw.init();
      assert.equal(lfw.lfc_json("./lf/src/minimal.lf", fsReadCapability), '{"out":"./lf/target","properties":{},"src":"./lf/src/minimal.lf"}');
    });
  });
});
