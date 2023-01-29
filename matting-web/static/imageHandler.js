"use strict"

var imageCanvas = document.getElementById('image');
var imageCTX = imageCanvas.getContext('2d');

var trimapCanvas = document.getElementById('trimap');
var trimapCTX = trimapCanvas.getContext('2d');

var resultImg = document.getElementById('result');

function handleFileSelect(event) {
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
        }
        img.src = event.target.result;
    };
    reader.readAsDataURL(file);
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

// TODO dynamic
const API_URL = 'http://localhost:8080';

function processImage() {
    let targetImage;
    let trimapImage;

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

        let formData = new FormData();
        formData.append('target', target);
        //formData.append('trimap', trimap);
        formData.append('mask', trimap);
    
        let xhr = new XMLHttpRequest();
    
        xhr.responseType = 'arraybuffer';
        xhr.open('POST', `${API_URL}/api/upload`);
        xhr.send(formData);
        xhr.onload = () => handleResponse(xhr);

    }
}

function handleResponse(xhr) {
    if (xhr.status == 200) {
        let imgData = new Uint8Array(xhr.response);
        let blob = new Blob([imgData], {type: 'image/png'});
        
        resultImg.src = URL.createObjectURL(blob);
    }
}