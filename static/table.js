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
	const [stale, setStale] = useState(false);

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
	}, [station, stale]);

	const [clock, setClock] = useState(get_date());
	useEffect(() => {
		const EXTRA_SECS = 5;
		const secs = (60 - new Date().getSeconds()) % 60 + EXTRA_SECS;
		let timer;

		setTimeout(() => {
			setClock(get_date());
			setStale(true);

			timer = setInterval(() => {
				setClock(get_date());
				setStale(true);
			}, 60 * 1000);
		}, secs * 1000);

		return () => clearInterval(timer);
	}, []);

	const rows = data.map(({title, time, track}) => html`
		<tr>
			<td>${title}</td>
			<td>${time}</td>
			<td>${track}</td>
		</tr>`);

	return html`
		<table>
			<caption class="clock">${get_date()}</caption>
			${status == 'ok' ? rows : html`<tr><td>${status}</td></tr>`}
		</table>`;
}

export default Table;