const crypto = require("crypto");
const fs = require("fs");
const path = require("path");

const nacl = require("tweetnacl");

const { MongoClient, ServerApiVersion } = require("mongodb");

const FALLBACK = "AHQSoftwaresTestingSignatureKeyUsedDuringFallback123";

const MASTER_SECRET_SEED = process.env.SEED || FALLBACK;

if (MASTER_SECRET_SEED == FALLBACK) {
  console.log("--------------------------------------------------");
  console.log("!!! WARNING: Using Fallback Seed !!!");
  console.log("The integrity of this key relies on the secrecy of the seed. ");
  console.log(
    "For production, set process.env.SEED to a secure, complex value."
  );
  console.log("--------------------------------------------------");
}

const APP_VERSION = process.env.VERSION || "v0.1.1";

function deriveDeterministicSeed(masterSeed, appVersion) {
  const inputString = `${masterSeed}_${appVersion}`;

  const derivedSeed = crypto.createHash("sha256").update(inputString).digest();

  return derivedSeed;
}

function generateDeterministicKeyPair() {
  const seedBuffer = deriveDeterministicSeed(MASTER_SECRET_SEED, APP_VERSION);

  try {
    const { publicKey, secretKey } = nacl.sign.keyPair.fromSeed(seedBuffer);

    return { publicKey, privateKey: secretKey };
  } catch (error) {
    console.error("Error generating key pair:", error.message);
    process.exit(1);
  }
}

const keys = generateDeterministicKeyPair();

console.log("Private Key written in auth module");
fs.writeFileSync(
  path.join(__dirname, "../server/src/auth/key.bin"),
  keys.privateKey
);

const base64KeyBuf = keys.publicKey;

const base64Key = Buffer.from(base64KeyBuf).toString("base64");

console.log(`PUBLIC KEY BUFFER (BASE64)

Orig Length: ${base64KeyBuf.byteLength}

${"-".repeat(44)}
${base64Key}
${"-".repeat(44)}
`);

async function run() {
  const client = new MongoClient(process.env.MONGODB, {
    serverApi: {
      version: ServerApiVersion.v1,
      strict: true,
      deprecationErrors: true,
    },
  });

  try {
    const twoMonthsInMSeconds = 60 * 24 * 60 * 60 * 1000; // Approx. 60 days
    const futureDate = new Date(Date.now() + twoMonthsInMSeconds);

    await client.connect();

    const keys = client.db("keys").collection("keys");

    const key = await keys.findOne({
      _id: APP_VERSION,
    });

    if (key && process.env.ALPHA == "true") {
      console.log("KEY ALREADY EXISTS. NO NEED TO UPDATE");
      return;
    }

    if (process.env.ALPHA != "true") {
      await keys.updateMany(
        {
          expiryDate: {
            $exists: false,
          },
          _id: {
            $ne: APP_VERSION,
          },
        },
        {
          $set: {
            expiryDate: futureDate,
          },
        }
      );
    }

    if (!key) {
      await keys.insertOne({
        _id: APP_VERSION,
        key: base64Key,
      });
    }
  } catch (e) {
    console.error(e);
  } finally {
    await client.close();
  }
}

if (process.env.MONGODB) {
  console.log("Pushing to MongoDB");
  run().catch(console.dir);
}
