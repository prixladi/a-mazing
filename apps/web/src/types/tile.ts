export type Position = [number, number];

export type TileKind = 'Empty' | 'Wall' | 'SoftWall' | 'Entrypoint' | 'Checkpoint' | 'Exit';

export type TileHighlight = {
  significancy: 1 | 2 | 3 | 4 | 5 | 6 | 7;
};

export type Tile =
  | { kind: Exclude<TileKind, 'Checkpoint'>; highlighted?: TileHighlight }
  | {
      kind: Extract<TileKind, 'Checkpoint'>;
      level: number;
      highlighted?: TileHighlight;
    };

export type MazeTileBoardBuilder = Tile[][];
export type MazeTileBoard = readonly (readonly Tile[])[];
