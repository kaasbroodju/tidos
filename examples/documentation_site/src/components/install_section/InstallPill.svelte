<svelte:options customElement="install-pill"></svelte:options>

<script>
    async function copy() {
        await navigator.clipboard.writeText('cargo add tidos');

        const toast = document.createElement('div');
        toast.textContent = 'Copied to clipboard!';
        toast.style.cssText = [
            'position:fixed',
            'bottom:24px',
            'left:50%',
            'transform:translateX(-50%) translateY(0)',
            'background:#1a202c',
            'color:#f0f6fb',
            'padding:10px 22px',
            'border-radius:8px',
            "font-family:'JetBrains Mono',monospace",
            'font-size:13px',
            'box-shadow:0 4px 16px rgba(0,0,0,.5)',
            'opacity:1',
            'transition:opacity .4s,transform .4s',
            'z-index:9999',
            'pointer-events:none',
        ].join(';');
        document.body.appendChild(toast);

        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transform = 'translateX(-50%) translateY(8px)';
            setTimeout(() => toast.remove(), 400);
        }, 1800);
    }
</script>

<button class="pill" onclick={copy}>
    <span class="prompt">$</span>
    <span>cargo add tidos</span>
    <span class="copy-hint">⌘C</span>
</button>

<style>
    .pill {
        display: flex;
        align-items: center;
        gap: 14px;
        padding: 12px 26px;
        border-radius: 999px;
        background: #020a18;
        border: 1px solid rgba(255, 255, 255, 0.10);
        font-family: 'JetBrains Mono', monospace;
        font-size: 14px;
        color: #f0f6fb;
        cursor: pointer;
        transition: border-color .2s, background .2s;
        box-shadow: 0 24px 80px rgba(0, 0, 0, 0.4), 0 0 0 1px rgba(79, 209, 197, 0.13);
        height: calc(24px + 14px * 2);
    }

    .pill:hover {
        border-color: rgba(255, 255, 255, 0.25);
        background: #030e22;
    }

    .prompt {
        color: #4fd1c5;
        font-weight: 600;
        user-select: none;
    }

    .copy-hint {
        opacity: 0.4;
        padding-left: 8px;
        font-size: 12px;
        user-select: none;
    }
</style>
