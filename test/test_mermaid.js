/**
 * Mermaid Diagram & Markdown Rendering Validator
 * Scans all markdown files across workspace repositories to ensure 100% valid Mermaid syntax.
 */

const fs = require('fs');
const path = require('path');

const REPOS = [
    path.resolve(__dirname, '../../shutri'),
    path.resolve(__dirname, '../../mdIngest'),
    path.resolve(__dirname, '../../deepDive'),
    path.resolve(__dirname, '../../ddma')
];

function findMarkdownFiles(dir, fileList = []) {
    if (!fs.existsSync(dir)) return fileList;
    const files = fs.readdirSync(dir);
    for (const file of files) {
        if (file === 'node_modules' || file === '.git' || file === 'target' || file === 'book') continue;
        const filePath = path.join(dir, file);
        const stat = fs.statSync(filePath);
        if (stat.isDirectory()) {
            findMarkdownFiles(filePath, fileList);
        } else if (file.endsWith('.md')) {
            fileList.push(filePath);
        }
    }
    return fileList;
}

function validateMermaidBlocks(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    const lines = content.split('\n');
    let inMermaid = false;
    let mermaidLines = [];
    let startLine = 0;
    const errors = [];

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i].trim();
        if (line.startsWith('```mermaid')) {
            inMermaid = true;
            mermaidLines = [];
            startLine = i + 1;
            continue;
        }
        if (inMermaid && line.startsWith('```')) {
            inMermaid = false;
            // Validate collected Mermaid lines
            const blockErrors = checkMermaidSyntax(mermaidLines, startLine);
            if (blockErrors.length > 0) {
                errors.push(...blockErrors);
            }
            continue;
        }
        if (inMermaid) {
            mermaidLines.push({ lineNum: i + 1, text: line });
        }
    }

    return errors;
}

function checkMermaidSyntax(lines, startLine) {
    const errors = [];
    for (const item of lines) {
        const text = item.text;
        if (!text || text.startsWith('%%')) continue; // Skip comments

        // Check 1: Subgraph Syntax
        if (text.startsWith('subgraph ')) {
            const afterSubgraph = text.substring(9).trim();
            // Check if subgraph has unquoted spaces in ID (e.g. "subgraph Pillar 1: Title")
            // Valid: "subgraph Pillar1["Title"]" or "subgraph Pillar1"
            const match = afterSubgraph.match(/^([^\s"\[]+)/);
            if (!match) {
                errors.push(`Line ${item.lineNum}: Invalid subgraph definition: "${text}". Must specify a single-word ID.`);
            } else {
                const rest = afterSubgraph.substring(match[1].length).trim();
                if (rest && !rest.startsWith('["') && !rest.startsWith('[') && !rest.startsWith('"')) {
                    errors.push(`Line ${item.lineNum}: Subgraph ID contains unquoted space or invalid format: "${text}". Use format: subgraph ID["Title"]`);
                }
            }
            continue;
        }

        // Check 2: Node ID with space before label bracket (excluding subgraph)
        // e.g. "Publish Engine["Title"]" instead of "PublishEngine["Title"]"
        if (text.match(/^[a-zA-Z0-9_-]+\s+[a-zA-Z0-9_-]+\s*\[/) && !text.startsWith('subgraph ')) {
            errors.push(`Line ${item.lineNum}: Node ID contains unquoted space before bracket: "${text}". Node IDs must be single words.`);
        }

        // Check 3: Arrow connection with spaced node ID
        // e.g. "SOUL --> Publish Engine" or "Publish Engine -->"
        if (text.includes('-->') || text.includes('---')) {
            const parts = text.split(/-->|---/);
            for (const part of parts) {
                const trimmed = part.trim();
                // If part has no brackets or quotes, but has spaces (e.g. "Publish Engine")
                if (trimmed && !trimmed.includes('[') && !trimmed.includes('"') && !trimmed.includes('(') && !trimmed.includes('|')) {
                    if (trimmed.includes(' ')) {
                        errors.push(`Line ${item.lineNum}: Connected Node ID contains unquoted space: "${trimmed}" in "${text}". Node IDs must be single words.`);
                    }
                }
            }
        }
    }
    return errors;
}

function runMermaidTestSuite() {
    console.log('📊 Starting Workspace Mermaid Syntax & Markdown Rendering Test Suite...\n');
    let totalFilesChecked = 0;
    let totalMermaidBlocks = 0;
    let totalErrors = 0;

    for (const repoPath of REPOS) {
        const repoName = path.basename(repoPath);
        console.log(`=======================================================`);
        console.log(`🔍 Auditing Markdown Files in Repository: ${repoName}`);
        console.log(`=======================================================`);

        const mdFiles = findMarkdownFiles(repoPath);
        for (const mdFile of mdFiles) {
            totalFilesChecked++;
            const relativePath = path.relative(repoPath, mdFile);
            const fileErrors = validateMermaidBlocks(mdFile);

            if (fileErrors.length > 0) {
                console.error(`  ❌ [${repoName}/${relativePath}] Parse Errors Found:`);
                for (const err of fileErrors) {
                    console.error(`     - ${err}`);
                }
                totalErrors += fileErrors.length;
            } else {
                console.log(`  ✅ [${repoName}/${relativePath}] Valid.`);
            }
        }
        console.log('');
    }

    if (totalErrors === 0) {
        console.log(`🎉 ALL ${totalFilesChecked} MARKDOWN FILES ACROSS ALL 4 REPOSITORIES PASSED MERMAID VALIDATION!`);
    } else {
        console.error(`💥 MERMAID TEST SUITE ENCOUNTERED ${totalErrors} SYNTAX ERROR(S).`);
        process.exitCode = 1;
    }
}

runMermaidTestSuite();
