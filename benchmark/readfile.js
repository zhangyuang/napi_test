const fs = require('fs')
const path = require('path')
const { Suite } = require('benchmark')
const { readfileReverse } = require('../napi-readfile.darwin.node')

const suite = new Suite()
const filePath = path.resolve(__dirname, '../index.js')

suite
  .add('readfile by nodejs', () => {
    fs.readFileSync(filePath).toString().split('').reverse().join('')
  })
  .add('readfile by napi', () => {
    readfileReverse(filePath)
  })
  .on('cycle', function (event) {
    //eslint-disable-next-line
    console.log(String(event.target))
  })
  .on('complete', function () {
    //eslint-disable-next-line
    console.log('Fastest is ' + this.filter('fastest').map('name'))
  })
  // run async
  .run({ async: true })
