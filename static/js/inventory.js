import {Grocery} from "./grocery.js"

let request = await fetch("/get_groceries", {
    method: "GET",
    headers: {
        "Content-Type": "application/json",
    },
});

let inventory =
    JSON.parse(await request.json())
        .map(element => {
            return new Grocery(element.typ, element.amount, element.unit)
        });

inventory.forEach(element => {
    const li = document.createElement("li");
    const text = document.createTextNode(`${element.type}: ${element.amount}${element.unit.toLowerCase()}`);

    li.appendChild(text);
    document.getElementById("groceries").appendChild(li);
});