#!/bin/bash
export N8N_BLOCK_FS_WRITE_ACCESS=false
export N8N_NODES_JS_ALLOW_BUILTIN=child_process,fs,path,os
export NODE_FUNCTION_ALLOW_BUILTIN=child_process,fs,path,os
export N8N_NODES_INCLUDE="n8n-nodes-base.webhook,n8n-nodes-base.code,n8n-nodes-base.set,n8n-nodes-base.if"
n8n start
