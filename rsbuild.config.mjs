const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

export default {
  source: {
    	  entry: {
	  	 hello: './index.js'
	  }
  },
  tools: {
  	 rspack: {
	 	 plugins: [
		 	 new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),

		 		 ]
	 }
  }
};
