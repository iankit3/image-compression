import * as wasm from "image-compression";
import {renderImage} from "./utils/ImageRenderer";

function printResultInDOM(result) {
  let textNode = document.createTextNode(result);
  document.body.appendChild(textNode);
}

window.handleFormSubmit = function(self, ev) {
  ev.preventDefault();
  let file = self.uploaded_file.files[0];
  let reader = new FileReader();
  reader.onload = e => {
    const u8Array = new Int8Array(reader.result);
    console.log(u8Array);
    
    var c1 = document.getElementById('c1');
    let b1 = new Blob([u8Array] , {type : file.type});
    renderImage(c1,b1)

    let compressedVec = wasm.parse_buffer(u8Array);
        //compressedVec = new Int8Array(compressedVec)
    var c2 = document.getElementById('c2');
    let b2 = new Blob([compressedVec] , {type : file.type});
    renderImage(c2,b2);
    console.log(compressedVec);


  };
  reader.readAsArrayBuffer(file);
};

window.jsLog = function(s) {
  console.log(s);
};

wasm.greet();
