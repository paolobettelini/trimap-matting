"use strict"

let brushThickness = 20;
let background = true;

let dragging = false;

let lines = [];
let currentLine = {};

trimapCanvas.onmousedown = e => {
    trimapCTX.lineCap = "round";
    trimapCTX.strokeStyle = background ? 'black' : 'white';
    trimapCTX.lineWidth = brushThickness;

    let x = e.offsetX;
    let y = e.offsetY;

    trimapCTX.beginPath();
    trimapCTX.moveTo(x, y);

    currentLine = {
        size: brushThickness,
        background: background,
        points: [{x: x, y: y}]
    }

    dragging = true;
}

trimapCanvas.onmousemove = e => {
    let x = e.offsetX;
    let y = e.offsetY;
    
    if (dragging) {
        trimapCTX.lineTo(x, y)
        trimapCTX.stroke();
        currentLine.points.push({x: x, y: y});
    }
}

trimapCanvas.onmouseup = _e => {
    if (dragging) { // avoid if something interrupted dragging mode
        lines.push(currentLine);
    }

    dragging = false;
}

function setBrushThickness(v) {
    brushThickness = v;
    trimapCTX.lineWidth = brushThickness;
}

function setTrimapOpacity(v) {
    trimapCanvas.style.opacity = v;
}

function setBackground(isBackground) {
    background = isBackground;
}

function redraw() {
    clearCanvas();
    
    // draw lines
    for (let i = 0; i < lines.length; i++) {
        let line = lines[i];
        if (line.points.length == 1) {
            continue;
        }

        trimapCTX.beginPath();
        
        trimapCTX.lineWidth = line.size;
        trimapCTX.strokeStyle = line.background ? 'black' : 'white';
    
        // draw points
        let points = line.points;
        
        trimapCTX.moveTo(points[0].x, points[0].y);
        for (let i = 1; i < line.points.length; i++) {
            trimapCTX.lineTo(points[i].x, points[i].y)
        }

        trimapCTX.stroke()
    }
}

// Undo
document.onkeydown = e => {
    if (e.key == 'z' && e.ctrlKey) {
        undo();
    }
}

function undo() {
    if (dragging) {
        dragging = false;
        redraw();
    } else {
        lines.pop();
        redraw();
    }
}

function clearCanvas() {
    trimapCTX.clearRect(0, 0, trimapCanvas.width, trimapCanvas.height);
}

function deleteHistory() {
    lines = [];
    currentLine = {};
}
