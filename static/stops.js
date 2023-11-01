import sfm from './stops/sfm.js';
import emt from './stops/emt.js';

const stops = [...Object.entries(sfm), ...Object.entries(emt)];

export {
	sfm, emt,
	stops as default
};
