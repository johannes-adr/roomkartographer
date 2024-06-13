import init, {SmithJS} from "./pkg/smith_js.js";
import { IMURotationData, WebMovement, WebPacket } from "./smith_types.js";
import CoordinateSystem from "./coordinatesystem.js";
console.log("Starting")
async function main() {
    await init("./pkg/smith_js_bg.wasm");
    let smith = new SmithJS((await (await fetch("/schema.smith")).text()));
    const proto = location.protocol.startsWith('https') ? 'wss' : 'ws'
    const wsUri = `${proto}://${location.host}/ws`
    let ws =new WebSocket(wsUri);
    let canv = document.getElementById("lidarScan") as HTMLCanvasElement;
    let coordinatesystem = new CoordinateSystem(canv);

    function sendPacket(packet: WebPacket){
        ws.send(smith.serialize(packet,"WebPacket"));
    }
    let last = "_"

    document.body.addEventListener("keyup",ev=>{
        last = "_"
        sendPacket(WebPacket.Movement(WebMovement.Stop()))
    })
    document.body.addEventListener("keydown",ev=>{
        let k = ev.key;
        if(k == last){
            return;
        }
        last = k;
        if(k == "w"){
            sendPacket(WebPacket.Movement(WebMovement.Forward()))
        }else if(k == "a"){
            sendPacket(WebPacket.Movement(WebMovement.Left()))
        }else if(k == "s"){
            sendPacket(WebPacket.Movement(WebMovement.Stop()))
        }else if(k == "d"){
            sendPacket(WebPacket.Movement(WebMovement.Right()))
        }
    })
    let rotdisp = document.getElementById("rotdisplay")!;
    let rotations: {rotation: number,time: number}[] = [];
    let rot = 0;
    
    ws.onmessage = async (msg)=>{
        let data = msg.data;
        if(data instanceof Blob){
            let binary = new Uint8Array(await data.arrayBuffer());
            let packet: WebPacket = smith.deserialize(binary,"WebPacket");
            Object.setPrototypeOf(packet,WebPacket.prototype);
            if(packet.getTag() == "LidarScan"){
                let lidarscan = packet.as_LidarScan();
                let pts = lidarscan.points;
                let time_ms = Number.parseInt(lidarscan.time);
                
                //Find closest rotation
                if(rotations.length > 0){
                    let min = rotations[0];
                    let mindiff = Math.abs(min.time - time_ms)
                    for(let i = 1;i < rotations.length;i++){
                        let loc = rotations[i];
                        let diff = Math.abs(loc.time - time_ms);
                        if(diff < mindiff){
                            mindiff = diff;
                            min = loc;
                        }
                    }
                    rot = min.rotation;
                }

                // //Rotate points agains own robot rotation to keep room stable
                // let rot_rad = rot / 180 * Math.PI;
                // for(let p of pts){
                    
                //     let x_new = p.x * Math.cos(rot_rad) - p.y * Math.sin(rot_rad);
                //     let y_new = p.x * Math.sin(rot_rad) + p.y * Math.cos(rot_rad);
                
                //     p.x = x_new;
                //     p.y = y_new;
                // }
              

                rotdisp.innerText = rot + "";
                coordinatesystem.render(lidarscan.points,rot);
            }else if(packet.getTag() == "IMURotation"){
                let pack = packet.as_IMURotation();
                rotations.push({rotation: pack.rotation, time: Number.parseInt(pack.time)});
                if(rotations.length > 20){
                    rotations = [...rotations.slice(1)];
                }
            }
            
            
          }
    };

}

main()