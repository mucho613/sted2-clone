import { test, expect } from "vitest";

import { formatNoteNumber } from "./noteNumber";

test("formatNoteNumber", () => {
  expect(formatNoteNumber(60)).toBe("C 4");
  expect(formatNoteNumber(61)).toBe("C# 4");
  expect(formatNoteNumber(62)).toBe("D 4");
  expect(formatNoteNumber(63)).toBe("D# 4");
  expect(formatNoteNumber(64)).toBe("E 4");
  expect(formatNoteNumber(65)).toBe("F 4");
  expect(formatNoteNumber(66)).toBe("F# 4");
  expect(formatNoteNumber(67)).toBe("G 4");
  expect(formatNoteNumber(68)).toBe("G# 4");
  expect(formatNoteNumber(69)).toBe("A 4");
  expect(formatNoteNumber(70)).toBe("A# 4");
  expect(formatNoteNumber(71)).toBe("B 4");
  expect(formatNoteNumber(72)).toBe("C 5");
});
