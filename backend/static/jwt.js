let jwt_token = ""

function get_jwt() {
   return jwt_token; 
}

async function fetch_jwt(){ 
   let res = await fetch("/get_jwt")
   let json = await res.json() 
   jwt_token = json.jwt  
}

fetch_jwt(); 
