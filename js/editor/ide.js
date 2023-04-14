function createKeyBindings() {
  editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KEY_S, function() {

    let content = editor.getValue();
    let encodedContent = btoa(content);

    if (prevFileEncodedContent == null || prevFileEncodedContent != encodedContent) {
      downloadCode(content);
      prevFileEncodedContent = encodedContent;
    }
  });

  editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.US_SLASH, function() {

    let selection = editor.getSelection();

    console.log(selection);

    let range = new monaco.Range(selection.startLineNumber, selection.startColumn, selection.endLineNumber, selection.endColumn);
    let text = editor.getModel().getValueInRange(range);

    let comment = '';
    let lines = text.split('\n');

    for (let i = 0; i < lines.length - 1; i++)
      comment += '#' + lines[i] + '\n';

    comment += '#' + lines[lines.length - 1];

    editor.executeEdits('comment', [{
      range: range,
      text: comment
    }]);
  });
}

function downloadCode(content) {
  var element = document.createElement('a');
  element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(content));
  element.setAttribute('download', "project.ram");

  element.style.display = 'none';
  document.body.appendChild(element);

  element.click();

  document.body.removeChild(element);
}

function configureRAMLanguage() {
  monaco.languages.register({
    id: 'ram'
  });

  monaco.languages.setMonarchTokensProvider('ram', makeTokensProvider());
}

