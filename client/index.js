// const src = import('./../public/pkg/index.js')
// const canvas = document.getElementById('game_canvas')
// const gl = canvas.getContext("webgl2"); // add aa??

import('./../public/pkg/index')
    .catch((err) => {
        document.getElementById("game_canvas").innerText = err.message;
        console.error(err);
    });

// src.then(m => {
//     m
//     // if (!gl) {
//     //     alert("Error starting WebGL 2. " +
//     //         "Does your browser support it?")
//     //     return;
//     // }
//     //
//     // const fps_target = 1000.0/30.0;
//     // const game_app = new m.StartApp();
//     // const start_time = Date.now();
//     // let last_draw_time = -1;
//
//     //game_app.start();
//
//     // function loop() {
//     //     window.requestAnimationFrame(loop);
//     //     const current_time = Date.now();
//     //     if (current_time >= last_draw_time + fps_target) {
//     //         last_draw_time = current_time;
//     //
//     //         if (window.innerWidth !== canvas.width ||
//     //             window.innerHeight !== canvas.height)
//     //         {
//     //             canvas.height = window.innerHeight;
//     //             canvas.clientHeight = window.innerHeight;
//     //             canvas.style.height = window.innerHeight;
//     //
//     //             canvas.width = window.innerWidth;
//     //             canvas.clientWidth = window.innerWidth;
//     //             canvas.style.width = window.innerWidth;
//     //
//     //             gl.viewport(0, 0, window.innerWidth, window.innerHeight);
//     //             game_app.on_resize(elapsedTime, window.innerHeight, window.innerWidth);
//     //         }
//     //
//     //         let elapsedTime = current_time - start_time;
//     //         game_app.update();
//     //         game_app.draw();
//     //     }
//     // }
// })