import React from 'react';

import { Position, Tile, MazeTileBoard, TileKind } from '~/types/tile';

import { MazeTile } from './maze-tile';

type Props = {
  tiles: MazeTileBoard;
  tileOnHover?: (tileKind: TileKind) => Tile | null;
  onTileClick: (position: Position, kind: TileKind) => any;
};

export const MazeBoard: React.FC<Props> = ({ tiles, tileOnHover, onTileClick }) => (
  <div className="flex justify-center">
    <div className="flex gap-[1px] border-2 border-slate-50">
      {tiles.map((row, x) => (
        <div key={x} className="flex flex-col-reverse gap-[1px]">
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
