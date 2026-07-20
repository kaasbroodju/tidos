<svelte:options customElement="framework-tabs"></svelte:options>

<script>
    let { frameworks = '', active = '' } = $props();

    let tabs = $derived(
        frameworks
            .split(',')
            .map((f) => f.trim())
            .filter(Boolean)
    );

    function navigate(tab) {
        document.querySelectorAll('[data-framework]').forEach(el => {
            el.style.display = 'none';
        });
        document.querySelectorAll(`[data-framework="${tab}"]`).forEach(el => {
            el.style.display = 'block';
        });
        active = tab;
        const url = new URL(window.location.href);
        url.searchParams.set('framework', tab);
        history.replaceState(null, '', url.toString());
    }
</script>

<div class="framework-container">
    <div class="tabs">
        {#each tabs as tab}
            <button class="tab" class:active={tab === active} onclick={() => navigate(tab)}>
                {tab}
            </button>
        {/each}
    </div>
</div>


<style>
    :host {
        display: block;
    }

    .framework-container {
        overflow-x: scroll;
        margin-bottom: 1.75rem;
    }

    .tabs {
        display: flex;
        gap: 0;

        border-bottom: 2px solid #30363d;
    }

    .tab {
        background: none;
        border: none;
        border-bottom: 2px solid transparent;
        margin-bottom: -2px;
        padding: 0.55rem 1.2rem;
        cursor: pointer;
        font-size: 0.9rem;
        color: #8b949e;
        transition: color 0.15s, border-color 0.15s;
        font-family: inherit;
        letter-spacing: 0.01em;
    }

    .tab:hover {
        color: #c9d1d9;
    }

    .tab.active {
        color: #4fd1c5;
        border-bottom-color: #4fd1c5;
        font-weight: 600;
    }
</style>
