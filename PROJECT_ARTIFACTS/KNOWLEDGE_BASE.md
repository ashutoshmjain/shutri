# Knowledge Base

This document serves as the repository for established patterns, technical decisions, and reusable solutions identified during development.

---

## 1. Architectural Decisions

### 1.1. Code-Node Bypass for Restricted Environments
On strict Debian/WSL environments where the `Execute Command` node is hidden or restricted, the #SMS pipeline uses the `Code` node (Javascript) to orchestrate local binaries. This bypass requires explicit authorization of the `child_process` module.

---

## 2. Technical Patterns

### 2.1. n8n Startup & Privileges (Debian/WSL)
To enable local file access and binary execution, n8n must be launched with specific environment variables. Use the following startup script (`start_n8n.sh`):

```bash
#!/bin/bash
# 1. Enable local file system write access
export N8N_BLOCK_FS_WRITE_ACCESS=false

# 2. Allow the 'child_process' module in Javascript nodes
export N8N_NODES_JS_ALLOW_BUILTIN=child_process,fs,path,os
export NODE_FUNCTION_ALLOW_BUILTIN=child_process,fs,path,os

# 3. Explicitly include required core nodes
export N8N_NODES_INCLUDE="n8n-nodes-base.webhook,n8n-nodes-base.code,n8n-nodes-base.set,n8n-nodes-base.if"

# 4. Start n8n
n8n start
```

### 2.2. Webhook Trigger vs. File Watcher
Due to WSL file system event latency, the `Webhook Trigger` is the preferred method for production-level reliability over the `Local File Trigger`.
