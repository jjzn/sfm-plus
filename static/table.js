import { h } from 'https://esm.sh/preact';
import { useState, useEffect } from 'https://esm.sh/preact/hooks';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);

const get_date = () => {
	const now = new Date();

	return new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'short' }).format(now);
};

function Clock({updated, setStale}) {
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

	const hours = Math.floor(updated / 60).toString().padStart(2, 0)
	const mins = (updated % 60).toString().padStart(2, 0);

	return html`<caption class="clock">${clock} (actualitzat ${hours}:${mins})</caption>`;
}

function Table({station}) {
	const [data, setData] = useState({ updated: 0, table: [] });
	const [status, setStatus] = useState('ok');
	const [stale, setStale] = useState(false);

	useEffect(async () => {
		setStatus('carregant...');

		const res = await fetch(`http://127.0.0.1:8420/${station}`);
		if (!res.ok) {
			setStatus('(sense dades)');
			return;
		}

		const json = await res.json();
		setData(json);
		setStatus(json.table.length ? 'ok' : '(cap tren)');
	}, [station, stale]);

	useEffect(() => {}, [stale]);

	const rows = data.table.map(({title, time, track}) => html`
		<tr>
			<td>${title}</td>
			<td>${time}</td>
			<td>${track}</td>
		</tr>`);

	rows.unshift(html`
		<tr>
			<th>Destinació</th>
			<th>Hora</th>
			<th>Via</th>
		</tr>
	`);

	const fallback = html`<tr>
		<th>Destinació</th>
			<th>Hora</th>
			<th>Via</th>
		</tr>
		<tr>
			<td>${status}</td>
		</tr>
	`;

	return html`
		<table>
			<${Clock} updated=${data.updated} setStale=${val => setStale(val)} />
			${status == 'ok' ? rows : fallback}
		</table>`;
}

export default Table;