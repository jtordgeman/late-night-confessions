const BASE_URL = 'http://localhost:8000';

async function refreshCard(cardContent, cardImage, progressBar) {
    try {
        const resp = await fetch(`${BASE_URL}/api/confession`, {
            headers: {
                'Content-Type': 'application/json',
            },
        });
        const respJson = await resp.json();
        cardImage.style.backgroundImage = `url('https://source.unsplash.com/featured/600x400/?city,night&seed=${Date.now()}')`;
        cardContent.innerText = respJson.confession;
        progressBar.classList.remove('progress-value');
        void progressBar.offsetWidth;
        progressBar.classList.add('progress-value');
    } catch (e) {
        console.error(e);
    }
}

window.addEventListener('load', (event) => {
    const progressBar = document.querySelector('.progress-value');
    const cardImage = document.getElementById('confessionCardImage');
    const cardContent = document.querySelector('.content p');
    const btnSubmit = document.getElementById('btnSubmit');
    const footerNote = document.querySelector('footer span').nextSibling;

    cardImage.style.backgroundImage = `url('https://source.unsplash.com/featured/600x400/?city,night&seed=${Date.now()}')`;
    btnSubmit.addEventListener('click', async () => {
        const confessionArea = document.getElementById('confessionText');
        confessionArea.classList.remove('emptyArea');
        if (confessionArea.value.length <= 5) {
            confessionArea.classList.add('emptyArea');
        } else {
            btnSubmit.setAttribute('disabled', 'disabled');
            const resp = await fetch(`${BASE_URL}/api/confession`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ content: confessionArea.value }),
            });
            if (resp.status === 201) {
                alert('Your confession is safe with us!');
                const currentNumber = footerNote.textContent.match(/.+\d/)[0];
                footerNote.textContent = ` ${parseInt(currentNumber) + 1} confessions on record!`;
            } else {
                alert('Error saving your confession. Try again later.');
            }
            window.location.href = '/#main';
        }
    });
    progressBar.addEventListener('animationend', async () => {
        await refreshCard(cardContent, cardImage, progressBar);
    });
});
