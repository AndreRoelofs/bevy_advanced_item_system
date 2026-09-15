// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

(() => {
    const darkThemes = ['frappe', 'macchiato', 'mocha'];
    const isDarkTheme = () => darkThemes.some(theme =>
        document.documentElement.classList.contains(theme)
    );
    const initiallyDark = isDarkTheme();

    mermaid.initialize({
        startOnLoad: true,
        theme: initiallyDark ? 'dark' : 'default',
        fontFamily: '"Geist Mono", ui-monospace, SFMono-Regular, Consolas, "Ubuntu Mono", Menlo, monospace',
    });

    // Reload after mdBook applies a light/dark change so Mermaid redraws its diagrams.
    // Observing the applied theme also handles Auto and OS color-scheme changes.
    new MutationObserver(() => {
        if (isDarkTheme() !== initiallyDark) {
            window.location.reload();
        }
    }).observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
})();
