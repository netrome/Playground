import torch
import torch.nn as nn

# Goal: Create a model that can compute the sum of an arbitrary sequence

def generate_sequence():
    seq = torch.randint(2, [20])
    label = sum(seq)

    return seq, label


class Model(nn.Module):

    def __init__(self):
        super().__init__()

        self.W = nn.Parameter(torch.randn(4, 4))

    def forward(self, sequence):
        registers = torch.zeros(4)

        for i in sequence:
            registers[0] = i # Input register
            registers = registers @ self.W

        return registers[1]


model = Model()
opt = torch.optim.Adam(model.parameters())

for i in range(10000):
    seq, label = generate_sequence()
    out = model(seq)

    loss = torch.abs(label - out)
    opt.zero_grad()
    loss.backward()
    model.W.grad *= torch.randint(2, model.W.shape).float()
    opt.step()
    print(loss)

import IPython
IPython.embed()

