import numpy as np
from tqdm import tqdm
import pandas as pd
import time
import multiprocessing
import subprocess
import json
import datetime


def run(i):
    output_str = subprocess.run(f'powershell cat in/{i:04}.txt | .\\target\\debug\\ahc037.exe > out/{i:04}.txt', shell=True, capture_output=True, text=True).stderr
    # print('output_str:', output_str.split('\n'))
    result = json.loads(output_str.split('\n')[-2])
    return result


def main(i):
    start = time.time()
    # print(i, 'start')
    r = run(i)
    t = round(time.time()-start, 4)
    N = r['N']
    cost = r['cost']
    score = r['score']
    opt_cost = r['opt_cost']
    opt_score = r['opt_score']
    data = [i, N, cost, score, opt_cost, opt_score, t]
    print('\r', 'end', i, end='')
    # print(i, 'end')
    return data


if __name__ == '__main__':
    start = time.time()
    print("start: ", datetime.datetime.fromtimestamp(start))
    trial = 150
    '''
    result = []
    for i in tqdm(range(trial)):
        data = main(i)
        result.append(data)
    '''
    processes = multiprocessing.cpu_count()
    with multiprocessing.Pool(processes=processes) as pool:
        data = [pool.apply_async(main, (i,)) for i in range(trial)]
        result = [d.get() for d in data]
    print()
    # '''
    df = pd.DataFrame(result, columns=['i', 'N', 'cost', 'score', 'opt_cost', 'opt_score', 'time'])
    score = np.mean(df['score'])
    sum_score = np.sum(df['score'])
    print(f"score: {format(int(sum_score), ',')}, score mean: {format(int(score), ',')}")
    df.to_csv('result.csv', index=False)
    print(f'end elapsed time: {time.time()-start:.2f}s')
