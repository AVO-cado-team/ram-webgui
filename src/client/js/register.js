

function fillUpRegisters()
{
    let registerContainer = document.getElementsByClassName("registers-container")[0];
    
    let acc = createRegisterElement(0);
    acc.classList.add("acc");

    registerContainer.appendChild(acc);

    for(let i = 1; i < 1000; i++)
        registerContainer.appendChild(createRegisterElement(i));
}

function updateRegister(registerNum, value)
{
    let register = document.getElementById("register-" + registerNum);
    register.innerHTML = value;
}

function addRegister(registerNum, value)
{
    let registerContainer = document.getElementsByClassName("registers-container")[0];
    registerContainer.appendChild(createRegisterElement(registerNum));
    updateRegister(registerNum, value);
}

function createRegisterElement(num)
{
    let register = document.createElement("div");
    register.classList.add("register");

    let registerNum = document.createElement("div");
    registerNum.classList.add("register-num");
    
    let registerNumP = document.createElement("p");
    registerNumP.innerHTML = num;
    registerNum.appendChild(registerNumP);

    let registerVal = document.createElement("div");
    registerVal.classList.add("register-val");
    registerVal.id = "register-" + num;
    registerVal.innerHTML = "0";

    register.appendChild(registerNum);
    register.appendChild(registerVal);

    return register;
}

fillUpRegisters();