const codegen = require('@cosmwasm/ts-codegen').default;
const path = require('path');
const fs = require('fs');
const findWorkspaceRoot = require('find-yarn-workspace-root');
const prettier = require('prettier');

const YARN_WORKSPACE_ROOT = findWorkspaceRoot() || '';
const schemaDir = path.join(YARN_WORKSPACE_ROOT, 'schema');
const outPath = path.join(
  YARN_WORKSPACE_ROOT,
  'typescript',
  'sdk',
  'src',
  'contracts',
);

const PRETTIER_CONFIG_PATH = path.join(YARN_WORKSPACE_ROOT, '.prettierrc.js');

const main = async () => {
  // Codegen
  await codegen({
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
      messageComposer: {
        enabled: true,
      },
    },
  });
  console.log('✨ Typescript code is generated successfully!');

  // Prettier
  const config = await prettier.resolveConfig(PRETTIER_CONFIG_PATH);
  const files = fs.readdirSync(outPath);
  const promises = await Promise.all(
    files.map(async (file) => {
      const filePath = path.join(outPath, file);
      if (fs.lstatSync(filePath).isDirectory()) {
        return null;
      }
      const formatted = await prettier.format(
        fs.readFileSync(filePath, 'utf8'),
        { parser: 'typescript', ...config },
      );
      return [filePath, formatted];
    }),
  );
  promises.forEach((out) => {
    if (!out) {
      return;
    }
    const [filePath, formatted] = out;
    fs.writeFileSync(filePath, formatted);
  });
  console.log('💅 Typescript code is formatted successfully!');
};

main();
