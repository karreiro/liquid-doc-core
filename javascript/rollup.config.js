import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import terser from '@rollup/plugin-terser';
import copy from 'rollup-plugin-copy';

export default {
  input: 'src/index.js',
  output: [
    {
      file: 'dist/index.js',
      format: 'cjs',
      exports: 'named'
    },
    {
      file: 'dist/index.mjs',
      format: 'es'
    }
  ],
  plugins: [
    resolve({
      preferBuiltins: false,
      browser: true
    }),
    commonjs(),
    copy({
      targets: [
        {
          src: '../web/pkg/liquid_doc_wasm_bg.wasm',
          dest: 'wasm/',
          rename: 'liquiddoc_parser.wasm'
        },
        {
          src: '../web/pkg/liquid_doc_wasm.js',
          dest: 'wasm/',
          rename: 'liquiddoc_parser.js'
        }
      ]
    }),
    terser()
  ],
  external: []
};
