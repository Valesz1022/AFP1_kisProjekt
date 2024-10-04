let new_post_elements: {
    back_button: HTMLButtonElement,
    title_box: HTMLInputElement,
    content_box: HTMLTextAreaElement,
    post_joke_button: HTMLButtonElement
    user_name: string
}

window.addEventListener('load', () => {
    new_post_elements = {
        back_button: document.getElementById('back_to_main_page') as HTMLButtonElement,
        title_box: document.getElementById('title_box') as HTMLInputElement,
        content_box: document.getElementById('content_box') as HTMLTextAreaElement,
        post_joke_button: document.getElementById('post_joke') as HTMLButtonElement,
        user_name: localStorage.getItem('globalUsername') as string
    }
    reLogin2(localStorage.getItem('globalUsername'), localStorage.getItem('globalPassword'));

    new_post_elements.back_button.addEventListener('click', () => {
        if (new_post_elements.user_name == "admin") {
            window.api.load_main_page_admin();
        }
        else {
            window.api.load_main_page_user();
        }
    })

    new_post_elements.post_joke_button.addEventListener('click', () => {
        post_joke(new_post_elements.user_name, new_post_elements.content_box.value);
    })

})

async function post_joke(username: string, content: string) {
    let response = await fetch(`${SERVER}/jokes?user_name=${username}&content=${content}`, {
        method: "POST"
    });

    switch (response.status) {
        case 201:
            console.log("Sikeres posztolás");
            break;
        case 401:
            console.log("Nincs bejelentkezve felhasználó.");
            break;
        case 409:
            console.log("Már van ilyen vicc");
            break;
        case 422:
            console.log("Hibás kérés paraméterek.");
            break;
        case 500:
            console.log("Valami hiba történt a szerveren.");
    }
}

async function reLogin2(username: string | null, password: string | null) {
    let response = await fetch(`${SERVER}/login?name=${username}&password=${password}`, {
        method: "POST"
    });

    switch (response.status) {
        case 200:
            console.log("újra bejelentkezve");
            break;
    }
}