import {Counter, Mycanvas} from "wasm-gameof-life";

//greet();
var canvas_elm = document.getElementById("mycanvas");
console.log(canvas_elm.width);
console.log(canvas_elm.height);
var mycanvas_elm = Mycanvas.new(canvas_elm, canvas_elm.width, canvas_elm.height);
var cnt = Counter.new();
cnt.log();
mycanvas_elm.resize();
mycanvas_elm.set_width(1920);
mycanvas_elm.set_height(1080);
mycanvas_elm.resize();
//cnt.inc();
//cnt.log();