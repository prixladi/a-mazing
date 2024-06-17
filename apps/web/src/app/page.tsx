'use client';

import { useCallback, useState } from 'react';

import { MazeBoard } from '~/components/maze';
import { useMaze, useMazer } from '~/hooks/maze';
import { Position, TileKind } from '~/types/tile';

import { MazeConfig } from '~/types/maze';
import { useMazerGenerator } from '~/hooks/maze/use-mazer-generator';
import { MazerGeneratorType } from 'mazer';

const tileOnHover = (kind: TileKind) =>
  kind === 'Empty' ? { kind: 'SoftWall' } : null;

export default function Home() {
  const [mazeConfig, setMazeConfig] = useState<MazeConfig>({
    colCount: 15,
    rowCount: 10,
    walls: [
      [5, 5],
      [7, 7],
    ],
    maxSoftWallCount: 8,
    entrypoints: [[0, 0]],
    checkpoints: [
      { position: [8, 8], level: 1 },
      { position: [8, 4], level: 2 },
      { position: [9, 9], level: 3 },
      { position: [13, 9], level: 4 },
      { position: [0, 9], level: 5 },
      { position: [3, 9], level: 6 },
      { position: [0, 6], level: 7 },
      { position: [14, 6], level: 8 },
      { position: [2, 2], level: 9 },
      { position: [14, 9], level: 999 },
      { position: [14, 2], level: 999 },
    ],
  });

  const { generateConfig } = useMazerGenerator();
  const {
    mazeMutations,
    mazeBoard,
    mazeLimits,
    mutateMazePosition,
    clearMutations,
    animatePath,
  } = useMaze(mazeConfig);

  const { score, path } = useMazer(mazeConfig, mazeMutations);

  const onTileClick = useCallback(
    (position: Position, kind: TileKind) => {
      if (kind === 'Empty') return mutateMazePosition(position, 'SoftWall');
      if (kind === 'SoftWall') return mutateMazePosition(position, 'Empty');
    },
    [mutateMazePosition]
  );

  return (
    <main className='max-w-screen-lg mx-auto'>
      <MazeBoard
        tiles={mazeBoard}
        onTileClick={onTileClick}
        tileOnHover={tileOnHover as any}
      />
      <div className='flex justify-between gap-10'>
        <pre>{score}</pre>
        <pre>{mazeLimits.softWallsRemaining}</pre>
        <button
          onClick={() => {
            const config = generateConfig(MazerGeneratorType.Waterfall);
            setMazeConfig(config);
          }}
        >
          Waterfall
        </button>
        <button
          onClick={() => {
            const config = generateConfig(MazerGeneratorType.Vanilla);
            setMazeConfig(config);
          }}
        >
          Vanilla
        </button>
        <button
          onClick={() => {
            if (path) animatePath(path);
          }}
        >
          RUN
        </button>
      </div>
    </main>
  );
}
