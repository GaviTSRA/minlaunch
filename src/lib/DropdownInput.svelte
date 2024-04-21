<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let options: Array<string>;

    $: selected = options[0];
    $: selecting = false;

    function select(value: string) {
        selecting = false;
        selected = value;
        dispatch("select", value)
    }
</script>

<main>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="container" on:click={()=>selecting=!selecting}>
        <p class="selected" style="--radius:{selecting ? '0px' : '10px'}">{selected}</p>
        {#if selecting}
            <div class="options">
                <div class="optionsInner">
                    {#each options as option}
                    <div class="option" on:click|stopPropagation={()=>select(option)}>
                        <p>{option}</p>
                    </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>
</main>

<style>
    .option > p {
        margin: 0;
    }
    .option {
        position:static;
        width: 15rem;
        height: 2rem;
        margin: 0 .5rem;
    }
    .selected {
        height: 1.5rem;
        background-color: #1f1f1f;
        padding: .25rem .5rem;
        text-align: left;
        border-top-left-radius: 10px;
        border-top-right-radius: 10px;
        border-bottom-left-radius: var(--radius);
        border-bottom-right-radius: var(--radius);
    }
    .container {
        width: 17.5rem;
        position: relative;
    }
    .options {
        width: calc(100% - 6px);
        max-height: 15rem;
        overflow: hidden;
        text-align: left;
        background-color: #252525;
        position: absolute;
        top: 3rem;
        left: 0;
        z-index: 4;
        border-bottom-left-radius: 10px;
        border-bottom-right-radius: 10px;
        border-style: solid;
        border-color: #1f1f1f;
        border-width: 3px;
    }
    .optionsInner {
        scrollbar-color: #2f2f2f #252525;
        overflow-y:scroll;
        max-height: 15rem;
    }
</style>