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
  <div className='w-full flex h-full justify-center items-center bg-emerald-50 shadow-md'>
    <LogIn className='text-emerald-500' />
  </div>
);

const ExitTile: React.FC = () => (
  <div className='w-full flex h-full justify-center items-center bg-red-100 shadow-md'>
    <LogOut className='text-red-600' />
  </div>
);

const WallTile: React.FC = () => (
  <div className='w-full h-full bg-amber-900 rounded-md' />
);

const SoftWallTile: React.FC = () => (
  <div className='w-full h-full bg-yellow-600 rounded-md' />
);

const EmptyTile: React.FC = () => (
  <div className='w-full h-full bg-slate-100' />
);

const CheckpointTile: React.FC<{ level: number }> = ({ level }) => (
  <div className='w-full flex h-full bg-slate-100'>
    <span
      className={clsx(
        'w-full flex h-full justify-center items-center rounded-full font-bold text-lg',
        getCheckpointColors(level)
      )}
    >
      {level}
    </span>
  </div>
);
