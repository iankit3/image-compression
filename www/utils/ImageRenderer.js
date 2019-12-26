
HTMLCanvasElement.prototype.renderImage = function(blob){
    var ctx = this.getContext('2d');
    var img = new Image();

    img.src = URL.createObjectURL(blob);
    //img.src = 'https://mdn.mozillademos.org/files/5397/rhino.jpg';
    img.onload = function(){
        ctx.drawImage(img, 0, 0)
    }

    return img
};


export function renderImage(canvas, blob){
    console.log(canvas.renderImage(blob));
}

