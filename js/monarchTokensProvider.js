const keywords = ["ADD", "SUB", "MUL", "DIV", "JUMP", "JMP", "JZ", "JZERO", "JGZ", "JGTZ", "LOAD", "STORE", "INPUT", "READ", "WRITE", "OUTPUT", "HALT"]

export function makeTokensProvider() {
  return {
    keywords: keywords,
    ignoreCase: true,
    tokenizer: {
      root: [
        [/@?[a-zA-Z][\w$]*/, {
          cases: {
            '@keywords': 'keyword',
            '@default': 'varialbe'
          }
        }],
        [/=\d+/, 'number'],
        [/\d+/, 'number'],
        [/[*]/, 'pointer'],
        [/#.*/, 'comment'],
        [/:/, 'pointer']
      ]
    }
  }
}
