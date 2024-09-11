let elements: {
    button: HTMLButtonElement
};

window.addEventListener('load', () => {
    elements = {
        button: document.getElementById('button') as HTMLButtonElement,
    };

    elements.button.addEventListener('click', () => {
        console.log("Button clicked")
    });
})
