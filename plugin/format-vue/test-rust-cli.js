#!/usr/bin/env node

/**
 * 测试 pixly-converter 是否可用
 * 运行: node test-rust-cli.js
 */

const { spawn } = require('child_process')
const path = require('path')
const fs = require('fs')

console.log('🔍 Testing pixly-converter availability...\n')

// 获取插件根目录
const pluginRoot = path.resolve(__dirname)
console.log(`Plugin root: ${pluginRoot}\n`)

const possiblePaths = [
  path.join(pluginRoot, 'bin/pixly-converter'),
  path.join(pluginRoot, '../bin/pixly-converter'),
  path.join(pluginRoot, '../../bin/pixly-converter'),
  path.join(pluginRoot, '../../../target/release/pixly-converter'),
  path.join(pluginRoot, '../../../target/debug/pixly-converter'),
  'pixly-converter'  // 系统PATH
]

console.log('Searching paths:')
possiblePaths.forEach((p, i) => {
  const resolved = path.resolve(p)
  const exists = p === 'pixly-converter' ? '(system PATH)' : (fs.existsSync(resolved) ? '✅ exists' : '❌ not found')
  console.log(`  ${i + 1}. ${p}`)
  console.log(`     ${resolved}`)
  console.log(`     ${exists}\n`)
})

async function testPath(testPath) {
  return new Promise((resolve) => {
    console.log(`\n🧪 Testing: ${testPath}`)
    
    const proc = spawn(testPath, ['--version'], { timeout: 3000 })
    let output = ''
    let error = ''
    
    proc.stdout.on('data', (data) => {
      output += data.toString()
    })
    
    proc.stderr.on('data', (data) => {
      error += data.toString()
    })
    
    proc.on('close', (code) => {
      if (code === 0) {
        console.log(`   ✅ SUCCESS`)
        console.log(`   Version: ${output.trim()}`)
        resolve({ success: true, path: testPath, version: output.trim() })
      } else {
        console.log(`   ❌ FAILED (exit code: ${code})`)
        if (error) console.log(`   Error: ${error.trim()}`)
        resolve({ success: false, path: testPath, code, error })
      }
    })
    
    proc.on('error', (err) => {
      console.log(`   ❌ SPAWN ERROR: ${err.message}`)
      resolve({ success: false, path: testPath, error: err.message })
    })
  })
}

async function main() {
  console.log('\n' + '='.repeat(60))
  console.log('Testing all paths...')
  console.log('='.repeat(60))
  
  for (const p of possiblePaths) {
    const result = await testPath(p)
    if (result.success) {
      console.log('\n' + '='.repeat(60))
      console.log('✅ FOUND WORKING BINARY!')
      console.log('='.repeat(60))
      console.log(`Path: ${result.path}`)
      console.log(`Version: ${result.version}`)
      console.log('\nYou can use this path in your plugin.')
      return
    }
  }
  
  console.log('\n' + '='.repeat(60))
  console.log('❌ NO WORKING BINARY FOUND')
  console.log('='.repeat(60))
  console.log('\nPlease compile pixly-converter:')
  console.log('  cd /path/to/pixly')
  console.log('  cargo build --release')
  console.log('\nThen copy to:')
  console.log(`  ${path.join(pluginRoot, 'bin/pixly-converter')}`)
}

main().catch(console.error)
