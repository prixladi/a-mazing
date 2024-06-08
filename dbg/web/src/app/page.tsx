'use client';

import Image from 'next/image';
import init, { Mazer, Position } from 'engine';
import { useEffect, useState } from 'react';

export default function Home() {
  useEffect(() => {
    init();
  }, []);

  return (
    <main>
      <button
        onClick={() => {
          const mazer = Mazer.new();
          const result = mazer?.run([Position.new(3, 0)]);
          console.log(result?.distance);
          console.log(result?.path);
        }}
      >
        AAAAAAAAAAAAAAA
      </button>
    </main>
  );
}
