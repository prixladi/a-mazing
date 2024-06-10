import React from 'react';
import { Position, Tile, TileBoard, TileKind } from '~/types/maze';
import { MazeTile } from './maze-tile';

type Props = {
  tiles: TileBoard;
  tileOnHover?: Tile;
  onTileClick: (position: Position, kind: TileKind) => any;
};

export const MazeBoard: React.FC<Props> = ({
  tiles,
  tileOnHover,
  onTileClick,
}) => (
  <div className='flex justify-center'>
    <div className='flex gap-[1px] border-slate-50 border-2'>
      {tiles.map((row, x) => (
        <div key={x} className='flex gap-[1px] flex-col'>
          {row.map((tile, y) => (
            <MazeTile
              x={x}
              y={y}
              key={`${x}--${y}`}
              tile={tile}
              tileOnHover={tileOnHover}
              onClick={onTileClick}
            />
          ))}
        </div>
      ))}
    </div>
  </div>
);
