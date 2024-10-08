let main_page_user_elements: {
    posts_container: HTMLDivElement
    up_vote_buttons: HTMLCollection
    down_vote_buttons: HTMLCollection
    new_post_button: HTMLButtonElement
    logout_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    main_page_user_elements = {
        posts_container: document.getElementById('posts_container') as HTMLDivElement,
        up_vote_buttons: document.getElementsByClassName('fa-arrow-up') as HTMLCollection,
        down_vote_buttons: document.getElementsByClassName('fa-arrow-down') as HTMLCollection,
        new_post_button: document.getElementById('new_post_button') as HTMLButtonElement,
        logout_button: document.getElementById('logout_button') as HTMLButtonElement
    }

    //viccek betöltése
    get_jokes();

    //kijelentkezés funkció
    main_page_admin_elements.logout_button.addEventListener('click', () => {
        logout();
        localStorage.clear();
    })

    //Új vicc oldal
    main_page_admin_elements.new_post_button.addEventListener('click', () => {
        window.api.load_new_post();
    })


})
async function refresh_vote_count2(joke_id: string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}`, {
        method: "GET"
    });

    let number: Types.Vote = await response.json();
    console.log("A szavazat lekérése: " + number + number.vote);
    console.log(number.vote);

    switch (response.status) {
        case 200:
            let upvote_number_p = document.getElementsByName(`${joke_id}_vote_number`)[0] as HTMLParagraphElement;
            if (upvote_number_p) {
                upvote_number_p.textContent = number.vote.toString();
                console.log("szavazat szám frissítve");
            }
            break;
        case 422:
            console.log("Hibás paraméterek");
            break;
        case 500:
            console.log("Hiba a szerveren");
            break;
    }
}

async function up_vote_joke2(joke_id: string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}&vote=1`, {
        method: "POST",
        credentials: "include"
    });

    console.log(response.body);

    switch (response.status) {
        case 201:
            console.log("sikeres szavazás");
            let upvote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLIFrameElement;
            upvote_button.classList.add('voted');
            refresh_vote_count(joke_id);
            break;
        case 401:
            console.log("Nincs bejelentkezve");
            break;
        case 404:
            console.log("nincs ilyen azonosító");
            break;
        case 422:
            console.log("Hibás paraméterek");
            break;
        case 500:
            console.log("Hiba a szerveren");
            break;
    }

}

async function down_vote_joke2(joke_id: string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}&vote=${-1}`, {
        method: "POST",
        credentials: "include"
    });

    switch (response.status) {
        case 201:
            console.log("sikeres szavazás");
            let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLIFrameElement;
            down_vote_button.classList.add('voted');
            refresh_vote_count(joke_id);
            break;
        case 401:
            console.log("Nincs bejelentkezve");
            break;
        case 404:
            console.log("nincs ilyen azonosító");
            break;
        case 422:
            console.log("Hibás paraméterek");
            break;
        case 500:
            console.log("Hiba a szerveren");
            break;
    }
}


async function change_vote2(joke_id: string, vote: string) {
    let response = await fetch(`${SERVER}/jokes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}&vote=${vote}`, {
        method: "PUT",
        credentials: "include"
    });

    switch (response.status) {
        case 200:
            let up_vote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLButtonElement;
            let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLButtonElement;
            if (vote === '1') {
                up_vote_button.classList.add('voted');
                down_vote_button.classList.remove('voted');
                console.log("Szavazat módosítva upvote-ra");
            }
            if (vote === '-1') {
                up_vote_button.classList.remove('voted');
                down_vote_button.classList.add('voted');
                console.log("Szavazat módosítva downvote-ra");
            }
            refresh_vote_count(joke_id);
            break;
        case 401:
            console.log("Nincs bejelentkezve a felhasználó");
            break;
        case 409:
            console.log("Nincs ilyen azonosító, vagy erre még nem szavazott");
            break;
        case 422:
            console.log("Hibás paraméterek");
            break;
        case 500:
            console.log("Valami hiba történt a szerveren");
            break;
    }
}



async function logout2() {
    let response = await fetch(`${SERVER}/logout`, {
        method: "GET",
        credentials: "include"
    });

    switch (response.status) {
        case 200:
            localStorage.clear();
            window.api.load_login_page();
            break;
        case 401:
            console.log("Sikertelen kijelentkezés, nincsen bejelentkezve a felhasználó.");
            break;
        case 500:
            console.log("Sikertelen kijelentkezés, valamilyen szerveroldali hiba miatt");
            break;
    }
}

async function apply_voted2(name: string | null, joke_id: number) {
    let response = await fetch(`${SERVER}/votes?name=${name}&joke_id=${joke_id}`, {
        method: "GET",
        credentials: "include"
    });

    let vote: Types.Vote = await response.json();
    if (vote.vote == 1) {
        let up_vote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLButtonElement;
        up_vote_button.classList.add('voted');
    }
    if (vote.vote == -1) {
        let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLButtonElement;
        down_vote_button.classList.add('voted');
    }
}