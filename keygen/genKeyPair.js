const crypto = require("node:crypto");

const privateKey =
  "MC4CAQAwBQYDK2VwBCIEIH7oDXljJcF7BxSrHjwJDR/1CLJ9JDbmGaNeMYzW4eWs";

const privateKeyObject = crypto.createPrivateKey({
  key: Buffer.from(privateKey, "base64"),
  format: "der",
  type: "pkcs8",
});
console.log(
  crypto.sign(null, "Hello World", privateKeyObject).toString("base64")
);
