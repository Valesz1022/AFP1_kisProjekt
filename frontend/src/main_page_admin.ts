let main_page_admin_elements: {
    posts_container: HTMLDivElement
    delete_button: HTMLButtonElement
    up_vote_button: HTMLButtonElement
    down_vote_button: HTMLButtonElement
    new_post_button: HTMLButtonElement
    logout_button: HTMLButtonElement
}

window.addEventListener('load', () => {
    main_page_admin_elements =  {
        posts_container: document.getElementById('posts_container') as HTMLDivElement,
        delete_button: document.getElementById('delete_buuton') as HTMLButtonElement,
        up_vote_button: document.getElementById('up_vote_button') as HTMLButtonElement,
        down_vote_button: document.getElementById('down_vote_button') as HTMLButtonElement,
        new_post_button: document.getElementById('new_post_button') as HTMLButtonElement,
        logout_button: document.getElementById('logout_button') as HTMLButtonElement
    }

    get_jokes();
})

async function get_jokes() {
    let response = await fetch(`${SERVER}`, {
        method: "GET"
    });

    let jokes = await response.json();

    jokes.json.forEach((joke: Types.Joke) => {
        //a divek létrehozása és appendelése a main_page_user.html alapján
        let post = document.createElement('div');
        post.classList.add('post');

        let post_left = document.createElement('div');
        post.classList.add('post_left');

        let post_left_top = document.createElement('div');
        post_left_top.classList.add('post_left_top');

        let post_left_top_left = document.createElement('div');
        post_left_top_left.classList.add('post_left_top_left');

        let post_username = document.createElement('p');
        post_username.classList.add('post_username');
        post_username.textContent = joke.user_name;

        //jelenleg nincs tárolva a header az adatb-ben
        let post_title = document.createElement('p');
        post_title.classList.add('post_title');
        post_title.textContent = 'Joke Header';

        post_left_top_left.appendChild(post_username);
        post_left_top_left.appendChild(post_title);

        let post_left_top_right = document.createElement('div');
        post_left_top_right.classList.add('post_left_top_right');
        post_left_top_right.id = 'delete_img';

        let delete_img = document.createElement('i');
        delete_img.classList.add('fa-solid fa-trash', 'fa-2x');

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
}