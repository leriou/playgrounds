import pandas as pd
import os
import fasttext
import random

amazon_source = "../../datasets/amazon.csv"

def trans_file(old_file):
    x = old_file + ".train"
    y = old_file + ".valid"
    h = pd.read_csv(old_file)

    with open(x, "w+") as fx:
        with open(y, "w+") as fy:
            for i in range(0, len(h)):
                s = "__label__" + str(h['Positive'][i])+" " + h['reviewText'][i] + "\n"
                if i > 12000:
                    fy.write(s)
                else:
                    fx.write(s)
    return x,y

amazon_train, amazon_test = trans_file(amazon_source)

model = fasttext.train_supervised(input=amazon_train,epoch=5,lr=0.8,wordNgrams=2,dim=128,loss='hs')
model.test(amazon_test,k=-1)

model.predict("got this kindle fire for Christmas. trying to download free angry birds and it is not working. started to down load over 2 hours ago. an it is only at 2 percent still. piece of crap:(",k=-1)