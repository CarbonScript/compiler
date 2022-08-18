
const stringData = `

This is the test statement for counting words.

`

const countWord = (str:string) => {
    let count = 0;
    let stri = stringData.trimStart().trimEnd();
    for(let i = 0;(stri[i]!==undefined && stri[i] === ' ');i++){
        count++;
    }
    return count + 1;
}

console.log(countWord(stringData));
