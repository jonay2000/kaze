Object.assign(window.search, {"doc_urls":["index.html#building-a-cpu-using-kaze","chapter_01_logic_gates.html#chapter-1-logic-gates"],"index":{"documentStore":{"docInfo":{"0":{"body":80,"breadcrumbs":4,"title":4},"1":{"body":0,"breadcrumbs":4,"title":4}},"docs":{"0":{"body":"Kaze is a library which allows users to describe hardware like a hardware descriptor language, as part of Rust source code. In this book all Kaze's features are described by guiding you through an example of a project in Kaze: building a CPU. CPU, as you might already know, stands for \"Central Processing Unit\" and is the heart of any computer. You might even say that the CPU is the computer and any other parts are just peripherals. This guide will be split into a number of chapter, also displayed in the sidebar. We will start with making some simple logic gates in Kaze, which will then be expanded to an Arithmetic and Logic Unit (ALU). After this we will talk about registers and memory, and how these, together with the ALU form a data loop. Next up will be control logic and conditional execution. More chapters may be added in the future such as caching, speculative execution and different computer architectures.","breadcrumbs":"Building a CPU using Kaze","id":"0","title":"Building a CPU using Kaze"},"1":{"body":"","breadcrumbs":"chapter 1: Logic gates","id":"1","title":"chapter 1: Logic gates"}},"length":2,"save":true},"fields":["title","body","breadcrumbs"],"index":{"body":{"root":{"1":{"df":1,"docs":{"1":{"tf":1.0}}},"a":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"0":{"tf":1.0}}}}},"r":{"df":0,"docs":{},"e":{"a":{"d":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"u":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"r":{"c":{"df":0,"docs":{},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}}},"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{}}}}},"c":{"a":{"c":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":2,"docs":{"0":{"tf":1.4142135623730951},"1":{"tf":1.0}}}}}}},"df":0,"docs":{}},"o":{"d":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}}}},"n":{"d":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"p":{"df":0,"docs":{},"u":{"df":1,"docs":{"0":{"tf":2.0}}}}},"d":{"a":{"df":0,"docs":{},"t":{"a":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"c":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"b":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"df":0,"docs":{}}},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"f":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"s":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"y":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}},"x":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"df":0,"docs":{}},"p":{"a":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{}}}},"f":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"df":1,"docs":{"0":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"g":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"df":0,"docs":{},"u":{"df":0,"docs":{},"i":{"d":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{}}}},"h":{"a":{"df":0,"docs":{},"r":{"d":{"df":0,"docs":{},"w":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"k":{"a":{"df":0,"docs":{},"z":{"df":0,"docs":{},"e":{"'":{"df":1,"docs":{"0":{"tf":1.0}}},"df":1,"docs":{"0":{"tf":2.0}}}}},"df":0,"docs":{},"n":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"l":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"i":{"c":{"df":2,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.0}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"p":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"m":{"a":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"m":{"b":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"r":{"df":0,"docs":{},"o":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{},"j":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"g":{"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"s":{"df":0,"docs":{},"i":{"d":{"df":0,"docs":{},"e":{"b":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"c":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}}},"p":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"t":{"a":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}},"u":{"c":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}},"t":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"u":{"df":0,"docs":{},"n":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"p":{"df":1,"docs":{"0":{"tf":1.0}}},"s":{"df":1,"docs":{"0":{"tf":1.0}},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"breadcrumbs":{"root":{"1":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}},"a":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"0":{"tf":1.0}}}}},"r":{"df":0,"docs":{},"e":{"a":{"d":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"u":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"r":{"c":{"df":0,"docs":{},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}}},"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}},"df":0,"docs":{}}}}},"c":{"a":{"c":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":2,"docs":{"0":{"tf":1.4142135623730951},"1":{"tf":1.4142135623730951}}}}}}},"df":0,"docs":{}},"o":{"d":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}}}},"n":{"d":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"p":{"df":0,"docs":{},"u":{"df":1,"docs":{"0":{"tf":2.23606797749979}}}}},"d":{"a":{"df":0,"docs":{},"t":{"a":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"c":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"b":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"df":0,"docs":{}}},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"f":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"s":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"y":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}},"x":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"df":0,"docs":{}},"p":{"a":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{}}}},"f":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"df":1,"docs":{"0":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"g":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.4142135623730951}}}}},"df":0,"docs":{},"u":{"df":0,"docs":{},"i":{"d":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{}}}},"h":{"a":{"df":0,"docs":{},"r":{"d":{"df":0,"docs":{},"w":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"k":{"a":{"df":0,"docs":{},"z":{"df":0,"docs":{},"e":{"'":{"df":1,"docs":{"0":{"tf":1.0}}},"df":1,"docs":{"0":{"tf":2.23606797749979}}}}},"df":0,"docs":{},"n":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"l":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"i":{"c":{"df":2,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.4142135623730951}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"p":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"m":{"a":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"m":{"b":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"r":{"df":0,"docs":{},"o":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{},"j":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"g":{"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"s":{"df":0,"docs":{},"i":{"d":{"df":0,"docs":{},"e":{"b":{"a":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"c":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}}},"p":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"t":{"a":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}},"u":{"c":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}},"t":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"u":{"df":0,"docs":{},"n":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"p":{"df":1,"docs":{"0":{"tf":1.0}}},"s":{"df":1,"docs":{"0":{"tf":1.4142135623730951}},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"title":{"root":{"1":{"df":1,"docs":{"1":{"tf":1.0}}},"b":{"df":0,"docs":{},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}}}}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}}},"df":0,"docs":{}},"p":{"df":0,"docs":{},"u":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"g":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{}},"k":{"a":{"df":0,"docs":{},"z":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"i":{"c":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}}}},"u":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"pipeline":["trimmer","stopWordFilter","stemmer"],"ref":"id","version":"0.9.5"},"results_options":{"limit_results":30,"teaser_word_count":30},"search_options":{"bool":"OR","expand":true,"fields":{"body":{"boost":1},"breadcrumbs":{"boost":1},"title":{"boost":2}}}});