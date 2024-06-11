export type Position = [number, number];

export type TileKind =
  | 'Empty'
  | 'Wall'
  | 'SoftWall'
  | 'Entrypoint'
  | 'Checkpoint'
  | 'Exit';

export type Tile =
  | { kind: Exclude<TileKind, 'Checkpoint'> }
  | { kind: Extract<TileKind, 'Checkpoint'>; level: number };

export type MazeTileBoardBuilder = Tile[][];
export type MazeTileBoard = readonly (readonly Tile[])[];
