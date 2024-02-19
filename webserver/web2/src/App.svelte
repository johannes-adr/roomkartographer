<script lang="ts">
    import { onMount } from "svelte";

    import init, { SmithJS } from "../public/pkg/smith_js";
    import { handleWebsocket } from "./logic/canvas_controller";
    import CoordinateSystem from "./logic/coordinatesystem";
    import KeyboardCapture from "./components/KeyboardCapture.svelte";
    let ws: WebSocket | undefined | null = undefined;

    let recording = false;

    let smith: SmithJS | undefined;
    let canv: HTMLCanvasElement;

    let record: any[] = [];
    onMount(async () => {
        await init("./pkg/smith_js_bg.wasm");
        let coordinatesystem = new CoordinateSystem(canv);
        smith = new SmithJS(await (await fetch("/schema.smith")).text());

        const proto = location.protocol.startsWith("https") ? "wss" : "ws";
        const wsUri = `${proto}://${location.host}/ws`;
        let wsl = new WebSocket(wsUri);

        if (wsl.readyState == wsl.CLOSED) {
            ws = null;
        } else {
            ws = wsl;
            ws.onmessage = async (msg) =>{
                let res = await handleWebsocket(msg, smith, (pts, rot) => {
                    coordinatesystem.render(pts, rot);
                });
                if(recording)record.push(res);
            }
        }
    });
</script>

<div id="root">
    <button on:click={()=>{
        recording = !recording;

        if(recording){
            return;
        }

        let txt = JSON.stringify(record);
        record = [];
        const blob = new Blob([txt], {type: 'text/json'});
        const elem = window.document.createElement('a');
        elem.href = window.URL.createObjectURL(blob);
        elem.download = "SLAMData.json";        
        document.body.appendChild(elem);
        elem.click();        
        document.body.removeChild(elem);
    }}
        >{#if recording}
            Stop
        {:else}
            Start
        {/if} Recording</button
    >
    <div>
    <canvas width="400px" height="400px" bind:this={canv} />

    </div>
</div>



{#if ws !== undefined && ws !== null && smith !== undefined}
    <KeyboardCapture {ws} {smith} />
{:else}
    Initing
{/if}


<style>
    #root{
        display: flex;
        flex-direction: column;
        ;
    }
</style>