import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import { defineConfig } from 'rollup';

const baseConfig = {
  input: 'src/index.ts',
  plugins: [
    resolve({
      browser: true,
      preferBuiltins: false,
    }),
    commonjs(),
    typescript({
      tsconfig: './tsconfig.json',
    }),
  ],
  external: ['react', 'vue', 'ws'],
};

export default defineConfig([
  // Node.js build
  {
    ...baseConfig,
    output: {
      file: 'dist/node/index.js',
      format: 'cjs',
      sourcemap: true,
    },
    external: [...baseConfig.external, 'ws'],
  },
  // Browser ESM build
  {
    ...baseConfig,
    output: {
      file: 'dist/browser/index.esm.js',
      format: 'esm',
      sourcemap: true,
    },
    external: [...baseConfig.external, 'ws'],
  },
  // Browser UMD build
  {
    ...baseConfig,
    output: {
      file: 'dist/browser/index.umd.js',
      format: 'umd',
      name: 'TaskQueueSDK',
      sourcemap: true,
    },
    external: [...baseConfig.external, 'ws'],
  },
]);
