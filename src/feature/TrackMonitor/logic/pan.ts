export const panText = (pan: number) => {
  if (pan === 64) {
    return "C";
  } if (pan === 0) {
    return "L";
  } if (pan === 127) {
    return "R";
  } if (pan < 64) {
    return `L${(pan - 64) * -1}`;
  } if (pan > 64) {
    return `R${pan - 64}`;
  }
}
