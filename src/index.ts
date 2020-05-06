import debug from "debug";

const log = debug("dgraph-js-native:index");
const addon = require("../native");

log(addon.hello());
