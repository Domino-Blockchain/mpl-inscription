const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "mpl_inscription_program",
  programId: "1NSA9E2dwbXfhmvP3VnnjpT8G5R89qnyw7AkXCjhzoB",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "inscription"),
});
