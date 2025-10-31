const path = require("node:path");
const fs = require("node:fs");
const crypto = require("node:crypto");

const { MongoClient, ServerApiVersion } = require("mongodb");

const client = new MongoClient(process.env.MONGODB, {
  serverApi: {
    version: ServerApiVersion.v1,
    strict: true,
    deprecationErrors: true,
  },
});

let initial = {
  "??": {
    intent: "A list of pre computed keys for AHQ AI Server Binaries",
    security:
      "The methodology is inherently insecure but sets the bar quite high to detect UNOFFICIAL ahq ai server builds with very high accuracy",
  },
};

async function run() {
  try {
    await client.connect();

    (
      await client
        .db("keys")
        .collection("keys")
        .find()
        .map((data) => {
          return {
            id: data._id,
            key: data.key,
            /**
             * @type {Date}
             */
            expiryDate: data.expiryDate,
          };
        })
        .toArray()
    ).forEach((obj) => {
      initial[obj.id] = {
        pubkey: obj.key,
        expiry: obj.expiryDate?.toUTCString(),
      };
    });
  } catch (e) {
    console.error(e);
  } finally {
    await client.close();
  }

  const output = JSON.stringify(initial, null, 2);

  const privateKeyObject = crypto.createPrivateKey({
    key: Buffer.from(process.env.KEY_FILE_INTEGRITY, "base64"),
    format: "der",
    type: "pkcs8",
  });

  fs.writeFileSync(
    path.join(__dirname, "../docs/src/public/keys.json"),
    output
  );

  const signer = crypto.createSign(); // Note: digest is omitted here, or use 'null'
  signer.update(output);
  signer.end();

  const outbuffer = signer.sign(privateKeyObject);

  fs.writeFileSync(
    path.join(__dirname, "../docs/src/public/keys.integrity"),
    outbuffer
  );
}

run();
