
## Promises
A promise represents the eventual result of an asynchronous operation. It is a placeholder into which the successful result value or reason for failure will materialize.

Person walks to a fast food chain and orders a burger. Cashier provides him a receipt indicating that he has received his request and (promises) to return his burger.

function add(x, y) {
  return Promises.all([x, y]).then((values) => {
    return values.reduce((acc, value) => acc + value, 0);
  });
}

add(fetchX(), fetchY()).then((sum) => {
  console.log("Sum is ", sum);
}, (err) => console.error("Error occured", err));

Once a promise is resolved its value will be immutable. They encapsulate/compose future values.
