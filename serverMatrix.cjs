const x64suffix = [
  ["", "-C target-cpu=x86-64 -C target-feature=+avx"],
  ["x86_64-v2", "-C target-cpu=x86-64-v2"],
  ["x86_64-v3", "-C target-cpu=x86-64-v3"],
  ["x86_64-v4", "-C target-cpu=x86-64-v4"],
  // Removed unsound featureset
  [
    "x86_64-v4-amx",
    "-C target-cpu=x86-64-v4 -C target-feature=+amx-avx512,+amx-bf16,+amx-complex,+amx-fp16,+amx-fp8,+amx-int8,+amx-movrs,+amx-tf32,+amx-tile",
  ],
];

const arm64Suffix = [
  ["", "-C target-cpu=generic"],
  ["neon", "-C target-feature=+neon"],
  ["v8a", "-C target-feature=+v8a,+neon"],
  ["v9a", "-C target-feature=+v9a,+neon"],
];

const appleArmSuffix = [
  ["", ""],
  ["apple-m4", "-C target-feature=+apple-m4"],
];

const winOrMacX64Alloc = ["stdalloc", "mimalloc"];
const linuxOrMacArmAlloc = ["stdalloc", "mimalloc", "jemalloc"];

const windowsX64 = (() => {
  const target = "x86_64-pc-windows-msvc";
  const os = "windows-latest";

  const out = [];

  winOrMacX64Alloc.forEach((allocator) => {
    x64suffix.forEach((suffix) => {
      out.push({
        os,
        target,
        allocator,
        end: suffix[0],
        rustflags: suffix[1],
      });
    });
  });

  return out;
})();

const windowsArm64 = (() => {
  const target = "aarch64-pc-windows-msvc";
  const os = "windows-11-arm";

  const out = [];

  winOrMacX64Alloc.forEach((allocator) => {
    arm64Suffix.forEach((suffix) => {
      out.push({
        os,
        target,
        allocator,
        end: suffix[0],
        rustflags: suffix[1],
      });
    });
  });

  return out;
})();

const linuxX64 = (() => {
  const target = "x86_64-unknown-linux-gnu";
  const os = "ubuntu-22.04";

  const out = [];

  linuxOrMacArmAlloc.forEach((allocator) => {
    x64suffix.forEach((suffix) => {
      out.push({
        os,
        target,
        allocator,
        end: suffix[0],
        rustflags: suffix[1],
      });
    });
  });

  return out;
})();

const linuxArm64 = (() => {
  const target = "aarch64-unknown-linux-gnu";
  const os = "ubuntu-22.04-arm";

  const out = [];

  linuxOrMacArmAlloc.forEach((allocator) => {
    arm64Suffix.forEach((suffix) => {
      out.push({
        os,
        target,
        allocator,
        end: suffix[0],
        rustflags: suffix[1],
      });
    });
  });

  return out;
})();

const macX64 = (() => {
  const target = "x86_64-apple-darwin";
  const os = "macos-latest";

  const out = [];

  winOrMacX64Alloc.forEach((allocator) => {
    out.push({
      os,
      target,
      allocator,
      end: "",
      rustflags: "",
    });
  });

  return out;
})();

const macosApple = (() => {
  const target = "aarch64-apple-darwin";
  const os = "macos-latest";

  const out = [];

  linuxOrMacArmAlloc.forEach((allocator) => {
    appleArmSuffix.forEach((suffix) => {
      out.push({
        os,
        target,
        allocator,
        end: suffix[0],
        rustflags: suffix[1],
      });
    });
  });

  return out;
})();

const sum = [
  ...windowsX64,
  ...windowsArm64,
  ...linuxX64,
  ...linuxArm64,
  ...macX64,
  ...macosApple,
];

console.log(sum);

module.exports = () => {
  return sum;
};
