/**
 * Determines the navigation direction based on the NavigationActivation.
 * Returns 'back', 'skip', or 'forward'.
 *
 * @param {NavigationActivation | null} activation
 */
function getNavDirection(activation) {
    if (!activation?.from?.url || !activation?.entry?.url) return 'forward';

    const fromPath = new URL(activation.from.url).pathname;
    const toPath   = new URL(activation.entry.url).pathname;

    // No transition when navigating between docs pages
    if (fromPath.startsWith('/docs') && toPath.startsWith('/docs')) {
        return 'skip';
    }

    // Browser back/forward button
    if (activation.navigationType === 'traverse' &&
        activation.from.index > activation.entry.index) {
        return 'back';
    }

    // Logo click: /docs/* → / feels semantically like going back
    if (activation.navigationType === 'push' &&
        fromPath.startsWith('/docs') &&
        toPath === '/') {
        return 'back';
    }

    return 'forward';
}

// pageswap fires on the OLD page right before unload.
window.addEventListener('pageswap', (e) => {
    if (!e.viewTransition) return;

    const direction = getNavDirection(e.activation);

    if (direction === 'skip') {
        e.viewTransition.skipTransition();
    } else if (direction === 'back') {
        e.viewTransition.types.add('back');
    }
});

// pagereveal fires on the NEW page before the transition plays.
// navigation.activation mirrors pageswap's e.activation so the same
// helper works — 'entry' is the current (new) page, 'from' is where we came from.
window.addEventListener('pagereveal', (e) => {
    if (!e.viewTransition) return;

    const direction = getNavDirection(navigation.activation);

    if (direction === 'skip') {
        e.viewTransition.skipTransition();
    } else if (direction === 'back') {
        e.viewTransition.types.add('back');
    }
});
