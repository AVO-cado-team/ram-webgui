window.onload = function() {

    let counter = 0.0;
    let main = document.getElementById("ram-web");
    let loader = document.getElementById("loader");

    let loadingAnimTimer = window.setInterval(function() {

        if (counter >= 1.0) {
            main.style.opacity = 1.0;
            loader.style.display = "none";
            clearInterval(loadingAnimTimer);
        }
        else {
            counter += 0.1;
            main.style.opacity = counter;
        }
    }, 50);
}