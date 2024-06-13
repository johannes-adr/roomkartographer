export default class CoordinateSystem {
    private width: number;
    private height: number;
    private ctx: CanvasRenderingContext2D;
    private zoomFactor: number = 1; // Default zoom factor
    private points: {x: number,y:number}[] = []
    constructor(canvas: HTMLCanvasElement) {
        this.ctx = canvas.getContext("2d")!;
        this.width = canvas.width;
        this.height = canvas.height;
        canvas.addEventListener("wheel",ev=>{
            this.zoomFactor += ev.deltaY * 0.0001;
            if(this.zoomFactor < 0){
                this.zoomFactor = 0;
            }
            this.render(this.points, 0);
        })
        this.setZoom(0.2);
    }

    render(points: { x: number, y: number }[], rot: number) {

        this.points = points;
        // Clear the canvas before drawing new points
        this.ctx.clearRect(0, 0, this.width, this.height);

        // Set styles for the points
        this.ctx.fillStyle = 'blue'; // Color of the points
        const pointSize = 1; // Size of the points

        // Draw each point
        points.forEach(point => {
            // Transform the coordinates if necessary
            const transformedX = this.transformCoordinate(point.x, 'x');
            const transformedY = this.transformCoordinate(point.y, 'y');

            // Draw the point
            this.ctx.beginPath();
            this.ctx.arc(transformedX, transformedY, pointSize, 0, Math.PI * 2, true);
            this.ctx.fill();
        });
        this.ctx.fillText(`zoom: ${Math.round(this.zoomFactor*100)/100}`,10,20)

        this.ctx.fillStyle = 'green'; // Color of the points
        this.ctx.beginPath();

        let x = this.transformCoordinate(0, 'x'); 
        let y = this.transformCoordinate(0, 'y');
        this.ctx.arc(x,y, 5, 0, Math.PI * 2, true);
        this.ctx.fill();
        let rotpi = rot / 180 * Math.PI;
        let len = 25;
        let x2 = -Math.cos(rotpi) * len + x;
        let y2 = Math.sin(rotpi) * len + y;

        // this.ctx.beginPath();
        // this.ctx.moveTo(x,y);
        // this.ctx.lineTo(x2,y2);
        // this.ctx.stroke();
    }

    private transformCoordinate(coordinate: number, axis: 'x' | 'y'): number {
        const range = 24 * this.zoomFactor; // Adjust range based on the zoom factor
        const midPoint = range / 2;

        // Calculate the size of the canvas depending on the axis
        const canvasSize = axis === 'x' ? this.width : this.height;
        const canvasMid = canvasSize / 2;

        // Scale and translate the coordinate
        let scaledCoordinate = ((coordinate + midPoint) / range) * canvasSize;

        return scaledCoordinate;
    }

    // Method to set the zoom factor
    setZoom(zoomFactor: number) {
        this.zoomFactor = zoomFactor;
    }
}
