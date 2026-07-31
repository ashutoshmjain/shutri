/**
 * Shutri Portal E2E Multi-Device Puppeteer Test Suite
 * Headless E2E verification across Desktop, Smartphone (iPhone 14 Pro), and Tablet (iPad Pro).
 */

const puppeteer = require('puppeteer');
const path = require('path');

// Device Viewport Configurations
const DEVICES = [
    {
        name: 'Desktop Browser (1440x900)',
        viewport: { width: 1440, height: 900, deviceScaleFactor: 1, isMobile: false, hasTouch: false }
    },
    {
        name: 'Smartphone — iPhone 14 Pro (393x852)',
        viewport: { width: 393, height: 852, deviceScaleFactor: 3, isMobile: true, hasTouch: true }
    },
    {
        name: 'Tablet — iPad Pro 11 (834x1194)',
        viewport: { width: 834, height: 1194, deviceScaleFactor: 2, isMobile: true, hasTouch: true }
    }
];

async function runMultiDevicePortalTests() {
    console.log('📱 Starting Shutri Portal Multi-Device Puppeteer Test Suite...\n');
    
    const browser = await puppeteer.launch({
        headless: 'new',
        args: ['--no-sandbox', '--disable-setuid-sandbox']
    });

    const indexPath = path.resolve(__dirname, '../index.html');
    const fileUrl = `file://${indexPath}`;

    let overallSuccess = true;

    for (const dev of DEVICES) {
        console.log(`=======================================================`);
        console.log(`🔍 Testing Device Profile: ${dev.name}`);
        console.log(`=======================================================`);

        const page = await browser.newPage();
        await page.setViewport(dev.viewport);

        try {
            await page.goto(fileUrl, { waitUntil: 'load' });

            // -----------------------------------------------------------------
            // Test 1: Layout & Title Verification
            // -----------------------------------------------------------------
            const title = await page.title();
            if (!title.includes('Shutri')) {
                throw new Error(`Title check failed on ${dev.name}`);
            }
            console.log(`  ✅ [${dev.name}] Layout & Title loaded cleanly.`);

            // -----------------------------------------------------------------
            // Test 2: Mobile/Tablet Touch Scroll to Submission Form
            // -----------------------------------------------------------------
            await page.click('.nav-submit-btn');
            await new Promise(r => setTimeout(r, 600));

            const isFormInView = await page.evaluate(() => {
                const section = document.getElementById('submission-section');
                const rect = section.getBoundingClientRect();
                return rect.top >= 0 && rect.top <= window.innerHeight;
            });

            if (!isFormInView) {
                throw new Error(`Smooth-scroll to submission form failed on ${dev.name}`);
            }
            console.log(`  ✅ [${dev.name}] Navigation CTA smooth-scrolled to submission form.`);

            // -----------------------------------------------------------------
            // Test 3: Responsive Terminal Q&A Touch Interaction
            // -----------------------------------------------------------------
            await page.type('#terminal-input', 'What is Shutri?');
            await page.click('.terminal-send-btn');
            await new Promise(r => setTimeout(r, 600));

            const terminalOutput = await page.evaluate(() => {
                return document.getElementById('terminal-body').innerText;
            });

            if (!terminalOutput.includes('dm@shutri:')) {
                throw new Error(`Terminal Q&A touch interaction failed on ${dev.name}`);
            }
            console.log(`  ✅ [${dev.name}] Terminal Q&A responded successfully.`);

            // -----------------------------------------------------------------
            // Test 4: Mobile Intake Pipeline Execution
            // -----------------------------------------------------------------
            await page.type('#submitter-name', 'Mobile Tester');
            await page.type('#research-title', 'Mobile Intake Test');
            await page.type('#research-url', 'https://arxiv.org/abs/2401.11111');
            await page.type('#research-abstract', 'Mobile viewport automated pipeline test.');

            await page.click('#submit-btn');
            await new Promise(r => setTimeout(r, 2600));

            const step4Text = await page.evaluate(() => {
                return document.getElementById('progress-step-4').innerText;
            });

            if (!step4Text.includes('Opening GitHub Issues portal')) {
                throw new Error(`Intake progress pipeline failed on ${dev.name}`);
            }
            console.log(`  ✅ [${dev.name}] Native intake form & 4-step pipeline passed.\n`);

        } catch (err) {
            console.error(`  ❌ [${dev.name}] FAILURE:`, err.message, '\n');
            overallSuccess = false;
        } finally {
            await page.close();
        }
    }

    await browser.close();

    if (overallSuccess) {
        console.log('🎉 ALL MULTI-DEVICE TESTS (DESKTOP, SMARTPHONE, TABLET) PASSED SUCCESSFULLY!');
    } else {
        console.error('💥 MULTI-DEVICE TEST SUITE ENCOUNTERED ERRORS.');
        process.exitCode = 1;
    }
}

runMultiDevicePortalTests();
