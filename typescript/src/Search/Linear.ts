export default function search<T>(list: Array<T>, needle: T) {
  for (let i = 0; i < list.length; ++i) {
    if (list[i] === needle) {
      return true;
    }
  }

  return false;
}
