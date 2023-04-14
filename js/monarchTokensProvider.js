let keywords = ["ADD", "SUB", "MUL", "DIV", "JUMP", "JMP", "JZ", "JZERO", "JGZ", "JGTZ", "LOAD", "STORE", "INPUT", "READ", "WRITE", "OUTPUT", "HALT"]

function makeTokensProvider() {
  return {
    keywords: keywords,
    tokenizer: {
      root: [
        [/@?[a-zA-Z][\w$]*/, {
          cases: {
            '@keywords': 'keyword',
            '@default': 'varialbe'
          }
        }],
        // [/\d+/, 'number'],
        [/=[\d*]/, 'delimiter'],
        [/[*]/, 'pointer'],
        [/#.*/, 'comment'],
        [/:/, 'pointer']
      ]
    }
  }
}
