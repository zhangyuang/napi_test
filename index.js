const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'napi-readfile' means native addon name is `napi-readfile`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `napi-readfile.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@napi-rs/napi-readfile-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'napi-readfile', 'yuuang-napi-readfile')
