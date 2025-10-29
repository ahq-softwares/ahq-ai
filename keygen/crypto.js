const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
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

const APP_VERSION = process.env.VERSION || "v0.1.0";

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

const base64Key = Buffer.from(keys.publicKey).toString("base64");

console.log("\n" + "-".repeat(50));
console.log("GENERATED PUBLIC KEY (SAFE TO SHARE)");
console.log("-".repeat(50));
console.log(base64Key);
console.log("-".repeat(50));

const keysJson = JSON.parse(
  fs.readFileSync(path.join(__dirname, "../docs/src/public/keys.json"))
);

keysJson[APP_VERSION] = base64Key;

fs.writeFileSync(
  path.join(__dirname, "../docs/src/public/keys.json"),
  JSON.stringify(keysJson, null, 2)
);
