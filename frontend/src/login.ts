let login_elements: {
    nav_bar_reg_button: HTMLButtonElement
    login_username: HTMLInputElement
    login_password: HTMLInputElement
    login_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    login_elements = {
        nav_bar_reg_button:
            document.getElementById('nav_bar_reg_button') as HTMLButtonElement,
        login_username:
            document.getElementById('login_username') as HTMLInputElement,
        login_password:
            document.getElementById('login_password') as HTMLInputElement,
        login_button:
            document.getElementById('login_button') as HTMLButtonElement
    }

    login_elements.nav_bar_reg_button.addEventListener('click', () => {
        window.api.load_reg_page()
    })

    login_elements.login_button.addEventListener('click', (e) => {
        sendLoginInfo(login_elements.login_username.value, login_elements.login_password.value);
    })
})

async function sendLoginInfo(username: string, password: string) {
    let response = await fetch(`${SERVER}/login?name=${username}&password=${password}`, {
        method: "POST",
        credentials: "include"
    });

    let user : Types.user = await response.json();
    
    switch (response.status) {
        
        case 200:
            localStorage.setItem('globalUsername', user.username);
            localStorage.setItem('globalPassword', password);
            localStorage.setItem('globalIsAdmin', user.admin);
            if (user.admin == "1") {
                window.api.load_main_page_admin();
            }
            else {
                window.api.load_main_page_user();
            }
            break;
        case 401:
            console.log("Sikertelen bejelentkezés valamilyen hiba miatt");
            break;
        case 500:
            console.log(" Sikertelen bejelentkezés szerveroldali hiba miatt");
            break;
    }
}

