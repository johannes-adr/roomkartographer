
export interface IMURotationData{
  rotation: number
  time: string
}

export class WebMovement{
   private tag: 'Forward' | 'Stop' | 'Left' | 'Right' | 'Speed'
   private val?: number
   private constructor(tag: 'Forward' | 'Stop' | 'Left' | 'Right' | 'Speed', val?: number){
     this.tag = tag;
     this.val = val;
   }
   
   static Forward(){return new WebMovement("Forward",)}
   as_Forward(){
      if (this.tag != 'Forward'){
         throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Forward'")
      }
          
   }

   static Stop(){return new WebMovement("Stop",)}
   as_Stop(){
      if (this.tag != 'Stop'){
         throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Stop'")
      }
          
   }

   static Left(){return new WebMovement("Left",)}
   as_Left(){
      if (this.tag != 'Left'){
         throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Left'")
      }
          
   }

   static Right(){return new WebMovement("Right",)}
   as_Right(){
      if (this.tag != 'Right'){
         throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Right'")
      }
          
   }

   static Speed(v: number){return new WebMovement("Speed",v)}
   as_Speed(){
      if (this.tag != 'Speed'){
         throw new Error("Enum WebMovement: trying to cast variant '" + this.tag + "' into 'Speed'")
      }
      return this.val as number    
   }
   getTag(){ return this.tag }
}

export class WebPacket{
   private tag: 'LidarScan' | 'IMURotation' | 'Movement'
   private val: WebMovement | IMURotationData | WebLidarScan
   private constructor(tag: 'LidarScan' | 'IMURotation' | 'Movement', val: WebMovement | IMURotationData | WebLidarScan){
     this.tag = tag;
     this.val = val;
   }
   
   static LidarScan(v: WebLidarScan){return new WebPacket("LidarScan",v)}
   as_LidarScan(){
      if (this.tag != 'LidarScan'){
         throw new Error("Enum WebPacket: trying to cast variant '" + this.tag + "' into 'LidarScan'")
      }
      return this.val as WebLidarScan    
   }

   static IMURotation(v: IMURotationData){return new WebPacket("IMURotation",v)}
   as_IMURotation(){
      if (this.tag != 'IMURotation'){
         throw new Error("Enum WebPacket: trying to cast variant '" + this.tag + "' into 'IMURotation'")
      }
      return this.val as IMURotationData    
   }

   static Movement(v: WebMovement){return new WebPacket("Movement",v)}
   as_Movement(){
      if (this.tag != 'Movement'){
         throw new Error("Enum WebPacket: trying to cast variant '" + this.tag + "' into 'Movement'")
      }
      return this.val as WebMovement    
   }
   getTag(){ return this.tag }
}

export interface WebLidarScan{
  points: WebPoint[]
  time: string
}

export interface WebPoint{
  x: number
  y: number
}
