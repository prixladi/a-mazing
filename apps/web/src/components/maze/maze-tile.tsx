import React, { useCallback, useMemo } from 'react';

import clsx from 'clsx';

import { Position, Tile, TileKind } from '~/types/tile';

import { MazeTileContent } from './maze-tile-content';

type Props = {
  tile: Tile;
  x: number;
  y: number;
  tileOnHover?: (tileKind: TileKind) => Tile | null;
  onClick: (position: Position, tileKind: TileKind) => any;
};

const MazeTileComponent: React.FC<Props> = ({
  tile,
  x,
  y,
  onClick: inputOnClick,
  tileOnHover: inputTileOnHover,
}) => {
  const onClick = useCallback(
    () => inputOnClick([x, y], tile.kind),
    [x, y, tile.kind, inputOnClick],
  );

  const tileOnHover = useMemo(
    () => (inputTileOnHover ? inputTileOnHover(tile.kind) : undefined),
    [inputTileOnHover, tile.kind],
  );

  return (
    <div onClick={onClick} className="group relative h-10 w-10">
      <div
        className={clsx(
          'absolute z-50 h-full w-full rounded-full bg-amber-800',
          tile.highlighted ? `opacity-${tile.highlighted?.significancy * 10}` : 'opacity-0',
        )}
      />
      {tileOnHover ? (
        <>
          <div className="block h-full w-full bg-slate-100 group-hover:hidden">
            <MazeTileContent tile={tile} />
          </div>
          <div className="hidden h-full w-full bg-slate-100 opacity-30 group-hover:block">
            <MazeTileContent tile={tileOnHover} />
          </div>
        </>
      ) : (
        <div className="h-full w-full bg-slate-100">
          <MazeTileContent tile={tile} />
        </div>
      )}
    </div>
  );
};

const propsAreEqual = (p1: Props, p2: Props) => {
  if (Object.keys(p1).length !== Object.keys(p2).length) return false;
  if (JSON.stringify(p1.tile) !== JSON.stringify(p2.tile)) return false;

  const { tile: _, ...compareData } = p1;
  return (
    Object.entries(compareData).filter(([key, value]) => value !== p2[key as keyof Props])
      .length === 0
  );
};

export const MazeTile: React.FC<Props> = React.memo(MazeTileComponent, propsAreEqual);
