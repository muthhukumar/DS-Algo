function containsDuplicate(nums: number[]): boolean {
    const hash = {}

    for (const n of nums) {
        if (!hash[n]) {
            hash[n] = true
        } else {
            return true
        }
    }

    return false
};

function containsDuplicateMap(nums: number[]): boolean {
   const hash = new Map()

   for(const n of nums){
        if(!hash.has(n)) {
            hash.set(n, true)
        }else {
            return true
        }
   }

   return false
};
