reactorDef = {
  Structure = "Person",
  Mutating = true,
  Posting = false,
};


function update(data)
  print(data.name);
  data.someArray.insert(data.someArray[-1] + 1)
  return data;
end
