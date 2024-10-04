let main_page_admin_elements: {
    posts_container: HTMLDivElement
    delete_buttons: HTMLCollection
    up_vote_buttons: HTMLCollection
    down_vote_buttons: HTMLCollection
    new_post_button: HTMLButtonElement
    logout_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    main_page_admin_elements = {
        posts_container: document.getElementById('posts_container') as HTMLDivElement,
        delete_buttons: document.getElementsByClassName('fa-trash') as HTMLCollection,
        up_vote_buttons: document.getElementsByClassName('fa-arrow-up') as HTMLCollection,
        down_vote_buttons: document.getElementsByClassName('fa-arrow-down') as HTMLCollection,
        new_post_button: document.getElementById('new_post_button') as HTMLButtonElement,
        logout_button: document.getElementById('logout_button') as HTMLButtonElement
    }
    reLogin(localStorage.getItem('globalUsername'), localStorage.getItem('globalPassword'));

    //viccek betöltése
    get_jokes();

    //kijelentkezés funkció
    main_page_admin_elements.logout_button.addEventListener('click', () => {
        sessionStorage.removeItem('isLoggedIn');
        localStorage.removeItem('globalUsername');
        logout();
    })

    //Új vicc oldal
    main_page_admin_elements.new_post_button.addEventListener('click', () => {
        window.api.load_new_post();
    })


})

async function referesh_vote_count(joke_id: string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}joke_id=${joke_id}`, {
        method: "GET"
    });

    let number = await response.json();

    switch (response.status) {
        case 200:
            let upvote_number_p = document.getElementsByName(`${joke_id}_number`)[0] as HTMLTextAreaElement;
            upvote_number_p.value = number;
            console.log("szavazat szám frissítve");
            break;
        case 422:
            console.log("Hibás paraméterek");
            break;
        case 500:
            console.log("Hiba a szerveren");
            break;
    }
}

async function up_vote_joke(joke_id: string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}&vote=1`, {
        method: "POST"
    });

    switch (response.status) {
        case 201:
            console.log("sikeres szavazás");
            let upvote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLIFrameElement;
            upvote_button.classList.add('voted');
            referesh_vote_count(joke_id);
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

async function down_vote_joke(joke_id: string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}&vote=-1`, {
        method: "POST"
    });

    switch (response.status) {
        case 201:
            console.log("sikeres szavazás");
            let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLIFrameElement;
            down_vote_button.classList.add('voted');
            referesh_vote_count(joke_id);
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

async function delete_vote(joke_id:string) {
    let response = await fetch(`${SERVER}/votes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}`, {
        method: "DELETE"
    });

    switch(response.status){
        case 200:
            let up_vote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLButtonElement;
            up_vote_button.classList.remove('voted');
            console.log("Szavazás törölve");
            referesh_vote_count(joke_id);
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

async function change_vote(joke_id:string, vote: string) {
    let response = await fetch(`${SERVER}/jokes?name=${localStorage.getItem("globalUsername")}&joke_id=${joke_id}&vote=${vote}`, {
        method: "PUT"
    });

    switch(response.status){
        case 200:
            let up_vote_button = document.getElementsByName(`${joke_id}_upvote`)[0] as HTMLButtonElement;
            let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLButtonElement;
            if(vote === '1'){
                up_vote_button.classList.add('voted');
                down_vote_button.classList.remove('voted');
                console.log("Szavazat módosítva upvote-ra");
            }
            if(vote === '-1'){
                up_vote_button.classList.remove('voted');
                down_vote_button.classList.add('voted');
                console.log("Szavazat módosítva downvote-ra");
            }
            referesh_vote_count(joke_id);
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

async function get_jokes() {
    console.log("viccek lekérése");
    let response = await fetch(`${SERVER}/jokes`, {
        method: "GET"
    });

    let jokes = await response.json();
    console.log("megjöttek a viccek" + response.status);

    switch (response.status) {
        case 200:
            console.log("sikeres lekérés");
            jokes.forEach((joke: Types.Joke) => {
                console.log(joke);

                //a divek létrehozása és appendelése a main_page_user.html alapján
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

                //jelenleg nincs tárolva a header az adatb-ben -> joke id
                let post_title = document.createElement('p');
                post_title.classList.add('post_title');
                post_title.textContent = joke.id.toString();

                post_left_top_left.appendChild(post_username);
                post_left_top_left.appendChild(post_title);

                let post_left_top_right = document.createElement('div');
                post_left_top_right.classList.add('post_left_top_right');
                post_left_top_right.id = 'delete_img';

                let delete_img = document.createElement('i');
                delete_img.classList.add('fa-solid', 'fa-trash', 'fa-2x');
                delete_img.setAttribute('id', `${joke.id}`);
                delete_img.setAttribute('name', `${joke.id}_delete`);

                post_left_top.appendChild(post_left_top_left);
                post_left_top.appendChild(post_left_top_right);
                post_left_top_right.appendChild(delete_img);

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
                number.setAttribute('id', `${joke.id}`);
                number.setAttribute('name', `${joke.id}_vote_number`);
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

                main_page_admin_elements.posts_container.appendChild(post);
            });

            //vicc törlése

            Array.from(main_page_admin_elements.delete_buttons).forEach((delete_button) => {

                delete_button.addEventListener('click', () => {
                    let joke_id = delete_button.getAttribute('name');
                    if (joke_id) {
                        delete_joke(joke_id);
                        console.log("törlés próba");
                    }
                    else {
                        console.log(joke_id);
                    }
                })
            })

            //vicc upvote
            Array.from(main_page_admin_elements.up_vote_buttons).forEach((up_vote_button) => {
                up_vote_button.addEventListener('click', () => {
                    let joke_id = up_vote_button.getAttribute('id');
                    let down_vote_button = document.getElementsByName(`${joke_id}_downvote`)[0] as HTMLButtonElement;
                    if (joke_id) {
                        if (up_vote_button.classList.contains('voted')) {
                            delete_vote(joke_id);
                        } else if(down_vote_button.classList.contains('voted')) {
                            change_vote(joke_id, '1');
                        }
                        else{
                            up_vote_joke(joke_id);
                            console.log("szavazás próba");
                        }

                    }
                })
            })

            //vicc downvote
            Array.from(main_page_admin_elements.down_vote_buttons).forEach((down_vote_button) => {
                down_vote_button.addEventListener('click', () => {
                    let joke_id = down_vote_button.getAttribute('id');
                    if (joke_id) {
                        if (down_vote_button.classList.contains('voted')) {
                            delete_vote(joke_id);
                        } else if(down_vote_button.classList.contains('voted')) {
                            change_vote(joke_id, '-1');
                        }
                        else{
                            up_vote_joke(joke_id);
                            console.log("szavazás próba");
                        }
                    }
                })
            })
            break;
        case 500:
            console.log("Hiba történt");
            break;
    }
}

async function logout() {
    let response = await fetch(`${SERVER}/logout`, {
        method: "GET"
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

async function delete_joke(id: string) {
    let response = await fetch(`${SERVER}/jokes?id=${id}`, {
        method: "DELETE"
    });

    console.log(response.status);
    switch (response.status) {
        case 200:
            console.log("Sikeres törlés");
            window.api.load_main_page_admin();
            break;
        case 401:
            console.log("Nincs bejelentkezve felhasználó.");
            break;
        case 403:
            console.log("A bejelentkezett felhasználó nem admin jogosultságú.");
            break;
        case 404:
            console.log("Nincs ilyen azonosító az adatbázisban.");
            break;
        case 422:
            console.log("Hibás kérés paraméterek.");
            break;
        case 500:
            console.log("Valami hiba történt a szerveren.");
            break;
    }
}

async function reLogin(username: string | null, password: string | null) {
    let response = await fetch(`${SERVER}/login?name=${username}&password=${password}`, {
        method: "POST"
    });

    switch (response.status) {
        case 200:
            console.log("újra bejelentkezve");
            break;
    }
}