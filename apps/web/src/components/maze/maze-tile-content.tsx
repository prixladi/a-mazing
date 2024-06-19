import clsx from 'clsx';
import { LogIn, LogOut } from 'lucide-react';

import { Tile } from '~/types/tile';
import { getCheckpointColors } from '~/utils/checkpoint';

type Props = { tile: Tile };

export const MazeTileContent: React.FC<Props> = ({ tile }) => {
  if (tile.kind === 'Entrypoint') return <EntrypointTile />;
  if (tile.kind === 'Exit') return <ExitTile />;
  if (tile.kind === 'Wall') return <WallTile />;
  if (tile.kind === 'SoftWall') return <SoftWallTile />;
  if (tile.kind === 'Checkpoint') return <CheckpointTile level={tile.level} />;
  if (tile.kind === 'Empty') return <EmptyTile />;
  throw new Error(`Unable to render content for '${tile.kind}' tile.`);
};

const EntrypointTile: React.FC = () => (
  <div className="flex h-full w-full items-center justify-center bg-emerald-50 shadow-md">
    <LogIn className="text-emerald-500" />
  </div>
);

const ExitTile: React.FC = () => (
  <div className="flex h-full w-full items-center justify-center bg-red-100 shadow-md">
    <LogOut className="text-red-600" />
  </div>
);

const WallTile: React.FC = () => <div className="h-full w-full rounded-md bg-amber-900" />;

const SoftWallTile: React.FC = () => <div className="h-full w-full rounded-md bg-yellow-600" />;

const EmptyTile: React.FC = () => <div className="h-full w-full bg-slate-100" />;

const CheckpointTile: React.FC<{ level: number }> = ({ level }) => (
  <div className="flex h-full w-full bg-slate-100">
    <span
      className={clsx(
        'flex h-full w-full items-center justify-center rounded-full text-lg font-bold',
        getCheckpointColors(level),
      )}
    >
      {level}
    </span>
  </div>
);
