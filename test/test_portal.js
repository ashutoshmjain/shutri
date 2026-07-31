/**
 * Shutri Portal E2E Multi-Device Puppeteer Test Suite
 * Headless E2E verification matched to user's exact hardware:
 * 1. Google Pixel 6a (Smartphone)
 * 2. Samsung Galaxy Tab S7 FE (Tablet)
 * 3. Desktop Browser
 */

const puppeteer = require('puppeteer');
const path = require('path');

// Exact User Hardware Profiles
const DEVICES = [
    {
        name: 'Desktop Browser (1440x900)',
        viewport: { width: 1440, height: 900, deviceScaleFactor: 1, isMobile: false, hasTouch: false }
    },
    {
        name: 'Smartphone — Google Pixel 6a (412x915)',
        viewport: { width: 412, height: 915, deviceScaleFactor: 2.625, isMobile: true, hasTouch: true }
    },
    {
        name: 'Tablet — Samsung Galaxy Tab S7 FE (800x1280)',
        viewport: { width: 800, height: 1280, deviceScaleFactor: 1.6, isMobile: true, hasTouch: true }
    }
];

async function runHardwareMatchedPortalTests() {
    console.log('📱 Starting Shutri Portal Hardware-Matched Puppeteer Test Suite...\n');
    
    const browser = await puppeteer.launch({
        headless: 'new',
        args: ['--no-sandbox', '--disable-setuid-sandbox']
    });

    const indexPath = path.resolve(__dirname, '../index.html');
    const fileUrl = `file://${indexPath}`;

    let overallSuccess = true;

    for (const dev of DEVICES) {
        console.log(`=======================================================`);
        console.log(`🔍 Testing Hardware Profile: ${dev.name}`);
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
            // Test 2: Touch Scroll to Submission Form
            // -----------------------------------------------------------------
            await page.click('.nav-submit-btn');
            await new Promise(r => setTimeout(r, 1000));

            const isFormInView = await page.evaluate(() => {
                const section = document.getElementById('submission-section');
                const rect = section.getBoundingClientRect();
                return rect.top <= window.innerHeight + 100 && rect.bottom >= 0;
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
            await new Promise(r => setTimeout(r, 1000));

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
            await page.type('#submitter-name', 'Pixel Tester');
            await page.type('#research-title', 'Hardware Matched Test');
            await page.type('#research-url', 'https://arxiv.org/abs/2401.22222');
            await page.type('#research-abstract', 'Hardware matched viewport automated test.');

            await page.click('#submit-btn');
            await new Promise(r => setTimeout(r, 2800));

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
        console.log('🎉 ALL HARDWARE-MATCHED TESTS (PIXEL 6A, GALAXY TAB S7 FE, DESKTOP) PASSED 100%! ');
    } else {
        console.error('💥 TEST SUITE ENCOUNTERED ERRORS.');
        process.exitCode = 1;
    }
}

runHardwareMatchedPortalTests();
