function isAnagram(s: string, t: string): boolean {
    if (s.length !== t.length) return false

    const shash = {}
    const thash = {}

    for (let i = 0; i < s.length; i++) {
        const c = s[i]
        const tc = t[i]

        if (!shash[c]) {
            shash[c] = 1
        } else {
            shash[c] += 1
        }

        if (!thash[tc]) {
            thash[tc] = 1
        } else {
            thash[tc] += 1
        }
    }

    for (const key of Object.keys(shash)) {
        if (shash[key] !== thash[key]) {
            return false
        }
    }

    return true
};
