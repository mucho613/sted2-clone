/**
 * Note number を "[KeyName] [Octave]" 形式の文字列に変換する
 * TODO: 調号による表記の変更対応
 *
 * @param noteNumber MIDI Note Number (0-127)
 */
export function formatNoteNumber(noteNumber: number): string {
  const keyNames = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

  const keyName = keyNames[noteNumber % 12];
  const octave = Math.floor(noteNumber / 12) - 1;

  return `${keyName} ${octave}`;
}
