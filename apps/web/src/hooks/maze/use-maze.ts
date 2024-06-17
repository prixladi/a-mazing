import { useCallback, useEffect, useRef, useState } from 'react';

import { MazeConfig, MazeMutations } from '~/types/maze';
import { Position, TileHighlight, TileKind } from '~/types/tile';
import { delay } from '~/utils/delay';

import {
  useConfiguredMazeBoard,
  useMutatedMazeBoard,
  useMazeLimits,
} from './base';

const defaultMazeMutations: MazeMutations = {
  softWalls: [],
  highlighted: [],
};

export const useMaze = (inputConfig: MazeConfig) => {
  const [config, setConfig] = useState(inputConfig);

  const [mazeMutations, setMazeMutations] =
    useState<MazeMutations>(defaultMazeMutations);

  const pathAnimationId = useRef<number | null>(null);

  const mazeLimits = useMazeLimits({ config: config, mazeMutations });
  const configuredBoard = useConfiguredMazeBoard(config);
  const mutatedMazeBoard = useMutatedMazeBoard({
    configuredBoard,
    mazeMutations,
  });

  // Need to do it like this because when configuration changes
  // we need to clear all mutations and only after that can use that new config
  // without it some mutation can be out of bounds of the new board
  // we also need to stop any running animations
  useEffect(() => {
    clearMutations();
    pathAnimationId.current = null;
    setConfig(inputConfig);
  }, [inputConfig]);

  const mutateMazePosition = useCallback(
    ([x, y]: Position, type: Extract<TileKind, 'SoftWall' | 'Empty'>) => {
      setMazeMutations((oldMutations) => {
        const softWalls = oldMutations.softWalls.filter(
          (position) => position[0] !== x || position[1] !== y
        );

        if (type === 'SoftWall' && softWalls.length < config.maxSoftWallCount) {
          softWalls.push([x, y]);
        }

        return { ...oldMutations, softWalls };
      });
    },
    [config.maxSoftWallCount]
  );

  const clearMutations = useCallback(() => {
    setMazeMutations(defaultMazeMutations);
  }, []);

  const animatePath = useCallback(
    async (pathInput: Position[], animationId: number) => {
      const path = [...pathInput];

      let end = false;
      while (!end && pathAnimationId.current === animationId) {
        const position = path.shift();
        setMazeMutations((oldMutations) => {
          const highlighted = [...oldMutations.highlighted]
            .filter(({ significancy }) => significancy > 0)
            .map((old) => {
              const significancy = old.significancy - 1;
              return {
                ...old,
                significancy: significancy as TileHighlight['significancy'],
              };
            });

          if (position) {
            highlighted.push({
              position,
              significancy: 7,
            });
          }

          if (highlighted.length === 0) end = true;

          return {
            ...oldMutations,
            highlighted: highlighted,
          };
        });

        await delay(75);
      }
    },
    []
  );

  const tryAnimatePath = useCallback(
    async (path: Position[]) => {
      const animationId = Math.random();
      pathAnimationId.current = animationId;
      setMazeMutations((old) => ({ ...old, highlighted: [] }));
      await animatePath(path, animationId);
    },
    [animatePath]
  );

  return {
    mutateMazePosition,
    clearMutations,
    mazeBoard: mutatedMazeBoard,
    mazeMutations,
    mazeLimits,
    animatePath: tryAnimatePath,
  };
};
