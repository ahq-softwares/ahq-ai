const crypto = require("crypto");
const fs = require("fs");
const path = require("path");

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

  // IMPORTANT NOTE:
  // The built-in Node.js crypto API does NOT support deterministic RSA generation via a seed.
  // It is typically used for generating modern keys like Ed25519 (an ECC algorithm) which
  // is a common and secure replacement for RSA in many signing contexts and is fully deterministic.
  // By using the 'seed' option below, the generated key pair will be IDENTICAL every time
  // this function is run with the same MASTER_SECRET_SEED and APP_VERSION.

  try {
    const { publicKey } = crypto.generateKeyPairSync("ed25519", {
      seed: seedBuffer,

      publicKeyEncoding: {
        type: "spki",
        format: "der",
      },
      // privateKeyEncoding: {
      //   type: "pkcs8",
      //   format: "der",
      // },
    });

    return { publicKey, privateKey: seedBuffer };
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

const buf = Buffer.from(keys.publicKey);
const base64Key = buf.subarray(buf.length - 32).toString("base64");

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
        },
        {
          $set: {
            expiryDate: futureDate,
          },
        }
      );
    }

    await keys.insertOne({
      _id: APP_VERSION,
      key: base64Key,
    });
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
