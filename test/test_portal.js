/**
 * Shutri Portal E2E Puppeteer Test Suite
 * Executed by Antigravity Agents to verify UI/UX regressions deterministically.
 */

const puppeteer = require('puppeteer');
const path = require('path');

async function runPortalTests() {
    console.log('🧪 Starting Shutri Portal E2E Tests...');
    
    const browser = await puppeteer.launch({
        headless: 'new',
        args: ['--no-sandbox', '--disable-setuid-sandbox']
    });

    const page = await browser.newPage();
    const indexPath = path.resolve(__dirname, '../index.html');
    const fileUrl = `file://${indexPath}`;

    try {
        await page.goto(fileUrl, { waitUntil: 'load' });
        console.log(`✅ Loaded landing page: ${fileUrl}`);

        // ---------------------------------------------------------------------
        // Test 1: Header Brand & Title Verification
        // ---------------------------------------------------------------------
        const title = await page.title();
        if (!title.includes('Shutri')) {
            throw new Error(`Page title check failed: ${title}`);
        }
        console.log('✅ Test 1 Passed: Page title verified.');

        // ---------------------------------------------------------------------
        // Test 2: CTA Smooth Scroll Execution
        // ---------------------------------------------------------------------
        await page.click('.nav-submit-btn');
        await page.waitForTimeout ? page.waitForTimeout(500) : new Promise(r => setTimeout(r, 500));
        
        const isFormVisible = await page.evaluate(() => {
            const section = document.getElementById('submission-section');
            const rect = section.getBoundingClientRect();
            return rect.top >= 0 && rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) + 500;
        });

        if (!isFormVisible) {
            throw new Error('CTA click did not navigate to submission section.');
        }
        console.log('✅ Test 2 Passed: CTA button smooth-scrolled to submission form.');

        // ---------------------------------------------------------------------
        // Test 3: Terminal Q&A Interrogation
        // ---------------------------------------------------------------------
        await page.type('#terminal-input', 'What is Shutri?');
        await page.click('.terminal-send-btn');
        await new Promise(r => setTimeout(r, 600));

        const terminalText = await page.evaluate(() => {
            return document.getElementById('terminal-body').innerText;
        });

        if (!terminalText.includes('dm@shutri:') || !terminalText.includes('collaborative intelligence')) {
            throw new Error(`Terminal Q&A response failed. Text: ${terminalText}`);
        }
        console.log('✅ Test 3 Passed: Terminal Q&A returned deepMind (dm) answer.');

        // ---------------------------------------------------------------------
        // Test 4: Native Form Submission & Pipeline Execution
        // ---------------------------------------------------------------------
        await page.type('#submitter-name', 'Test Automator');
        await page.type('#research-title', 'E2E Test Paper');
        await page.type('#research-url', 'https://arxiv.org/abs/2401.00000');
        await page.type('#research-abstract', 'Testing automated intake pipeline execution.');

        // Intercept target window opening for GitHub issue
        const targetPromise = new Promise(resolve => browser.once('targetcreated', resolve));

        await page.click('#submit-btn');
        console.log('⏳ Triggered intake form submission...');

        await new Promise(r => setTimeout(r, 2600));

        const progressStep4Text = await page.evaluate(() => {
            return document.getElementById('progress-step-4').innerText;
        });

        if (!progressStep4Text.includes('Opening GitHub Issues portal')) {
            throw new Error(`Intake progress pipeline failed. Step 4 text: ${progressStep4Text}`);
        }
        console.log('✅ Test 4 Passed: Intake pipeline executed all 4 steps successfully.');

        console.log('\n🎉 ALL SHUTRI PORTAL E2E TESTS PASSED SUCCESSFULLY!');

    } catch (err) {
        console.error('❌ E2E Test Failure:', err.message);
        process.exitCode = 1;
    } finally {
        await browser.close();
    }
}

runPortalTests();
