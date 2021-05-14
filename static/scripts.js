async function refreshCard(cardContent, progressBar) {
    try {
        const resp = await fetch(`/api/bekenntnis`, {
            headers: {
                'Content-Type': 'application/json',
            },
        });
        const respJson = await resp.json();
        cardContent.innerText = respJson.content;
        progressBar.classList.remove('progress-value');
        void progressBar.offsetWidth;
        progressBar.classList.add('progress-value');
    } catch (e) {
        console.error(e);
    }
}

window.addEventListener('load', (event) => {
    const progressBar = document.querySelector('.progress-value');
    const cardContent = document.querySelector('.content p');
    const btnSubmit = document.getElementById('btnSubmit');
    const footerNote = document.querySelector('footer span').nextSibling;

    btnSubmit.addEventListener('click', async () => {
        const bekenntnisText = document.getElementById('bekenntnisText');
        bekenntnisText.classList.remove('emptyArea');
        if (bekenntnisText.value.length <= 1) {
            bekenntnisText.classList.add('emptyArea');
        } else {
            btnSubmit.setAttribute('disabled', 'disabled');
            const resp = await fetch(`/api/bekenntnis`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ content: bekenntnisText.value }),
            });
            if (resp.status === 201) {
                alert('Dein Bekenntnis bleibt unser Geheimnis!');
                const currentNumber = footerNote.textContent.match(/.+\d/)[0];
                footerNote.textContent = ` ${parseInt(currentNumber) + 1} gespeicherte Bekenntnisse!`;
                bekenntnisText.value = '';
            } else {
                alert('Fehler beim Speichern deines Bekenntnisses. Versuch es später nochmal!');
            }
            window.location.href = '/#main';
        }
    });
    progressBar.addEventListener('animationend', async () => {
        await refreshCard(cardContent, progressBar);
    });
});