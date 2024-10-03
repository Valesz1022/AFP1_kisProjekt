let main_page_admin_elements: {
    posts_container: HTMLDivElement
    delete_buttons: HTMLCollection
    up_vote_buttons: HTMLCollection
    down_vote_buttons: HTMLCollection
    new_post_button: HTMLButtonElement
    logout_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    main_page_admin_elements =  {
        posts_container: document.getElementById('posts_container') as HTMLDivElement,
        delete_buttons: document.getElementsByClassName('fa-trash') as HTMLCollection,
        up_vote_buttons: document.getElementsByClassName('fa-arrow-up') as HTMLCollection,
        down_vote_buttons: document.getElementsByClassName('fa-arrow-down') as HTMLCollection,
        new_post_button: document.getElementById('new_post_button') as HTMLButtonElement,
        logout_button:document.getElementById('logout_button') as HTMLButtonElement
    }

    //viccek betöltése
    get_jokes();

    //kijelentkezés funkció
    main_page_admin_elements.logout_button.addEventListener('click', () => {
        logout();
    })

    //Új vicc oldal
    main_page_admin_elements.new_post_button.addEventListener('click', () =>{
        window.api.load_new_post();
    })

    //vicc törlése
    Array.from(main_page_admin_elements.delete_buttons).forEach((delete_button) => {
        let joke_id = delete_button.getAttribute('name');
        delete_button.addEventListener('click', () => {
            if(joke_id){
                delete_joke(joke_id);
            }
        })
    })

    //vicc upvote
    Array.from(main_page_admin_elements.up_vote_buttons).forEach((up_vote_button) => {
        up_vote_button.addEventListener('click', () => {
            up_vote_joke()
        })
    })

    //vicc downvote


})

async function up_vote_joke() {
    
}

async function get_jokes() {
    console.log("viccek lekérése");
    let response = await fetch(`${SERVER}/jokes`, {
        method: "GET"
    });

    let jokes = await response.json();
    console.log("megjöttek a viccek" + response.status);

    switch(response.status){
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
                delete_img.setAttribute('name', `${joke.id}`);
        
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
                up_vote.appendChild(up_vote_icon);
        
                let vote_number = document.createElement('div');
                let number = document.createElement('p');
                number.textContent = joke.votes.toString();
                vote_number.appendChild(number);
        
        
                let down_vote = document.createElement('div');
                down_vote.classList.add('down_vote');
                let down_vote_icon = document.createElement('i');
                down_vote_icon.classList.add('fa-solid', 'fa-arrow-down', 'fa-2x');
                down_vote.appendChild(down_vote_icon);
        
                post_right.appendChild(up_vote);
                post_right.appendChild(vote_number);
                post_right.appendChild(down_vote);
        
                post.appendChild(post_left);
                post.appendChild(post_right);
        
                main_page_admin_elements.posts_container.appendChild(post);
            });
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

    switch(response.status){
        case 200:
            window.api.load_login_page();
        case 500:
            console.log("Hiba történt");
    }
}

async function delete_joke(id: string) {
    let response = await fetch(`${SERVER}/jokes?id=${id}`, {
        method: "DELETE"
    });


}