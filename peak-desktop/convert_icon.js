
import sharp from 'sharp';
import fs from 'fs';
import path from 'path';

const inputFile = process.argv[2];
const outputDir = process.argv[3];

if (!inputFile || !outputDir) {
    console.error('Usage: node convert_icon.js <input> <outputDir>');
    process.exit(1);
}

async function convert() {
    try {
        const sizes = [32, 128];
        const image = sharp(inputFile).ensureAlpha();

        // 32x32
        await image
            .resize(32, 32)
            .toFormat('png')
            .toFile(path.join(outputDir, '32x32.png'));

        // 128x128
        await image
            .resize(128, 128)
            .toFormat('png')
            .toFile(path.join(outputDir, '128x128.png'));

        // 128x128@2x (256x256)
        await image
            .resize(256, 256)
            .toFormat('png')
            .toFile(path.join(outputDir, '128x128@2x.png'));

        // icon.png (512x512) for good measure
        await image
            .resize(512, 512)
            .toFormat('png')
            .toFile(path.join(outputDir, 'icon.png'));

        console.log('Icons generated successfully in RGBA format.');
    } catch (err) {
        console.error('Error processing image:', err);
        process.exit(1);
    }
}

convert();
