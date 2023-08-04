// TODO: make a proper tooltip component, our previous one was a bit jankey and ugly, falling back to the browser's default for now
export const toolTip = (node: any, tip: string) => {
	node.title = tip;
};

export default toolTip;
