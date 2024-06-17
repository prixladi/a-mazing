import type { Config } from 'tailwindcss';

let usedOpacities = [
  'opacity-10',
  'opacity-20',
  'opacity-30',
  'opacity-40',
  'opacity-50',
  'opacity-60',
  'opacity-70',
  'opacity-80',
  'opacity-90',
];

const config = {
  darkMode: ['class'],
  content: [
    './pages/**/*.{ts,tsx}',
    './components/**/*.{ts,tsx}',
    './app/**/*.{ts,tsx}',
    './src/**/*.{ts,tsx}',
  ],
  prefix: '',
  safelist: usedOpacities,
} satisfies Config;

export default config;
