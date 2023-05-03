import { h } from 'https://esm.sh/preact';
import { useState, useEffect } from 'https://esm.sh/preact/hooks';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);

const get_date = () => {
	const now = new Date();

	return new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'short' }).format(now);
};

function Table({station}) {
	const [data, setData] = useState([]);
	const [status, setStatus] = useState('ok');

	useEffect(async () => {
		setStatus('carregant...');

		const res = await fetch(`http://127.0.0.1:8420/${station}`);
		if (!res.ok) {
			setStatus('(sense dades)');
			return;
		}

		const {table} = await res.json();

		setData(table);
		setStatus(table.length ? 'ok' : '(cap tren)');
	}, [station]);

	const rows = data.map(({title, time, track}) => html`
		<tr>
			<td>${title}</td>
			<td>${time}</td>
			<td>${track}</td>
		</tr>`);

	const [clock, setClock] = useState(get_date());
	useEffect(() => {
		const timer = setInterval(() => setClock(get_date()), 1000);
		return () => clearInterval(timer);
	}, []);

	return html`
		<table>
			<caption class="clock">${get_date()}</caption>
			${status == 'ok' ? rows : html`<tr><td>${status}</td></tr>`}
		</table>`;
}

export default Table;