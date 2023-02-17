"use strict"

var backgroundColor = '#000000';
var foregroundColor = '#FFFFFF';
var borderColor = '#808080';

let intColor1 = hexToInt(borderColor);
let intColor2 = hexToInt(foregroundColor);
let intColor3 = hexToInt(borderColor);

var trimapOpacity = 0.75;

var brushThickness = 20;
var brushColor = backgroundColor;
var fillBrush = false;

var dragging = false;
var lines = [];
var currentLine = {};

trimapCanvas.onmousedown = e => {
    let x = e.offsetX;
    let y = e.offsetY;

    currentLine = {
        size: brushThickness,
        background: brushColor,
        fill: fillBrush,
        points: [{x: x, y: y}]
    }

    if (fillBrush) { // Fill tool
        fill(brushColor, x, y);

        lines.push(currentLine)
    } else { // Start drawing line
        trimapCTX.lineCap = 'round';
        trimapCTX.lineJoin = 'round';
        trimapCTX.strokeStyle = brushColor;
        trimapCTX.lineWidth = brushThickness;
        
        trimapCTX.beginPath();
        trimapCTX.moveTo(x, y);
     
        dragging = true;
    }
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
    trimapOpacity = v;
    trimapCanvas.style.opacity = v;
}

function setBackground() {
    brushColor = backgroundColor;
}

function setForeground() {
    brushColor = foregroundColor;
}

function setBorder() {
    brushColor = borderColor;
}

function toggleFill() {
    fillBrush = !fillBrush;
}

function redraw() {
    clearTrimapCanvas();
    
    // draw lines
    for (let i = 0; i < lines.length; i++) {
        let line = lines[i];

        if (line.fill) { // Fill tool
            let point = line.points[0];

            fill(line.background, point.x, point.y);
        } else { // Draw line
            if (line.points.length == 1) {
                continue;
            }
        
            trimapCTX.beginPath();
            
            trimapCTX.lineWidth = line.size;
            trimapCTX.strokeStyle = line.background;
        
            // draw points
            let points = line.points;
            
            trimapCTX.moveTo(points[0].x, points[0].y);
            for (let i = 1; i < line.points.length; i++) {
                trimapCTX.lineTo(points[i].x, points[i].y)
            }
        
            trimapCTX.stroke()
        }
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

function clearTrimapCanvas() {
    trimapCTX.fillStyle = borderColor;
    trimapCTX.fillRect(0, 0, trimapCanvas.width, trimapCanvas.height);
}

function deleteHistory() {
    lines = [];
    currentLine = {};
}

function initTrimapCanvas() {
    setTrimapOpacity(trimapOpacity);
    clearTrimapCanvas();
    deleteHistory();

    // does this work?
    trimapCTX.imageSmoothingEnabled = false;
}

function fill(color, x, y) {
    let imageData = trimapCTX.getImageData(0, 0, trimapCanvas.width, trimapCanvas.height);
    let clickedColor = getPixelColor(imageData, x, y);
    let fillColor = hexToInt(color);
    let stack = [];
    stack.push({x, y});

    if (clickedColor == fillColor) {
        // Avoid infinite loop
        return;
    }

    while (stack.length > 0) {
        let point = stack.pop();
        if (point.x < 0 || point.x >= trimapCanvas.width || point.y < 0 || point.y >= trimapCanvas.height) {
            continue;
        }

        let pixelIndex = (point.y * imageData.width + point.x) * 4;
        if (!isSameColor(imageData, pixelIndex, clickedColor)) {
            continue;
        }

        setPixelColor(imageData, pixelIndex, fillColor);

        stack.push({x: point.x + 1, y: point.y});
        stack.push({x: point.x - 1, y: point.y});
        stack.push({x: point.x, y: point.y + 1});
        stack.push({x: point.x, y: point.y - 1});
    }

    trimapCTX.putImageData(imageData, 0,0);

    function getPixelColor(imageData, x, y) {
        const pixelIndex = (y * imageData.width + x) * 4;
        let r = imageData.data[pixelIndex];
        let g = imageData.data[pixelIndex + 1];
        let b = imageData.data[pixelIndex + 2];
        return b | (g << 8) | (r << 16);
    }

    function isSameColor(imageData, pixelIndex, color) {
        let r = imageData.data[pixelIndex];
        let g = imageData.data[pixelIndex + 1];
        let b = imageData.data[pixelIndex + 2];
        let imageColor = b | (g << 8) | (r << 16);

        // These ifs implement a tolerance (only consider the other two colors)
        if (intColor1 == color) {
            return imageColor != intColor2 && imageColor != intColor3;
        }

        if (intColor2 == color) {
            return imageColor != intColor1 && imageColor != intColor3;
        }

        if (intColor3 == color) {
            return imageColor != intColor1 && imageColor != intColor2;
        }

        // zero tolerance. Does not work because of antialiasing
        // return imageColor == color;
    }

    function setPixelColor(imageData, pixelIndex, color) {
        let r = (color >> 16) & 0xFF;
        let g = (color >> 8) & 0xFF;
        let b = color & 0xFF;

        imageData.data[pixelIndex] = r;
        imageData.data[pixelIndex + 1] = g;
        imageData.data[pixelIndex + 2] = b;
    }
}

function hexToInt(color) {
    let r = parseInt(color.substr(1, 2), 16);
    let g = parseInt(color.substr(3, 2), 16);
    let b = parseInt(color.substr(5, 2), 16);

    return b | (g << 8) | (r << 16);
}