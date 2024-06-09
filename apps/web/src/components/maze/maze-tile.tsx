import React from 'react';

import { Position, Tile, TileType } from '~/types/maze';
import { MazeTileContent } from './maze-tile-content';

type Props = {
  tile: Tile;
  x: number;
  y: number;
  tileOnHover?: Tile;
  onClick: (position: Position, type: TileType) => any;
};

const MazeTileComponent: React.FC<Props> = ({
  tile,
  x,
  y,
  tileOnHover,
  onClick,
}) => {
  return (
    <>
      <div
        onClick={() => onClick([x, y], tile.type)}
        className='w-10 h-10 group'
      >
        {tileOnHover && tile.type == 'Empty' ? (
          <>
            <div className='h-full w-full block group-hover:hidden bg-slate-100'>
              <MazeTileContent tile={tile} />
            </div>
            <div className='opacity-30 h-full w-full hidden group-hover:block bg-slate-100'>
              <MazeTileContent tile={tileOnHover} />
            </div>
          </>
        ) : (
          <div className='h-full w-full bg-slate-100'>
          <MazeTileContent tile={tile} />
          </div>
        )}
      </div>
    </>
  );
};

const propsAreEqual = (p1: Props, p2: Props) => {
  if (Object.keys(p1).length !== Object.keys(p2).length) return false;
  if (JSON.stringify(p1.tile) !== JSON.stringify(p2.tile)) return false;

  const { tile: _, ...compareData } = p1;
  return (
    Object.entries(compareData).filter(([key, value]) => {
      return value !== p2[key as keyof Props];
    }).length === 0
  );
};

export const MazeTile: React.FC<Props> = React.memo(
  MazeTileComponent,
  propsAreEqual
);
