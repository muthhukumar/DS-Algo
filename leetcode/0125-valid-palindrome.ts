function isPalindrome(s: string): boolean {
  let lo = 0;
  let hi = s.length - 1;

  while (lo < hi) {
    if (!isValidChar(s[lo])) {
      lo++;
      continue;
    }

    if (!isValidChar(s[hi])) {
      hi--;
      continue;
    }

    if (s[lo].toLowerCase() !== s[hi].toLowerCase()) {
      return false;
    }

    lo++;
    hi--;
  }

  return true;
}

function isValidChar(ch: string) {
  if (ch === " ") return false;

  if (Number(ch) >= 0 && Number(ch) <= 9) return true;

  const code = ch.charCodeAt(0);

  return (code >= 97 && code <= 122) || (code >= 65 && code <= 90);
}
