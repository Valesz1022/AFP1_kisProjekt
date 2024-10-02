let reg_elements: {
    nav_bar_login_button: HTMLButtonElement
    username: HTMLInputElement
    password: HTMLInputElement
    register: HTMLButtonElement
}

window.addEventListener('load', () => {
    reg_elements = {
        nav_bar_login_button:
            document.getElementById('nav_bar_login_button') as HTMLButtonElement,
        username:
            document.getElementById('username') as HTMLInputElement,
        password:
            document.getElementById('password') as HTMLInputElement,
        register:
            document.getElementById('register') as HTMLButtonElement
    }

    reg_elements.nav_bar_login_button.addEventListener('click', () => {
        window.api.load_login_page()
    })

    reg_elements.register.addEventListener('click', () => {
        const errorDiv = document.getElementById("error-messages");
        errorDiv!.innerHTML = "";

        if (validateForm()) {
            sendUser();
        } else {

            for (let i = 0; i < hibak.length; i++) {
                const errorMessage = document.createElement("p");
                errorMessage.textContent = hibak[i];
                errorDiv!.appendChild(errorMessage);
            }
        }

    })
})

let hibak: Array<string> = [];
function validateForm(): boolean {
    const specialCharPattern = /[^a-zA-Z0-9 ]/;
    hibak = [];

    if (reg_elements.username.value == null || reg_elements.username.value == "") {
        hibak.push("A felhasználónév nem lehet üres!")
    }
    if (reg_elements.password.value == null || reg_elements.password.value == "") {
        hibak.push("A jelszó nem lehet üres!")
    }
    if (specialCharPattern.test(reg_elements.username.value)) {
        hibak.push("A felhasználónév nem tartalmazhat speciális karaktert!")
    }

    return hibak.length === 0;
}

async function sendUser() {
    let response = await fetch(`${SERVER}/register?name=${reg_elements.username.value}&password=${reg_elements.password.value}`, { method: "POST" })
    console.log('retek')
    console.log(response.status)
}
