import re
import matplotlib.pyplot as plt
import math
import pandas as pd

# 存儲多個檔案的數據的列表
all_latencies = []

all_avg = []
all_var = []
all_loss = []

prio = 3

# 迭代每個檔案
for file_name in ['0.txt', '1.txt', '2.txt', '3.txt']:
    # 存儲當前檔案的數據的列表
    latencies = []

    # 開啟當前檔案並讀取內容
    with open(file_name, 'r') as file:
        data = file.read()

    # 使用正規表達式找到所有的Latency數據
    latency_matches = re.findall(r'Latency: (\d+\.\d+) ms', data)

    # 將字串轉換為浮點數並添加到當前檔案的數據列表中
    latencies = [float(latency) for latency in latency_matches]
    
    sum = 0.0
    for l in latencies:
        sum += l
    avg = sum / len(latencies)
    all_avg.append(avg)
    sum = 0
    for l in latencies:
        sum += pow(l - avg, 2)
    var = pow(sum / len(latencies), 0.5)
    all_var.append(var)
    cnt = 0
    for l in latencies:
        if (abs(l - avg) > var * 3):
            cnt+=1
    loss = cnt / len(latencies)
    all_loss.append(loss)
    
    # 將當前檔案的數據列表添加到所有檔案數據的列表中
    all_latencies.append(latencies)

# 將列表轉換為DataFrame
df = pd.DataFrame({'Average(ms)': all_avg, 'Std. Deviation(ms)': all_var, 'Data Loss(%)': all_loss})

# 格式化DataFrame
df = df.applymap('{:.2f}'.format)

# 印出DataFrame
print(df)
aligned_str = df.to_string(index=False, justify='center')
with open('table.txt', 'w') as file:
    file.write(aligned_str + "\n")
    

# 繪製折線圖
for i, latencies in enumerate(all_latencies, start=0):
    plt.plot(latencies, label=f'Priority {i}')

plt.title('Latency over Wi-Fi')	
plt.xlabel('Measurement Index')
plt.ylabel('Latency (ms)')

# 設定y軸範圍
plt.ylim(0, 800)

plt.legend()  # 添加圖例

# 保存圖片
for i in ['png', 'svg']:
    plt.savefig(f'conbined_latency_plot.{i}')

# 顯示圖片
plt.show()

