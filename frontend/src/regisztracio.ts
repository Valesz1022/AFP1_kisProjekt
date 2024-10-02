let reg_elements: {
    nav_bar_login_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    reg_elements = {
        nav_bar_login_button:
            document.getElementById('nav_bar_login_button') as HTMLButtonElement
    }

    reg_elements.nav_bar_login_button.addEventListener('click', () => {
        window.api.load_login_page()
    })
})

let hibak: Array<string> = [];
function validateForm(): boolean {
    const usernameInput = document.getElementById("username") as HTMLInputElement;
    const passwordInput = document.getElementById("password") as HTMLInputElement;

    const username = usernameInput.value;
    const password = passwordInput.value;
    const specialCharPattern = /[^a-zA-Z0-9 ]/;

    hibak = [];

    if (username == null || username == "") {
        hibak.push("A felhasználónév nem lehet üres!")
    }
    if (password == null || password == "") {
        hibak.push("A jelszó nem lehet üres!")
    }
    if (specialCharPattern.test(username)) {
        hibak.push("A felhasználónév nem tartalmazhat speciális karaktert!")
    }

    return hibak.length === 0;
}

document.querySelector("form")!.addEventListener("submit", (event) => {

    const errorDiv = document.getElementById("error-messages");
    errorDiv!.innerHTML = "";

    if (!validateForm()) {

        for (let i = 0; i < hibak.length; i++) {
            const errorMessage = document.createElement("p");
            errorMessage.textContent = hibak[i];
            errorDiv!.appendChild(errorMessage);
        }


        event.preventDefault();
    }
});
