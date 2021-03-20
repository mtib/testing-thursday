
let wc = document.getElementById("wrong-change");
let cc = document.getElementById("correct-change");

window.setTimeout(() => {
    wc.classList.add("bg-red");
    window.setTimeout(() => {
        cc.classList.remove("zero-mw");
        cc.classList.add("bg-green");
        window.setTimeout(() => {
            wc.classList.add("zero-mw");
            window.setTimeout(() => {
                cc.classList.remove("bg-green");
            }, 2000)
        }, 4000)
    }, 500)
}, 500)

