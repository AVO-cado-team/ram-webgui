

// highlightError(editor: monaco.editor.IStandaloneCodeEditor, error: any, line: number, column: number): void;
export function highlightError(editor, error, line, column) {
    editor.deltaDecorations(editor.getModel().getAllDecorations().filter(d => d.options.className === 'error-line-highlight').map(d => d.id), []);
    // Опции декораций для подсветки строки
    const decoration = {
        range: new monaco.Range(line, 1, line, 1),
        options: {
            isWholeLine: true,
            className: 'error-line-highlight', // CSS класс для стилизации строки с ошибкой
            glyphMarginClassName: 'error-glyph' // Опционально: CSS класс для иконки в margin
        }
    };

    // Добавляем декорацию в редактор
    editor.deltaDecorations([], [decoration]);
}