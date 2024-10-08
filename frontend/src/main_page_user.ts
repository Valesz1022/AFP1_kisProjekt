let main_page_user_elements: {
    posts_container: HTMLDivElement
    up_vote_buttons: HTMLCollectionOf<Element>
    down_vote_buttons: HTMLCollectionOf<Element>
    new_post_button: HTMLButtonElement
    logout_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    main_page_user_elements = {
        posts_container: document.getElementById('posts_container') as HTMLDivElement,
        up_vote_buttons: document.getElementsByClassName('fa-arrow-up') as HTMLCollectionOf<Element>,
        down_vote_buttons: document.getElementsByClassName('fa-arrow-down') as HTMLCollectionOf<Element>,
        new_post_button: document.getElementById('new_post_button') as HTMLButtonElement,
        logout_button: document.getElementById('logout_button') as HTMLButtonElement
    }

    //viccek betöltése
    get_jokes2();

    //kijelentkezés funkció
    main_page_user_elements.logout_button.addEventListener('click', () => {
        logout2();
        localStorage.clear();
    });

    //Új vicc oldal
    main_page_user_elements.new_post_button.addEventListener('click', () => {
        window.api.load_new_post();
    });

});
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
            refresh_vote_count2(joke_id);
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
            refresh_vote_count2(joke_id);
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

async function delete_vote2(joke_id: string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}`, {
        method: "DELETE",
        credentials: "include"
    });

    switch (response.status) {
        case 200:
            let up_vote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLButtonElement;
            let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLButtonElement;
            up_vote_button.classList.remove('voted');
            down_vote_button.classList.remove('voted');
            console.log("Szavazás törölve");
            refresh_vote_count2(joke_id);
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
            refresh_vote_count2(joke_id);
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

async function get_jokes2() {
    console.log("viccek lekérése");
    let response = await fetch(`${SERVER}/jokes`, {
        method: "GET"
    });

    let jokes = await response.json();
    console.log("megjöttek a viccek", response.status);

    switch (response.status) {
        case 200:
            console.log("sikeres lekérés");
            jokes.forEach((joke: Types.Joke) => {
                let post = document.createElement('div');
                post.classList.add('post');

                let post_left = document.createElement('div');
                post_left.classList.add('post_left');

                let post_left_top = document.createElement('div');
                post_left_top.classList.add('post_left_top');

                let post_left_top_left = document.createElement('div');
                post_left_top_left.classList.add('post_left_top_left');

                let post_username = document.createElement('p');
                post_username.classList.add('post_username');
                post_username.textContent = joke.user_name;

                let post_title = document.createElement('p');
                post_title.classList.add('post_title');
                post_title.textContent = joke.id.toString();

                post_left_top_left.appendChild(post_username);
                post_left_top_left.appendChild(post_title);

                let post_left_bottom = document.createElement('div');
                post_left_bottom.classList.add('post_left_bottom');

                let post_content = document.createElement('p');
                post_content.classList.add('post_content');
                post_content.textContent = joke.content;

                post_left_bottom.appendChild(post_content);
                post_left.appendChild(post_left_top);
                post_left.appendChild(post_left_bottom);

                let post_right = document.createElement('div');
                post_right.classList.add('post_right');

                let up_vote = document.createElement('div');
                up_vote.classList.add('up_vote');
                let up_vote_icon = document.createElement('i');
                up_vote_icon.classList.add('fa-solid', 'fa-arrow-up', 'fa-2x');
                up_vote_icon.setAttribute('id', `${joke.id}`);
                up_vote_icon.setAttribute('name', `${joke.id}_upvote`);
                up_vote.appendChild(up_vote_icon);

                let vote_number = document.createElement('div');
                let number = document.createElement('p');
                number.textContent = joke.votes.toString();
                number.setAttribute('id', `${joke.id}_vote_number`);
                vote_number.appendChild(number);

                let down_vote = document.createElement('div');
                down_vote.classList.add('down_vote');
                let down_vote_icon = document.createElement('i');
                down_vote_icon.classList.add('fa-solid', 'fa-arrow-down', 'fa-2x');
                down_vote_icon.setAttribute('id', `${joke.id}`);
                down_vote_icon.setAttribute('name', `${joke.id}_downvote`);
                down_vote.appendChild(down_vote_icon);

                post_right.appendChild(up_vote);
                post_right.appendChild(vote_number);
                post_right.appendChild(down_vote);

                post.appendChild(post_left);
                post.appendChild(post_right);

                main_page_user_elements.posts_container.appendChild(post);

                apply_voted2(localStorage.getItem('globalUsername'), joke.id);
            });

            Array.from(main_page_user_elements.up_vote_buttons).forEach((up_vote_button) => {
                up_vote_button.addEventListener('click', () => {
                    let joke_id = up_vote_button.getAttribute('id');
                    let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLButtonElement;
                    if (joke_id) {
                        if (up_vote_button.classList.contains('voted')) {
                            delete_vote2(joke_id);
                        } else if (down_vote_button.classList.contains('voted')) {
                            change_vote2(joke_id, '1');
                        } else {
                            up_vote_joke2(joke_id);
                        }
                    }
                });
            });

            Array.from(main_page_user_elements.down_vote_buttons).forEach((down_vote_button) => {
                down_vote_button.addEventListener('click', () => {
                    let joke_id = down_vote_button.getAttribute('id');
                    let up_vote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLButtonElement;
                    if (joke_id) {
                        if (down_vote_button.classList.contains('voted')) {
                            delete_vote2(joke_id);
                        } else if (up_vote_button.classList.contains('voted')) {
                            change_vote2(joke_id, '-1');
                        } else {
                            down_vote_joke2(joke_id);
                        }
                    }
                });
            });
            break;
        case 500:
            console.log("Hiba történt");
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