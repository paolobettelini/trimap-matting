"use strict"

// e.g. "http://localhost"
const API_URL = window.location.href.substring(0, 8 + window.location.href.substring(8).indexOf('/'));

// Canvases
var imageCanvas = document.getElementById('image');
var imageCTX = imageCanvas.getContext('2d');

// Canvase contexts
var trimapCanvas = document.getElementById('trimap');
var trimapCTX = trimapCanvas.getContext('2d', {alpha: false});

/// Image elements
var maskImg = document.getElementById('mask');
var replacementImg = document.getElementById('replacement');
var resultImg = document.getElementById('result');

/// Loading gifs
var maskLoading = document.getElementById('mask-loading');
var resultLoading = document.getElementById('result-loading');

// Transformation inputs
var colorInput = document.getElementById('color_input');
var transparentRadio = document.getElementById('transparent');
var replacementInput = document.getElementById('replacement_input');

// Represent if a set of inputs and buttons have been enabled
var trimapToolsInit = false;
var maskToolsInit = false;
var finalResultToolsInit = false;

function handleImageFileSelect(event) {
    let files = event.target.files;
    let file = files[0]; // take first one
    
    if (!validateFile(file)) {
        return;
    }

    let reader = new FileReader();
    reader.onload = event => {
        let img = new Image();
        img.onload = () => {
            imageCanvas.width = img.width;
            imageCanvas.height = img.height;
            imageCTX.drawImage(img, 0, 0);
            trimapCanvas.width = img.width;
            trimapCanvas.height = img.height;
            initTrimapCanvas();

            showTrimapTools();
        }
        img.src = event.target.result;
    };
    reader.readAsDataURL(file);
}

function handleMaskFileSelect(event) {
    fileToImg(event, maskImg);
    showMaskTools();
}

function handleReplacementFileSelect(event) {
    fileToImg(event, replacementImg);
}

function generateMask() {
    let targetImage = null;
    let trimapImage = null;

    // Just in case
    if (!areSameSize(imageCanvas, trimapCanvas)) {
        alert("The trimap image must be the same size of the target image")
        return;
    }

    imageCanvas.toBlob(blob => {
        targetImage = blob;
        sendImages(targetImage, trimapImage);
    }, 'image/png');

    trimapCanvas.toBlob(blob => {
        trimapImage = blob;
        sendImages(targetImage, trimapImage);
    }, 'image/png');

    // This function is called two times.
    // It does not execute anything until both arguments
    // are non-null. First call will have either
    // target or trimap as null.
    
    function sendImages(target, trimap) {
        if (target == null || trimap == null) {
            return;
        }

        startMaskLoading();

        let formData = new FormData();
        formData.append('target', target);
        formData.append('trimap', trimap);
    
        let xhr = new XMLHttpRequest();
    
        xhr.responseType = 'arraybuffer';
        xhr.open('POST', `${API_URL}/api/matting`);
        xhr.send(formData);
        xhr.onload = () => handleResponse(xhr);
        
        function handleResponse(xhr) {
            if (xhr.status == 200) {
        
                let imgData = new Uint8Array(xhr.response);
                let blob = new Blob([imgData], {type: 'image/png'});
                
                maskImg.src = URL.createObjectURL(blob);
                showMaskTools();
            } else {
                alert(`A server error has occured: ${xhr.statusText}`);
            }

            endMaskLoading();
        }

    }
}

/* Inputs/buttons enabling handling */

function showTrimapTools() {
    if (trimapToolsInit) {
        return;
    }

    initTools('trimap-tool');
    
    trimapToolsInit = true;
    if (maskToolsInit) {
        initTools('transform-tool');
    }
}

function showMaskTools() {
    if (maskToolsInit) {
        return;
    }

    initTools('mask-tool');

    maskToolsInit = true;
    if (trimapToolsInit) {
        initTools('transform-tool');
    }
}

function showFinalResultTools() {
    if (finalResultToolsInit) {
        return;
    }

    initTools('result-tool');

    finalResultToolsInit = true;
}

function initTools(className) {
    let tools = document.getElementsByClassName(className);
    for (let el of tools) {
        el.disabled = false;
    }
}

function fileToImg(event, imgElement) {
    let files = event.target.files;
    let file = files[0]; // take first one
    
    if (!validateFile(file)) {
        return;
    }

    let reader = new FileReader();
    reader.onload = () => {
        imgElement.src = reader.result;
    };
    reader.readAsDataURL(file);
}

/* Transformation inputs */

function transparentOption() {
    colorInput.disabled = true;
    replacementInput.disabled = true;
    replacementImg.style.opacity = 0.5;
}

function fillOption() {
    colorInput.disabled = false;
    replacementInput.disabled = true;
    replacementImg.style.opacity = 0.5;
}

function replaceOption() {
    colorInput.disabled = true;
    replacementInput.disabled = false;
    replacementImg.style.opacity = 1;
}

function applyTransformation() {
    let targetImage = null;
    let maskImage = null;
    let replacementImage = null;

    let transformation = {
        transparent: transparentRadio.checked,
        fill: colorInput.disabled ? undefined : colorInput.value,
        replace: !replacementInput.disabled,
    };

    if (!areSameSize(imageCanvas, maskImg)) {
        alert("The mask image must be the same size of the target image")
        return;
    }

    if (transformation.replace) {
        if (!areSameSize(imageCanvas, replacementImg)) {
            alert("The replacement image must be the size size of the target/mask image")
            return;
        }
    }

    imageCanvas.toBlob(blob => {
        targetImage = blob;
        sendImages(targetImage, maskImage, replacementImage, transformation);
    }, 'image/png');

    fetch(maskImg.src)
        .then(res => res.blob())
        .then(blob => {
            maskImage = blob;
            sendImages(targetImage, maskImage, replacementImage, transformation);
        });

    if (transformation.replace) {
        fetch(replacementImg.src)
            .then(res => res.blob())
            .then(blob => {
                replacementImage = blob;
                sendImages(targetImage, maskImage, replacementImage, transformation);
            });
    }

    function sendImages(target, mask, replacement) {
        if (target == null || mask == null || (transformation.replace && replacement == null) ) {
            return;
        }

        startResultLoading();

        let formData = new FormData();
        formData.append('target', target);
        formData.append('mask', mask);
        if (transformation.replace) {
            formData.append('replacement', replacement);
        }
    
        let xhr = new XMLHttpRequest();
    
        xhr.responseType = 'arraybuffer';
        let endpoint = getEndpoint(transformation);
        xhr.open('POST', `${API_URL}/api/${endpoint}`);
        xhr.send(formData);
        xhr.onload = () => handleResponse(xhr);
        
        function handleResponse(xhr) {
            if (xhr.status == 200) {
                let imgData = new Uint8Array(xhr.response);
                let blob = new Blob([imgData], {type: 'image/png'});
                
                resultImg.src = URL.createObjectURL(blob);
                showFinalResultTools();
            } else {
                alert(`A server error has occured: ${xhr.statusText}`);
            }

            endResultLoading();
        }

    }
}

// Returns the API endpoint given the transformation object
function getEndpoint(transformation) {
    if (transformation.replace) {
        return 'replace';
    }

    if (transformation.transparent) {
        return 'transparent';
    }

    if (transformation.fill != undefined) {
        // remove '#'
        return `fill/${transformation.fill.substring(1)}`;
    }

    return undefined;
}

function validateFile(file) {
    if (!file.type.match('image.*')) {
        alert("Invalid file type. Please select an image");
        return false;
    }

    if (file.size > 2 * 1024 * 1024) {
        alert("File too big. The file cannot exceed 2MB");
        return false;
    }

    return true;
}

/* Download image section */

function downloadMask() {
    downloadImage(maskImg, 'mask.png');
}

function downloadFinalResult() {
    downloadImage(resultImg, 'result.png');
}

// Downloads the content of the <img> tag to a file
function downloadImage(element, filename) {
    fetch(element.src)
        .then(image => image.blob())
        .then(blob => {
            let imageURL = URL.createObjectURL(blob);

            let link = document.createElement('a');
            link.href = imageURL;
            link.download = filename;
            link.click();
        })
}

function areSameSize(el1, el2) {
    return el1.width == el2.width && el1.height == el2.height;
}

// Loading gifs
function startMaskLoading() {
    hideElement(maskImg);
    showElement(maskLoading);
}

function endMaskLoading() {
    hideElement(maskLoading);
    showElement(maskImg);
}

function startResultLoading() {
    hideElement(resultImg);
    showElement(resultLoading);
}

function endResultLoading() {
    hideElement(resultLoading);
    showElement(resultImg);
}

function showElement(el) {
    el.classList.remove('hidden');
}

function hideElement(el) {
    el.classList.add('hidden');
}