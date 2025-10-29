const path = require("node:path");
const fs = require("node:fs");
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
          };
        })
        .toArray()
    ).forEach((obj) => {
      initial[obj.id] = obj.key;
    });
  } catch (e) {
    console.error(e);
  } finally {
    await client.close();
  }

  fs.writeFileSync(
    path.join(__dirname, "../docs/src/public/keys.json"),
    JSON.stringify(initial, null, 2)
  );
}

run();
