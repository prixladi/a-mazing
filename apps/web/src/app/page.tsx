'use client';

import { useCallback} from 'react';

import { MazeBoard } from '~/components/maze';
import { useMaze } from '~/hooks/use-maze';
import { Position, Tile, TileKind } from '~/types/maze';

import { useMazer } from '~/hooks/use-mazer';

const tileOnHover: Tile = { kind: 'SoftWall' };

export default function Home() {
  const { mazeState, mazeTiles, setTileOnPosition } = useMaze({
    colCount: 15,
    rowCount: 10,
    walls: [
      [5, 5],
      [7, 7],
    ],
    softWalls: [],
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

  const { score } = useMazer(mazeState);

  const onTileClick = useCallback(
    (position: Position, kind: TileKind) => {
      if (kind === 'Empty')
        return setTileOnPosition(position, { kind: 'SoftWall' });
      if (kind === 'SoftWall')
        return setTileOnPosition(position, { kind: 'Empty' });
    },
    [setTileOnPosition]
  );

  return (
    <main className='max-w-screen-lg mx-auto'>
      <MazeBoard
        tiles={mazeTiles}
        onTileClick={onTileClick}
        tileOnHover={tileOnHover}
      />
      {score}
    </main>
  );
}
