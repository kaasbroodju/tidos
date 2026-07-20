<svelte:options customElement={{ tag: "mobile-menu", shadow: "none" }} />

<script>
    // `shadow: "none"` is important: an IDREF relationship like `aria-controls`
    // does not cross the shadow DOM boundary, so we render into light DOM where
    // the button and the controlled <nav> live in the same tree.
    let { controls = "", label = "Menu" } = $props();

    let open = $state(false);
    let button;
    let target = $state(null);

    // Locate the controlled <nav> and mark it collapsible. Doing this from JS
    // (instead of in the server-rendered HTML) keeps a sane no-JS fallback:
    // without JS the nav is never collapsed, so the links stay reachable.
    $effect(() => {
        target = controls ? document.getElementById(controls) : null;
        target?.setAttribute("data-collapsible", "");
        return () => target?.removeAttribute("data-collapsible");
    });

    // Reflect the open state onto the nav so CSS can show/hide the drawer.
    $effect(() => {
        target?.toggleAttribute("data-open", open);
    });

    function toggle() {
        open = !open;
    }

    function onKeydown(event) {
        if (event.key === "Escape" && open) {
            open = false;
            button?.focus();
        }
    }

    function onPointerdown(event) {
        if (!open) return;
        const path = event.composedPath();
        if (!path.includes(button) && !(target && path.includes(target))) {
            open = false;
        }
    }
</script>

<svelte:window onkeydown={onKeydown} onpointerdown={onPointerdown} />

<button
    bind:this={button}
    class="hamburger"
    class:open
    type="button"
    aria-label={label}
    aria-controls={controls}
    aria-expanded={open}
    onclick={toggle}
>
    <span></span>
    <span></span>
    <span></span>
</button>

<style>
    .hamburger {
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
        gap: 5px;
        /* 44px touch target — WCAG 2.5.5 / 2.5.8 */
        width: 44px;
        height: 44px;
        padding: 0;
        margin: 0;
        background: none;
        border: none;
        cursor: pointer;
        color: inherit;
    }

    .hamburger span {
        display: block;
        width: 26px;
        height: 3px;
        border-radius: 2px;
        background: #fff;
        transition: transform 0.25s ease, opacity 0.2s ease;
    }

    /* animate the three bars into an X when open */
    .hamburger.open span:nth-child(1) {
        transform: translateY(8px) rotate(45deg);
    }

    .hamburger.open span:nth-child(2) {
        opacity: 0;
    }

    .hamburger.open span:nth-child(3) {
        transform: translateY(-8px) rotate(-45deg);
    }

    .hamburger:focus-visible {
        outline: 2px solid #4fd1c5;
        outline-offset: 4px;
        border-radius: 4px;
    }
</style>
