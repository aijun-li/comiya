import pluginJs from '@eslint/js';
import pluginPrettierRecommended from 'eslint-plugin-prettier/recommended';
import pluginVue from 'eslint-plugin-vue';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default [
  { files: ['**/*.{js,mjs,cjs,ts,vue}'] },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/essential'],
  pluginPrettierRecommended,
  {
    files: ['*.vue', '**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: '@typescript-eslint/parser',
        sourceType: 'module',
      },
    },
  },
  {
    rules: {
      'prettier/prettier': 'error',
      'vue/multi-word-component-names': 'off',
    },
  },
  {
    ignores: ['src/*.d.ts', 'src-tauri/**/*'],
  },
];
