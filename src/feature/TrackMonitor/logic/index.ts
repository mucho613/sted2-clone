import { BLACK_KEY_HEIGHT, BLACK_KEY_WIDTH, WHITE_KEY_HEIGHT, WHITE_KEY_WIDTH } from "../constant";

export const calculateKeyPositionOnlyWhiteKeys = (keyNumber: number) => {
  const octave = Math.floor(keyNumber / 12);
  const keyNumberInOctave = keyNumber % 12;

  const octaveOffset = octave * WHITE_KEY_WIDTH * 7;

  switch (keyNumberInOctave) {
    case 0:
      return {
        x: octaveOffset + 0,
        y: 0,
        width: WHITE_KEY_WIDTH,
        height: WHITE_KEY_HEIGHT,
      };
    case 2:
      return {
        x: octaveOffset + WHITE_KEY_WIDTH,
        y: 0,
        width: WHITE_KEY_WIDTH,
        height: WHITE_KEY_HEIGHT,
      };
    case 4:
      return {
        x: octaveOffset + WHITE_KEY_WIDTH * 2,
        y: 0,
        width: WHITE_KEY_WIDTH,
        height: WHITE_KEY_HEIGHT,
      };
    case 5:
      return {
        x: octaveOffset + WHITE_KEY_WIDTH * 3,
        y: 0,
        width: WHITE_KEY_WIDTH,
        height: WHITE_KEY_HEIGHT,
      };
    case 7:
      return {
        x: octaveOffset + WHITE_KEY_WIDTH * 4,
        y: 0,
        width: WHITE_KEY_WIDTH,
        height: WHITE_KEY_HEIGHT,
      };
    case 9:
      return {
        x: octaveOffset + WHITE_KEY_WIDTH * 5,
        y: 0,
        width: WHITE_KEY_WIDTH,
        height: WHITE_KEY_HEIGHT,
      };
    case 11:
      return {
        x: octaveOffset + WHITE_KEY_WIDTH * 6,
        y: 0,
        width: WHITE_KEY_WIDTH,
        height: WHITE_KEY_HEIGHT,
      };
    default:
      return {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
      };
  }
};

export const calculateKeyPositionOnlyBlackKeys = (keyNumber: number) => {
  const octave = Math.floor(keyNumber / 12);
  const keyNumberInOctave = keyNumber % 12;

  const octaveOffset = octave * WHITE_KEY_WIDTH * 7;

  switch (keyNumberInOctave) {
    case 1:
      return {
        x: octaveOffset + (WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2),
        y: 0,
        width: BLACK_KEY_WIDTH,
        height: BLACK_KEY_HEIGHT,
      };
    case 3:
      return {
        x: octaveOffset + (WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2) + WHITE_KEY_WIDTH,
        y: 0,
        width: BLACK_KEY_WIDTH,
        height: BLACK_KEY_HEIGHT,
      };
    case 6:
      return {
        x: octaveOffset + (WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2) + WHITE_KEY_WIDTH * 3,
        y: 0,
        width: BLACK_KEY_WIDTH,
        height: BLACK_KEY_HEIGHT,
      };
    case 8:
      return {
        x: octaveOffset + (WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2) + WHITE_KEY_WIDTH * 4,
        y: 0,
        width: BLACK_KEY_WIDTH,
        height: BLACK_KEY_HEIGHT,
      };
    case 10:
      return {
        x: octaveOffset + (WHITE_KEY_WIDTH - BLACK_KEY_WIDTH / 2) + WHITE_KEY_WIDTH * 5,
        y: 0,
        width: BLACK_KEY_WIDTH,
        height: BLACK_KEY_HEIGHT,
      };
    default:
      return {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
      };
  }
};
