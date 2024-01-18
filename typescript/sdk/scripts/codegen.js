const codegen = require('@cosmwasm/ts-codegen').default;
const path = require('path');
const fs = require('fs');
const findWorkspaceRoot = require('find-yarn-workspace-root');

const YARN_WORKSPACE_ROOT = findWorkspaceRoot();
const schemaDir = path.join(YARN_WORKSPACE_ROOT, 'schema');

const outPath = path.join(YARN_WORKSPACE_ROOT, 'typescript', 'sdk', 'src');
fs.rmSync(outPath, { recursive: true, force: true });

codegen({
  contracts: [{ name: 'burndrop', dir: schemaDir }],
  outPath,
  options: {
    bundle: {
      bundleFile: 'index.ts',
      scope: 'contracts',
    },
    client: {
      enabled: true,
    },
    reactQuery: {
      enabled: true,
      version: 'v5',
      mutations: false,
    },
    recoil: {
      enabled: true,
    },
    messageComposer: {
      enabled: true,
    },
  },
}).then(() => {
  console.log('✨ Typescript code is generated successfully!');
});
