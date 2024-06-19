/** @type {import("prettier").Config} */
export default {
    printWidth: 100,
    parser: 'typescript',
    singleQuote: true,
    arrowParens: 'always',
    tabWidth: 2,
    semi: true,
    trailingComma: 'all',
    plugins: ['prettier-plugin-tailwindcss'],
    overrides: [
      {
        files: ['**/*.json'],
        options: {
          singleQuote: false,
        },
      },
      {
        files: '**/*.json',
        options: {
          parser: 'json',
        },
      },
    ],
  };