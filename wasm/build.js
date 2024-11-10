// run `node build.js` to build the wasm package
const wasmBuildDir = './rust/target/wasm32-unknown-unknown/release/xlsx_diff_wasm.wasm'
const pkgDir = './env'
const targetList = ['web', 'nodejs', 'bundler']
// const { exec } = require('child_process')
import { exec } from 'child_process'
console.log('Building wasm package...')
const build = async (targetList) => {
    return new Promise(async (resolve, reject) => {
        try {
            await buildWasm()
            for (const target of targetList) {
                await bindgenBuildTarget(target)
            }
            console.log('Wasm package built successfully!')
            resolve()
        } catch (err) {
            console.error(err)
            reject(err)
        }
    })
}
const buildWasm = async () => {
    return new Promise((resolve, reject) => {
        console.log('Building wasm...')
        exec('cd ./rust && cargo build --release --target wasm32-unknown-unknown', (err, stdout, stderr) => {
            if (err) {
                console.error(err)
                reject(err)
            }
            console.log(stdout)
            console.error(stderr)
            resolve()
        })
    })
}
const bindgenBuildTarget = async (target) => {
    return new Promise((resolve, reject) => {
        console.log(`Building wasm-bindgen for ${target}...`)
        exec(`wasm-bindgen ${wasmBuildDir} --out-dir ${pkgDir}/${target} --target ${target} --out-name index`, (err, stdout, stderr) => {
            if (err) {
                console.error(err)
                reject(err)
            }
            console.log(stdout)
            console.error(stderr)
            resolve()
        })
    })
}
build(targetList)
