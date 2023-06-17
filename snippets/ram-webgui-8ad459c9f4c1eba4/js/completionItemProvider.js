const KEYWORDS = [
  "ADD", "SUB", "MUL", "DIV", "JUMP", "JMP", "JZ", "JZERO", "JGZ",
  "JGTZ", "LOAD", "STORE", "INPUT", "READ", "WRITE", "OUTPUT", "HALT"
];

const CompletionItemKind = {
  Method: 0,
  Function: 1,
  Constructor: 2,
  Field: 3,
  Variable: 4,
  Class: 5,
  Struct: 6,
  Interface: 7,
  Module: 8,
  Property: 9,
  Event: 10,
  Operator: 11,
  Unit: 12,
  Value: 13,
  Constant: 14,
  Enum: 15,
  Enummember: 16,
  Keyword: 17,
  Text: 18,
  Color: 19,
  File: 20,
  Reference: 21,
  Customcolor: 22,
  Folder: 23,
  Typeparameter: 24,
  User: 25,
  Issue: 26,
  Snippet: 27,
}

export function completionItemsProvider() {
  return {
    provideCompletionItems: (model, position) => {
      let c = model.getValue().split('\n')[position.lineNumber - 1][position.column - 2];
      let keywords = KEYWORDS.slice(0);

      if (c >= 'a' && c <= 'z')
        keywords = keywords.map(s => s.toLowerCase());

      return {
        suggestions: keywords.map(keyword => ({
          label: keyword,
          kind: CompletionItemKind.Keyword,
          insertText: keyword,
        }))
      }
    }
  }
}
