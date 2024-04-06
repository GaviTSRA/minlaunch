<script>
    import { createEventDispatcher } from 'svelte';
    
    export let title;
    export let selected;

    const dispatch = createEventDispatcher();
    
    function click() {
        dispatch("selectPage")
    }
</script>

<main>
    <button on:click={click}>
        <p class="{selected ? "selected" : ""}">{title}</p>
    </button>
</main>

<style>
    main {
        margin: 0 1rem;
    }
    p {
        font-weight: 550;
        color: #CCCCCC;
    }
    button {
        background-color: #1f1f1f;
        border-style: none;
        padding: 0;
    }
    .selected {
        color: white;
        border-style: solid;
        border-width: 0;
        border-bottom-width: 2px;
        border-color: aqua;
        padding-bottom: 5px;
    }
</style>