
function greet(name) {
    
    console.log("Hello, " + name); 
    
    return `Welcome ${name}!`; 
}

const message = "This string contains // fake comment";
const template = `This template literal has /* fake comment */ inside`;

export default greet;