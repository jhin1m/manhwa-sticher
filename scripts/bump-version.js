#!/usr/bin/env node

/**
 * Script to bump version across all necessary files
 * Usage: node scripts/bump-version.js <new-version>
 * Example: node scripts/bump-version.js 0.2.0
 */

import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

const newVersion = process.argv[2];

if (!newVersion) {
  console.error('❌ Error: Please provide a version number');
  console.log('Usage: node scripts/bump-version.js <version>');
  console.log('Example: node scripts/bump-version.js 0.2.0');
  process.exit(1);
}

// Validate version format (basic check)
if (!/^\d+\.\d+\.\d+(-[a-z0-9.]+)?$/i.test(newVersion)) {
  console.error('❌ Error: Invalid version format');
  console.log('Version should be in format: X.Y.Z or X.Y.Z-beta.1');
  process.exit(1);
}

const files = [
  {
    path: 'package.json',
    update: (content) => {
      const pkg = JSON.parse(content);
      pkg.version = newVersion;
      return JSON.stringify(pkg, null, 2) + '\n';
    }
  },
  {
    path: 'src-tauri/tauri.conf.json',
    update: (content) => {
      const config = JSON.parse(content);
      config.version = newVersion;
      return JSON.stringify(config, null, 2) + '\n';
    }
  },
  {
    path: 'src-tauri/Cargo.toml',
    update: (content) => {
      return content.replace(
        /^version = ".*"$/m,
        `version = "${newVersion}"`
      );
    }
  }
];

console.log(`🚀 Bumping version to ${newVersion}...\n`);

for (const file of files) {
  try {
    const filePath = join(rootDir, file.path);
    const content = readFileSync(filePath, 'utf-8');
    const updated = file.update(content);
    writeFileSync(filePath, updated, 'utf-8');
    console.log(`✅ Updated ${file.path}`);
  } catch (error) {
    console.error(`❌ Error updating ${file.path}:`, error.message);
    process.exit(1);
  }
}

console.log(`\n✨ Successfully bumped version to ${newVersion}!`);
console.log('\nNext steps:');
console.log('1. Review the changes');
console.log(`2. git add .`);
console.log(`3. git commit -m "chore: bump version to ${newVersion}"`);
console.log(`4. git tag v${newVersion}`);
console.log(`5. git push origin main`);
console.log(`6. git push origin v${newVersion}`);
